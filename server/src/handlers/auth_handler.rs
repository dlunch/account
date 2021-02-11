use async_diesel::AsyncRunQueryDsl;
use async_trait::async_trait;
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl,
};
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use super::base;
use crate::config::Config;
use crate::db::models;
use crate::db::schema::users::dsl;

mod pb {
    tonic::include_proto!("auth");
}

use pb::{LoginRequest, LoginResponse, RegisterRequest};

pub struct AuthHandler {
    pool: Pool<ConnectionManager<PgConnection>>,
    config: Config,
}

impl AuthHandler {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>, config: Config) -> pb::auth_server::AuthServer<Self> {
        pb::auth_server::AuthServer::new(Self { pool, config })
    }

    fn hash_password(&self, password: &str) -> String {
        argon2::hash_encoded(password.as_bytes(), self.config.password_salt.as_bytes(), &argon2::Config::default()).unwrap()
    }
}

#[async_trait]
impl pb::auth_server::Auth for AuthHandler {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let request = request.into_inner();

        let user = dsl::users
            .filter(dsl::username.eq(request.username))
            .first_async::<models::User>(&self.pool)
            .await
            .map_err(|_| Status::new(Code::PermissionDenied, "Login Failure"))?;

        let password_hash = self.hash_password(&request.password);
        let matches = argon2::verify_encoded(&password_hash, &user.password).unwrap();

        if (!matches) {
            Err(Status::new(Code::PermissionDenied, "Login Failure"))
        } else {
            let token = base::create_token(&user.id.to_string(), &self.config.token_secret);

            Ok(Response::new(LoginResponse { token }))
        }
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        let password_hash = self.hash_password(&request.password);

        let user = models::User {
            id: Uuid::new_v4(),
            username: request.username,
            password: password_hash.into_bytes(),
        };

        insert_into(dsl::users).values(user).execute_async(&self.pool).await.unwrap();

        Ok(Response::new(()))
    }
}
