use std::str::{self, FromStr};

use tonic::{metadata::MetadataValue, Request, Status};
use uuid::Uuid;

use super::token;

pub fn check_auth(mut request: Request<()>, token_secret: &str) -> Result<Request<()>, Status> {
    if let Some(authorization) = request.metadata().get("authorization") {
        let authorization = str::from_utf8(authorization.as_bytes()).unwrap();

        let split = authorization.split(' ').collect::<Vec<_>>();
        let (bearer, token) = (split[0], split[1]);

        if bearer.to_lowercase() != "bearer" {
            Err(Status::unauthenticated("No valid auth token"))
        } else {
            let user_id = token::decode(token, token_secret);

            request.metadata_mut().remove("user_id");
            request.metadata_mut().append("user_id", MetadataValue::from_str(&user_id).unwrap());

            Ok(request)
        }
    } else {
        Err(Status::unauthenticated("No valid auth token"))
    }
}

pub fn get_user_id(request: Request<()>) -> Uuid {
    Uuid::from_str(request.metadata().get("user_id").unwrap().to_str().unwrap()).unwrap()
}
