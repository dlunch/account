use std::env;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use argon2::{self, Config};
use async_trait::async_trait;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use tonic::{Code, Request, Response, Status};

use super::models;
use super::schema;
use super::token;

mod pb {
    tonic::include_proto!("auth");
}

use pb::{LoginRequest, LoginResponse};

pub struct Auth {
    pool: Pool<ConnectionManager<PgConnection>>,
    salt: Vec<u8>,
}

impl Auth {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> pb::auth_server::AuthServer<Self> {
        let salt = env::var("SALT").unwrap();
        pb::auth_server::AuthServer::new(Self {
            pool,
            salt: salt.into_bytes(),
        })
    }
}

#[async_trait]
impl pb::auth_server::Auth for Auth {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();

        let user = schema::users::dsl::users
            .filter(schema::users::dsl::username.eq(&request.username))
            .first::<models::Users>(&self.pool.get().unwrap())
            .map_err(|_| Status::new(Code::PermissionDenied, "Login Failure"))?;

        let hash = argon2::hash_encoded(request.password.as_bytes(), &self.salt, &Config::default()).unwrap();
        let matches = argon2::verify_encoded(&hash, &user.password).unwrap();

        if (!matches) {
            Err(Status::new(Code::PermissionDenied, "Login Failure"))
        } else {
            let now = SystemTime::now();
            let exp = now + Duration::from_secs(60 * 60);
            let token = token::encode(token::Claims {
                sub: user.id.to_string(),
                exp: exp.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
                nbf: now.duration_since(UNIX_EPOCH).unwrap().as_secs() as usize,
            });

            Ok(Response::new(LoginResponse { token }))
        }
    }
}
