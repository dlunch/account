#[macro_use]
extern crate diesel;

mod auth;
mod models;
mod schema;
mod token;

use std::env;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use tonic::transport::Server;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let manager = ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").unwrap());
    let pool = Pool::builder().build(manager).unwrap();

    let auth_service = auth::Auth::new(pool.clone());

    let addr = "0.0.0.0:9090".parse().unwrap();
    Server::builder().add_service(auth_service).serve(addr).await?;

    Ok(())
}
