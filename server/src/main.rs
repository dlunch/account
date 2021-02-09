use async_trait::async_trait;
use tonic::{transport::Server, Request, Response, Status};

mod pb {
    tonic::include_proto!("auth");
}

use pb::{LoginRequest, LoginResponse};

struct Auth {}

#[async_trait]
impl pb::auth_server::Auth for Auth {
    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<LoginResponse>, Status> {
        let username = request.into_inner().username;
        println!("username: {}", username);

        Ok(Response::new(LoginResponse { token: username }))
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let addr = "0.0.0.0:9090".parse().unwrap();
    let auth = Auth {};

    let service = pb::auth_server::AuthServer::new(auth);
    Server::builder().add_service(service).serve(addr).await?;

    Ok(())
}
