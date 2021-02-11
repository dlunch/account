pub mod models;
pub mod schema;

use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

use super::config::Config;

pub fn create_pool(config: &Config) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    Pool::builder().build(manager)
}
