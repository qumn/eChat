use std::option;

use axum::extract::{FromRequest, RequestParts};
use axum::headers::{authorization::Bearer, Authorization};
use axum::{async_trait, TypedHeader};
use chrono::{Duration, Local};
use eChat::err::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tracing::debug;

const SECRET: &'static str = "secret";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthUser {
    pub exp: usize,
    pub uid: u64,
    pub username: String,
    pub mail: String,
}

impl AuthUser {
    pub fn new(uid: u64, username: String, mail: String) -> Self {
        // get now sec
        let exp = (Local::now() + Duration::days(3)).timestamp() as usize;
        AuthUser {
            exp,
            uid,
            username,
            mail,
        }
    }
    pub fn encode(&self) -> String {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(SECRET.as_ref()),
        )
        .unwrap()
    }

    fn decode(token: &str) -> Result<Self, Error> {
        debug!(token = token, "parse token");
        decode::<AuthUser>(
            token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| {
            debug!(error = ?e, "while parse token");
            Error::Unauthorized
        })
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthUser
where
    B: Send,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let authorization = TypedHeader::<Authorization<Bearer>>::from_request(req)
            .await
            .map_err(|_| Error::Unauthorized);
        if authorization.is_ok() {
            debug!("get token from header");
            return AuthUser::decode(authorization.unwrap().token());
        }

        // header have not token, try get token from uri
        if let Some(query) = req.uri().query() {
            let access_token = get_param(query, "access_token");
            if let Some(access_token) = access_token {
                debug!("get token from param");
                return AuthUser::decode(access_token);
            }
        }
        return Err(Error::Unauthorized);
    }
}

fn get_param<'a>(query: &'a str, param: &str) -> Option<&'a str> {
    query
        .split('&')
        .find(|s| s.starts_with(param))
        .map(|s| s.split('=').nth(1).unwrap())
}

#[cfg(test)]
mod test {
    use std::time::SystemTime;

    use chrono::Local;

    #[test]
    fn test() {
        let seconds1 = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let seconds2 = Local::now().timestamp();
        println!("{} {}", seconds1, seconds2);
    }
}
