use crate::models::prelude::*;

use anyhow::{Context, Result};
use chrono::{Duration, Local};

use jsonwebtoken::{decode, encode};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};

pub static TOKEN_EXPIRY_DURATION: i64 = 60 * 60 * 24;

#[derive(Debug, Deserialize, Serialize)]
pub struct UserToken {
    pub sub: String,
    pub exp: usize,
}

impl UserToken {
    pub fn with_user_id(user_id: impl AsRef<str>) -> Self {
        Self {
            sub: user_id.as_ref().to_string(),
            exp: (Local::now() + Duration::seconds(TOKEN_EXPIRY_DURATION)).timestamp() as usize,
        }
    }

    pub fn generate_token(user_id: impl AsRef<str>, key: &[u8]) -> Result<String> {
        let info = UserToken::with_user_id(user_id);
        let key = EncodingKey::from_secret(key);
        Ok(encode(&Header::default(), &info, &key).with_context(|| "failed to generate token")?)
    }
}

impl UserToken {
    pub fn decode_token(token: impl AsRef<str>, key: &[u8]) -> Result<TokenData<UserToken>> {
        Ok(decode::<Self>(
            token.as_ref(),
            &DecodingKey::from_secret(key),
            &Validation::default(),
        )?)
    }
}
