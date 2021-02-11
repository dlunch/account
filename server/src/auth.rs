use async_trait::async_trait;
use tonic::{Request, Response, Status};

mod pb {
    tonic::include_proto!("auth");
}

use pb::{LoginRequest, LoginResponse};

pub struct Auth {}

impl Auth {
    pub fn service() -> pb::auth_server::AuthServer<Self> {
        pb::auth_server::AuthServer::new(Self {})
    }
}

#[async_trait]
impl pb::auth_server::Auth for Auth {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let username = request.into_inner().username;
        println!("username: {}", username);

        Ok(Response::new(LoginResponse { token: username }))
    }
}
