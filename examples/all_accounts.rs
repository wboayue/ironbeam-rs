use std::env;
use std::error::Error;

use ironbeam_rs::client::{Client, Credentials};

/// List all accounts for the authenticated trader.
///
/// Set IRONBEAM_USERNAME, IRONBEAM_PASSWORD, and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example all_accounts
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::builder()
        .credentials(Credentials {
            username: env::var("IRONBEAM_USERNAME")?,
            password: env::var("IRONBEAM_PASSWORD")?,
            api_key: env::var("IRONBEAM_API_KEY")?,
        })
        .demo()
        .connect()
        .await?;

    let accounts = client.all_accounts().await?;
    println!("Accounts: {accounts:?}");

    Ok(())
}
