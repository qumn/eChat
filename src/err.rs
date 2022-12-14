use axum::http::header::WWW_AUTHENTICATE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use sqlx::error::DatabaseError;
use sqlx::mysql::MySqlDatabaseError;
use std::borrow::Cow;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Error>;


#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("request path not found")]
    NotFound,

    #[error("error in the request body")]
    UnprocessableEntity {
        errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
    },

    #[error("error occurred with the database")]
    Sqlx(#[from] sqlx::Error),

    #[error("{0}")]
    Duplicated(String),

    #[error("error occurred server internal")]
    Axum(#[from] axum::Error),

    #[error("error occurred while parse json")]
    Serde(#[from] serde_json::Error),

    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
}

impl Error {
    /// Convenient constructor for `Error::UnprocessableEntity`.
    ///
    /// Multiple for the same key are collected into a list for that key.
    ///
    /// Try "Go to Usage" in an IDE for examples.
    pub fn unprocessable_entity<K, V>(errors: impl IntoIterator<Item = (K, V)>) -> Self
    where
        K: Into<Cow<'static, str>>,
        V: Into<Cow<'static, str>>,
    {
        let mut error_map = HashMap::new();

        for (key, val) in errors {
            error_map
                .entry(key.into())
                .or_insert_with(Vec::new)
                .push(val.into());
        }

        Self::UnprocessableEntity { errors: error_map }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Sqlx(_) | Self::Anyhow(_) | Self::Duplicated(_) | Self::Axum(_) | Self::Serde(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

/// Axum allows you to return `Result` from handler functions, but the error type
/// also must be some sort of response type.
///
/// By default, the generated `Display` impl is used to return a plaintext error message
/// to the client.
impl IntoResponse for Error {
    //type Body = Full<Bytes>;
    //type BodyError = <Full<Bytes> as HttpBody>::Error;
    fn into_response(self) -> Response {
        tracing::error!(error =?self, "meet a error");
        match self {
            Self::UnprocessableEntity { errors } => {
                #[derive(serde::Serialize)]
                struct Errors {
                    errors: HashMap<Cow<'static, str>, Vec<Cow<'static, str>>>,
                }

                return (StatusCode::UNPROCESSABLE_ENTITY, Json(Errors { errors })).into_response();
            }
            Self::Unauthorized => {
                return (
                    self.status_code(),
                    // Include the `WWW-Authenticate` challenge required in the specification
                    // for the `401 Unauthorized` response code:
                    // https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/401
                    //
                    // The Realworld spec does not specify this:
                    // https://realworld-docs.netlify.app/docs/specs/backend-specs/error-handling
                    //
                    // However, at Launchbadge we try to adhere to web standards wherever possible,
                    // if nothing else than to try to act as a vanguard of sanity on the web.
                    [(WWW_AUTHENTICATE, HeaderValue::from_static("Token"))]
                        .into_iter()
                        .collect::<HeaderMap>(),
                    self.to_string(),
                )
                    .into_response();
            }

            Self::Sqlx(ref e) => {
                tracing::error!("SQLx error: {:?}", e);
            }

            Self::Anyhow(ref e) => {
                tracing::error!("Generic error: {:?}", e);
            }
            Self::Duplicated(ref e) => {
                tracing::error!("Duplicate error: {:?}", e);
            }
            // Other errors get mapped normally.
            _ => (),
        }

        (self.status_code(), self.to_string()).into_response()
    }
}
pub trait ResultExt<T> {
    /// If `self` contains a SQLx database constraint error with the given name,
    /// transform the error.
    ///
    /// Otherwise, the result is passed through unchanged.
    fn on_constraint(
        self,
        name: &str,
        f: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T>;

    fn on_duplicated(self, f: String) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn on_constraint(
        self,
        name: &str,
        map_err: impl FnOnce(Box<dyn DatabaseError>) -> Error,
    ) -> Result<T> {
        self.map_err(|e| match e.into() {
            Error::Sqlx(sqlx::Error::Database(dbe)) if dbe.constraint() == Some(name) => {
                map_err(dbe)
            }
            e => e,
        })
    }

    fn on_duplicated(self, msg: String) -> Result<T> {
        self.map_err(|e| -> Error {
            match e.into() {
                Error::Sqlx(sqlx::Error::Database(dbe)) => {
                    match dbe.try_downcast_ref::<MySqlDatabaseError>() {
                        // 1062 is the error code of mysql duplicate entry
                        Some(mysql_dbe) if mysql_dbe.number() == 1062 => Error::Duplicated(msg),
                        _ => Error::Sqlx(sqlx::Error::Database(dbe)),
                    }
                }
                e => e,
            }
        })
    }
}
