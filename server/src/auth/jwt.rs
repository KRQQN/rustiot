use std::{env, sync::OnceLock};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i64,
    pub email: String,
    pub jti: String,
    pub exp: usize,
}

static ENCODING_KEY: OnceLock<EncodingKey> = OnceLock::new();
static DECODING_KEY: OnceLock<DecodingKey> = OnceLock::new();

fn encoding_key() -> &'static EncodingKey {
    ENCODING_KEY.get_or_init(|| {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET environment variable is missing");

        EncodingKey::from_secret(secret.as_bytes())
    })
}

fn decoding_key() -> &'static DecodingKey {
    DECODING_KEY.get_or_init(|| {
        let secret = env::var("JWT_SECRET").expect("JWT_SECRET environment variable is missing");

        DecodingKey::from_secret(secret.as_bytes())
    })
}

pub fn create(user_id: i64, email: impl Into<String>) -> jsonwebtoken::errors::Result<String> {
    let claims = Claims {
        sub: user_id,
        email: email.into(),
        jti: Uuid::new_v4().to_string(),
        exp: (Utc::now() + Duration::days(7)).timestamp() as usize,
    };

    encode(&Header::default(), &claims, encoding_key())
}

pub fn verify(token: &str) -> jsonwebtoken::errors::Result<Claims> {
    Ok(decode::<Claims>(token, decoding_key(), &Validation::default())?.claims)
}
