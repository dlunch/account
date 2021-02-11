use std::time::{Duration, SystemTime, UNIX_EPOCH};

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub nbf: usize,
}

pub fn create(user_id: String, secret: &str) -> String {
    let now = SystemTime::now();
    let exp = now + Duration::from_secs(60 * 60);
    let claims = Claims {
        sub: user_id,
        exp: exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
        nbf: now.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

#[allow(dead_code)]
pub fn decode(token: String, secret: &str) -> String {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation {
            validate_nbf: true,
            ..Default::default()
        },
    )
    .unwrap()
    .claims
    .sub
}
