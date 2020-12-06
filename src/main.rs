mod scrap;

use std::boxed::Box;
use std::error::Error;

use dotenv::dotenv;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    scrap::scrap_kbcard().await?;

    Ok(())
}
