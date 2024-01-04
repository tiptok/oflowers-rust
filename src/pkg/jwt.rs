use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::HttpRequest;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct UserToken {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data
    pub user_id: i64,
}

pub const AUTHORIZATION: &str = "Authorization";

impl UserToken {
    pub fn decode(
        token: String,
        secret: String,
    ) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
        jsonwebtoken::decode::<UserToken>(
            &token,
            &DecodingKey::from_secret(secret.as_bytes()),
            &Validation::default(),
        )
    }
    pub fn encode(&mut self, secret: String, expires: i64) -> String {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        self.iat = now;
        self.exp = now + expires;
        jsonwebtoken::encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap()
    }
    pub fn decode_from_request(
        req: HttpRequest,
        secret: String,
    ) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
        let mut token: &str = "";
        if let Some(val) = req.headers().get(AUTHORIZATION) {
            if let Ok(token_str) = val.to_str() {
                if token_str.starts_with("bearer") || token_str.starts_with("Bearer") {
                    token = token_str[6..token_str.len()].trim();
                }
            }
        }
        UserToken::decode(token.to_string(), secret)
    }
}
