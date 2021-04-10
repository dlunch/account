pub mod models;
pub mod schema;

use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};

use super::config::Config;

embed_migrations!();

pub fn create_db_pool(config: &Config) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(&config.database_url);
    let pool = Pool::builder().build(manager)?;

    embedded_migrations::run(&pool.get().unwrap()).unwrap();

    Ok(pool)
}
