mod auth;

use tonic::transport::Server;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let auth_service = auth::Auth::service();

    let addr = "0.0.0.0:9090".parse().unwrap();
    Server::builder().add_service(auth_service).serve(addr).await?;

    Ok(())
}
