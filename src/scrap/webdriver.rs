use std::env;

use fantoccini::Client;
use serde_json::json;
use webdriver::capabilities::Capabilities;

pub async fn create_webdriver_client() -> Client {
    let webdriver_host = env::var("WEBDRIVER_HOST").unwrap();
    let webdriver_headless = env::var("WEBDRIVER_HEADLESS").is_ok();

    let mut cap = Capabilities::new();
    if webdriver_headless {
        let arg = json!({"args": ["-headless"]});
        cap.insert("moz:firefoxOptions".to_string(), arg);
    }

    Client::with_capabilities(&webdriver_host, cap)
        .await
        .expect("failed to connect to WebDriver")
}
