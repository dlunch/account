use std::env;

use fantoccini::{Client, ClientBuilder};
use serde_json::json;
use webdriver::capabilities::Capabilities;

pub async fn create_webdriver_client() -> Client {
    let webdriver_host = env::var("WEBDRIVER_HOST").unwrap();
    let webdriver_headed = env::var("WEBDRIVER_HEADED").is_ok();

    let mut cap = Capabilities::new();
    if !webdriver_headed {
        let arg = json!({"args": ["-headless"]});
        cap.insert("moz:firefoxOptions".into(), arg);
    }
    cap.insert("acceptInsecureCerts".into(), json!(true));

    ClientBuilder::native().capabilities(cap).connect(&webdriver_host).await.unwrap()
}
