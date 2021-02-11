use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub nbf: usize,
}

const JWT_SECRET: &str = "SecretValue!2#4%6&7"; // TODO env var

pub fn create(user_id: String) -> String {
    let now = SystemTime::now();
    let exp = now + Duration::from_secs(60 * 60);
    let claims = Claims {
        sub: user_id,
        exp: exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
        nbf: now.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref())).unwrap()
}

pub fn decode(token: String) -> String {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation {
            validate_nbf: true,
            ..Default::default()
        },
    )
    .unwrap()
    .claims
    .sub
}
