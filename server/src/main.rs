#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod config;
mod db;
mod handlers;
mod redis;

use tonic::transport::Server;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let config = envy::from_env::<config::Config>()?;
    let db_pool = db::create_db_pool(&config)?;
    let redis_pool = redis::create_redis_pool(&config)?;

    let auth_service = handlers::Auth::new(db_pool.clone(), config.clone());
    let card_service = handlers::Card::new(db_pool.clone(), redis_pool.clone(), config.clone());

    let addr = config.listen_addr.parse()?;
    Server::builder().add_service(auth_service).add_service(card_service).serve(addr).await?;

    Ok(())
}
