use std::str::{self, FromStr};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use cookie::Cookie;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tonic::{metadata::MetadataValue, Request, Status};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub nbf: usize,
}

pub fn create_token(user_id: &str, secret: &str) -> String {
    let now = SystemTime::now();
    let exp = now + Duration::from_secs(60 * 60);
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
        nbf: now.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
    };

    jsonwebtoken::encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).unwrap()
}

fn decode_token(token: &str, secret: &str) -> String {
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

pub fn check_auth(mut request: Request<()>, token_secret: &str) -> Result<Request<()>, Status> {
    if let Some(cookie) = request.metadata().get("Cookie") {
        let cookie_str = str::from_utf8(cookie.as_bytes()).unwrap();
        let cookie = Cookie::parse(cookie_str).unwrap();

        let user_id = decode_token(cookie.value(), token_secret);

        request.metadata_mut().remove("user_id");
        request.metadata_mut().append("user_id", MetadataValue::from_str(&user_id).unwrap());

        Ok(request)
    } else {
        Err(Status::unauthenticated("No valid auth token"))
    }
}

pub fn get_user_id<T>(request: &Request<T>) -> Uuid {
    Uuid::from_str(request.metadata().get("user_id").unwrap().to_str().unwrap()).unwrap()
}
