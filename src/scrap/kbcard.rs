use std::time::Duration;

use async_std::task;
use async_trait::async_trait;
use fantoccini::{error::CmdError, Client, Locator};
use serde_json::{json, Value};

use super::webdriver;

#[async_trait]
trait WebDriverClientExtension {
    async fn click_keypad_at(&mut self, x: isize, y: isize) -> Result<Value, CmdError>;
}

#[async_trait]
impl WebDriverClientExtension for Client {
    async fn click_keypad_at(&mut self, x: isize, y: isize) -> Result<Value, CmdError> {
        let res = self
            .execute(
                "
                const nodes = document.querySelectorAll('li.sp');
                for (const node of nodes) {
                    const style = window.getComputedStyle(node);
                    if (style.backgroundPositionX === arguments[0] + 'px' && style.backgroundPositionY === arguments[1] + 'px') {
                        const rect = node.getBoundingClientRect();
                        const event = new MouseEvent('click', {clientX: rect.x, clientY: rect.y, bubbles: true})
                        node.dispatchEvent(event);
                        return;
                    }
                }
                throw 'no such element';
            ",
                vec![json!(x), json!(y)],
            )
            .await;

        res
    }
}

pub async fn scrap_kbcard(id: &str, password: &str) -> Result<(), CmdError> {
    if password.len() > 12 {
        panic!("kbcard password length must be lower or equal than 12 chars");
    }

    let mut c = webdriver::create_webdriver_client().await;

    c.goto("https://card.kbcard.com").await?;

    // To login page
    c.find(Locator::Id("loginLinkBtn")).await?.click().await?;

    // Show login form
    c.find(Locator::Id("perTab01")).await?.click().await?;

    // Input credentials

    c.find(Locator::Id("인터넷서비스로그인ID")).await?.send_keys(&id).await?;
    c.find(Locator::Id("loginPwd")).await?.click().await?;

    let keys = [
        vec!['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'],
        vec!['q', 'w', 'e', 'r', 't', 'y', 'u', 'i', 'o', 'p'],
        vec!['a', 's', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'z'],
        vec!['x', 'c', 'v', 'b', 'n', 'm', 'Q', 'W', 'E', 'R'],
        vec!['T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F'],
        vec!['G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B'],
        vec!['N', 'M', '`', '-', '=', '[', ']', '/', ';', '\''],
        vec![',', '.', '/', '~', '!', '@', '#', '$', '%', '^'],
        vec!['&', '*', '(', ')', '_', '+', '{', '}', '|', ':'],
        vec!['"', '<', '>', '?', ' '],
    ];
    let shift_specialchars = [
        '~', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '+', '{', '}', '|', ':', '"', '<', '>', '?',
    ];

    let key_width = 42;
    let key_height = 34;
    let find_key_pos = |key| {
        for (y, row) in keys.iter().enumerate() {
            for (x, &item) in row.iter().enumerate() {
                if item == key {
                    return [-(x as isize) * key_width, -(y as isize) * key_height];
                }
            }
        }
        panic!()
    };

    for password_char in password.chars() {
        let [x, y] = find_key_pos(password_char);
        if password_char.is_uppercase() || shift_specialchars.contains(&password_char) {
            c.click_keypad_at(-180, -340).await?; // shift
        }
        c.click_keypad_at(x, y).await?;
    }
    c.click_keypad_at(-278, -374).await?; // finish

    c.find(Locator::Id("doIdLogin")).await?.click().await?;

    c.wait_for_navigation(None).await?;

    task::sleep(Duration::from_secs(1)).await; // ??? we need this
    println!("{}", c.find(Locator::Css("#BeyondViewAreaDivId em")).await?.text().await?);

    c.close().await
}
