use std::env;
use std::time::Duration;

use async_std::task;
use dotenv::dotenv;
use fantoccini::{Client, Locator};
use serde_json::json;
use webdriver::capabilities::Capabilities;

#[async_std::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    dotenv().ok();

    let kbcard_id = env::var("KBCARD_ID").unwrap();
    let kbcard_password = env::var("KBCARD_PW").unwrap();
    let webdriver_host = env::var("WEBDRIVER_HOST").unwrap();

    let webdriver_headless = env::var("WEBDRIVER_HEADLESS").is_ok();

    let mut cap = Capabilities::new();
    if webdriver_headless {
        let arg = json!({"args": ["-headless"]});
        cap.insert("moz:firefoxOptions".to_string(), arg);
    }

    let mut c = Client::with_capabilities(&webdriver_host, cap)
        .await
        .expect("failed to connect to WebDriver");

    c.goto("https://m.kbcard.com").await?;

    // To login page
    c.find(Locator::Css(".hmBtn")).await?.click().await?;
    c.find(Locator::Css(".btnLogin")).await?.click().await?;

    // Show login form
    c.find(Locator::LinkText("아이디")).await?.click().await?;

    // Input credentials
    c.find(Locator::Id("userId")).await?.send_keys(&kbcard_id).await?;
    c.find(Locator::Id("userPwd")).await?.click().await?;
    for password_char in kbcard_password.chars() {
        c.find(Locator::XPath(&format!("//*[@aria-label=\"{}\"]", password_char)))
            .await?
            .click()
            .await?;
    }
    c.find(Locator::XPath("//*[@aria-label=\"입력완료\"]")).await?.click().await?;

    c.find(Locator::Id("btnIdPwdLogin")).await?.click().await?;

    task::sleep(Duration::from_secs(10)).await;

    c.close().await
}
