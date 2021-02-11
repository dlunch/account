use std::env;

use argon2::{self, Config};
use async_trait::async_trait;
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use super::models;
use super::schema;
use super::token;

mod pb {
    tonic::include_proto!("auth");
}

use pb::{LoginRequest, LoginResponse, RegisterRequest};

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

    fn hash_password(&self, password: &str) -> String {
        argon2::hash_encoded(password.as_bytes(), &self.salt, &Config::default()).unwrap()
    }
}

#[async_trait]
impl pb::auth_server::Auth for Auth {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        use schema::users::dsl::*;

        let request = request.into_inner();

        let user = users
            .filter(username.eq(&request.username))
            .first::<models::User>(&self.pool.get().unwrap())
            .map_err(|_| Status::new(Code::PermissionDenied, "Login Failure"))?;

        let password_hash = self.hash_password(&request.password);
        let matches = argon2::verify_encoded(&password_hash, &user.password).unwrap();

        if (!matches) {
            Err(Status::new(Code::PermissionDenied, "Login Failure"))
        } else {
            let token = token::create(user.id.to_string());

            Ok(Response::new(LoginResponse { token }))
        }
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<()>, Status> {
        use schema::users::dsl::*;

        let request = request.into_inner();

        let password_hash = self.hash_password(&request.password);

        let user = models::User {
            id: Uuid::new_v4(),
            username: request.username,
            password: password_hash.into_bytes(),
        };

        insert_into(users).values(&user).execute(&self.pool.get().unwrap()).unwrap();

        Ok(Response::new(()))
    }
}
