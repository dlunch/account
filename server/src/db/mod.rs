pub mod models;
pub mod schema;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

use super::config::Config;

pub fn create_pool(config: &Config) -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    Pool::builder().build(manager).unwrap()
}
