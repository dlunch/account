use fantoccini::{Client, Locator};
use serde_json::json;
use webdriver::capabilities::Capabilities;

// let's set up the sequence of steps we want the browser to take
#[async_std::main]
async fn main() -> Result<(), fantoccini::error::CmdError> {
    let mut cap = Capabilities::new();
    let arg = json!({"args": ["-headless"]});
    cap.insert("moz:firefoxOptions".to_string(), arg);

    let mut c = Client::with_capabilities("http://localhost:4444", cap)
        .await
        .expect("failed to connect to WebDriver");

    // first, go to the Wikipedia page for Foobar
    c.goto("https://en.wikipedia.org/wiki/Foobar").await?;
    let url = c.current_url().await?;
    assert_eq!(url.as_ref(), "https://en.wikipedia.org/wiki/Foobar");

    // click "Foo (disambiguation)"
    c.find(Locator::Css(".mw-disambig")).await?.click().await?;

    // click "Foo Lake"
    c.find(Locator::LinkText("Foo Lake")).await?.click().await?;

    let url = c.current_url().await?;
    println!("{}", url.as_ref());

    c.close().await
}
