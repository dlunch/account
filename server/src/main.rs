mod scrap;

use std::boxed::Box;
use std::env;
use std::error::Error;

use dotenv::dotenv;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let kbcard_id = env::var("KBCARD_ID").unwrap();
    let kbcard_password = env::var("KBCARD_PW").unwrap();

    scrap::scrap_kbcard(&kbcard_id, &kbcard_password).await?;

    Ok(())
}
