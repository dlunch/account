#[macro_use]
extern crate diesel;

mod auth;
mod config;
mod models;
mod schema;
mod token;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use tonic::transport::Server;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let config = envy::from_env::<config::Config>()?;

    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = Pool::builder().build(manager).unwrap();

    let auth_service = auth::Auth::new(pool.clone(), config.clone());

    let addr = config.listen_addr.parse().unwrap();
    Server::builder().add_service(auth_service).serve(addr).await?;

    Ok(())
}
