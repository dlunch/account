use std::env;
use std::time::Duration;

use async_std::task;
use fantoccini::Locator;
use maplit::hashmap;

use super::webdriver;

pub async fn scrap_kbcard() -> Result<(), fantoccini::error::CmdError> {
    let mut c = webdriver::create_webdriver_client().await;

    // TODO mobile kbcard doesn't have transaction list. we have to call pc version on vm with astx

    c.goto("https://m.kbcard.com").await?;

    // To login page
    c.find(Locator::Css(".hmBtn")).await?.click().await?;
    c.find(Locator::Css(".btnLogin")).await?.click().await?;

    // Show login form
    c.find(Locator::LinkText("아이디")).await?.click().await?;

    // Input credentials
    let kbcard_id = env::var("KBCARD_ID").unwrap();
    let kbcard_password = env::var("KBCARD_PW").unwrap();

    if kbcard_password.len() > 12 {
        panic!("kbcard password length must be lower or equal than 12 chars");
    }

    c.find(Locator::Id("userId")).await?.send_keys(&kbcard_id).await?;
    c.find(Locator::Id("userPwd")).await?.click().await?;

    let specialchars_map = hashmap! {
         '`' => "어금기호",
        '~' => "물결표시",
        '!' => "느낌표",
        '@' => "골뱅이",
        '#' => "우물정",
        '$' => "달러기호",
        '%' => "퍼센트",
        '^' => "꺽쇠",
        '&' => "앰퍼샌드",
        '*' => "별표",
        // TODO
    };
    for password_char in kbcard_password.chars() {
        if password_char.is_uppercase() {
            c.find(Locator::XPath("//*[@aria-label=\"쉬프트\"]")).await?.click().await?;
            c.find(Locator::XPath(&format!("//*[@aria-label=\"대문자{}\"]", password_char)))
                .await?
                .click()
                .await?;
            c.find(Locator::XPath("//*[@aria-label=\"쉬프트\"]")).await?.click().await?;
        } else if !password_char.is_alphanumeric() {
            let label = specialchars_map.get(&password_char).unwrap();
            c.find(Locator::XPath("//*[@aria-label=\"특수키\"]")).await?.click().await?;
            c.find(Locator::XPath(&format!("//*[@aria-label=\"{}\"]", label))).await?.click().await?;
            c.find(Locator::XPath("//*[@aria-label=\"특수키\"]")).await?.click().await?;
        } else {
            c.find(Locator::XPath(&format!("//*[@aria-label=\"{}\"]", password_char)))
                .await?
                .click()
                .await?;
        }
    }
    // webpage hides keyboard on max password len(12) reached
    if kbcard_password.len() < 12 {
        c.find(Locator::XPath("//*[@aria-label=\"입력완료\"]")).await?.click().await?;
    }

    c.find(Locator::Id("btnIdPwdLogin")).await?.click().await?;

    task::sleep(Duration::from_secs(10)).await;

    c.close().await
}
