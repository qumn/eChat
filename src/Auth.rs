use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::{FromRequest, RequestParts};
use axum::headers::{authorization::Bearer, Authorization};
use axum::{async_trait, TypedHeader};
use eChat::err::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET: &'static str = "secret";

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthUser {
    exp: usize,
    pub uid: u64,
    pub username: String,
    pub mail: String,
}

impl AuthUser {
    pub fn new(uid: u64, username: String, mail: String) -> Self {
        AuthUser {
            exp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs() as usize
                + 3 * 60 * 60,
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
    fn decode(token: &str) -> Self {
        decode::<AuthUser>(
            token,
            &DecodingKey::from_secret(SECRET.as_ref()),
            &Validation::default(),
        )
        .unwrap()
        .claims
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
            .map_err(|_| Error::Unauthorized)?;
        Ok(AuthUser::decode(authorization.token()))
    }
}
