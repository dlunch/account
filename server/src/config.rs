use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub listen_addr: String,
    pub database_url: String,
    pub password_salt: String,
    pub token_secret: String,
}
