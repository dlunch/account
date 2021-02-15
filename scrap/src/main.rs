use std::process::Command;

mod config;
mod kbcard;
mod webdriver;

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

    // TODO
    kbcard::scrap_kbcard("test", "test").await?;

    Ok(())
}
