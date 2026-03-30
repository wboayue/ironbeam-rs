use std::env;
use std::error::Error;

use ironbeam_rs::client::{Client, Credentials};

/// Look up the front-month contract for a futures product, rolling to
/// the next contract when expiration is within 5 calendar days.
///
/// Pass an exchange and product root as arguments, or defaults to CME ES.
///
/// ```sh
/// cargo run --example front_month
/// cargo run --example front_month -- CME NQ
/// cargo run --example front_month -- CEC GC
/// ```
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let exchange = env::args().nth(1).unwrap_or_else(|| "CME".into());
    let product = env::args().nth(2).unwrap_or_else(|| "ES".into());

    let client = Client::builder()
        .credentials(Credentials {
            username: env::var("IRONBEAM_USERNAME")?,
            password: env::var("IRONBEAM_PASSWORD")?,
            api_key: env::var("IRONBEAM_API_KEY")?,
        })
        .demo()
        .rate_limit(8)
        .connect()
        .await?;

    let roll_days = 5;
    let symbol = client.front_month(&exchange, &product, roll_days).await?;
    println!("Front month for {exchange} {product}: {symbol}");

    // Fetch a quote
    let quotes = client.quotes(&[&symbol]).await?;
    if let Some(q) = quotes.first() {
        println!(
            "  last={:?} bid={:?} ask={:?} vol={:?}",
            q.last_price, q.bid, q.ask, q.total_volume
        );
    }

    client.logout().await?;

    Ok(())
}
