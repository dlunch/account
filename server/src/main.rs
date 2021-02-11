#[macro_use]
extern crate diesel;

mod config;
mod db;
mod handlers;

use tonic::transport::Server;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let config = envy::from_env::<config::Config>()?;
    let pool = db::create_pool(&config);

    let auth_service = handlers::AuthHandler::new(pool.clone(), config.clone());
    let card_service = handlers::CardHandler::new(pool.clone(), config.clone());

    let addr = config.listen_addr.parse().unwrap();
    Server::builder().add_service(auth_service).add_service(card_service).serve(addr).await?;

    Ok(())
}
