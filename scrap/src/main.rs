use std::process::Command;

use futures::stream::StreamExt;
use prost::Message;

mod config;
mod kbcard;
mod structs;
mod util;
mod webdriver;

use structs::CardScrapRequest;

#[async_std::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv::dotenv().ok();

    let config = envy::from_env::<config::Config>()?;

    if config.start_webdriver {
        Command::new("geckodriver").spawn()?;
    }
    if config.start_astx {
        Command::new("/opt/AhnLab/ASTx/astxdaemon").spawn()?;
    }

    let client = redis::Client::open(config.redis_url)?;
    let mut pubsub = client.get_async_connection().await?.into_pubsub();

    pubsub.subscribe("scrap").await?;
    let stream = pubsub.into_on_message();

    stream
        .for_each(|msg| async move {
            let payload: Vec<u8> = msg.get_payload().unwrap();

            if msg.get_channel_name() == "scrap" {
                let req = CardScrapRequest::decode(&*payload).unwrap();
                if req.card_company == 0 {
                    // TODO enum
                    kbcard::scrap_kbcard("test", "test").await.unwrap();
                }
            } else {
                panic!("Unknown channel {}", msg.get_channel_name())
            }
        })
        .await;

    Ok(())
}
