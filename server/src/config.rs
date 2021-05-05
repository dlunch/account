use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub database_url: String,
    pub redis_url: String,
    pub token_secret: String,
    pub credential_secret: String,
}
