use crate::config::Config;

use deadpool_redis::Pool;
use redis::RedisError;

pub fn create_redis_pool(config: &Config) -> Result<Pool, RedisError> {
    let cfg = deadpool_redis::Config {
        url: Some(config.redis_url.to_owned()),
        ..Default::default()
    };

    cfg.create_pool()
}
