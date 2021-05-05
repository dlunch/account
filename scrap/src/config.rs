use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub start_webdriver: bool,
    pub start_astx: bool,
    pub redis_url: String,
    pub credential_secret: String,
}
