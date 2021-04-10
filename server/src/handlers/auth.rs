use async_diesel::AsyncRunQueryDsl;
use async_trait::async_trait;
use cookie::Cookie;
use diesel::{
    insert_into,
    r2d2::{ConnectionManager, Pool},
    ExpressionMethods, PgConnection, QueryDsl,
};
use rand::Rng;
use tonic::{Code, Request, Response, Status};
use uuid::Uuid;

use super::base;
use crate::config::Config;
use crate::db::models;
use crate::db::schema::users::dsl;

mod pb {
    tonic::include_proto!("auth");
}

use pb::{LoginRequest, RegisterRequest};

pub struct Auth {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    config: Config,
}

impl Auth {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>, config: Config) -> pb::auth_server::AuthServer<Self> {
        pb::auth_server::AuthServer::new(Self { db_pool, config })
    }

    fn hash_password(&self, password: &str) -> String {
        let salt = rand::thread_rng().gen::<[u8; 32]>();
        argon2::hash_encoded(password.as_bytes(), &salt, &argon2::Config::default()).unwrap()
    }

    fn verify_password(&self, password: &str, encoded_password: &str) -> bool {
        argon2::verify_encoded(encoded_password, password.as_bytes()).unwrap()
    }
}

#[async_trait]
impl pb::auth_server::Auth for Auth {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        let user = dsl::users
            .filter(dsl::username.eq(request.username))
            .first_async::<models::User>(&self.db_pool)
            .await
            .map_err(|_| Status::new(Code::PermissionDenied, "Login Failure"))?;

        let matches = self.verify_password(&request.password, &user.password);

        if (!matches) {
            Err(Status::new(Code::PermissionDenied, "Login Failure"))
        } else {
            let token = base::create_token(&user.id.to_string(), &self.config.token_secret);
            let cookie = Cookie::build("token", token).secure(true).http_only(true).finish();

            let mut response = Response::new(());
            response.metadata_mut().insert("Set-Cookie", cookie.to_string().parse().unwrap());

            Ok(response)
        }
    }

    async fn register(&self, request: Request<RegisterRequest>) -> Result<Response<()>, Status> {
        let request = request.into_inner();

        let password_hash = self.hash_password(&request.password);

        let count = dsl::users
            .select(diesel::dsl::count_star())
            .filter(dsl::username.eq(request.username.clone()))
            .first_async::<i64>(&self.db_pool)
            .await
            .unwrap();

        if count != 0 {
            Err(Status::already_exists("Username already taken"))
        } else {
            insert_into(dsl::users)
                .values((
                    dsl::id.eq(Uuid::new_v4()),
                    dsl::username.eq(request.username),
                    dsl::password.eq(password_hash),
                ))
                .execute_async(&self.db_pool)
                .await
                .unwrap();

            Ok(Response::new(()))
        }
    }
}
