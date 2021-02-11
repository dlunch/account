use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub nbf: usize,
}

const JWT_SECRET: &str = "SecretValue!2#4%6&7"; // TODO env var

pub fn encode(claims: Claims) -> String {
    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET.as_ref())).unwrap()
}

pub fn decode(token: String) -> Claims {
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
}
