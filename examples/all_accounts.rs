use std::env;

use ironbeam_rs::client::{Client, Credentials};

/// List all accounts for the authenticated trader.
///
/// Set IRONBEAM_USERNAME and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example all_accounts
/// ```
#[tokio::main]
async fn main() -> ironbeam_rs::error::Result<()> {
    let client = Client::new()
        .credentials(Credentials::ApiKey {
            username: env::var("IRONBEAM_USERNAME")?,
            api_key: env::var("IRONBEAM_API_KEY")?,
        })
        .demo()
        .connect()
        .await?;

    let accounts = client.all_accounts().await?;
    println!("Accounts: {accounts:?}");

    Ok(())
}
