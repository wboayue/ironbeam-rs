use std::env;
use std::error::Error;

use ironbeam_rs::client::{Client, Credentials};

/// Demonstrate market data API endpoints.
///
/// Set IRONBEAM_USERNAME, IRONBEAM_PASSWORD, and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example market
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
        .rate_limit(8)
        .connect()
        .await?;

    let symbol = "XCEC:GC.J26";

    // Quotes
    let quotes = client.quotes(&[symbol]).await?;
    println!("Quotes for {symbol}:");
    for q in &quotes {
        println!(
            "  last={:?} bid={:?} ask={:?} vol={:?}",
            q.last_price, q.bid, q.ask, q.total_volume
        );
    }

    // Depth
    let depths = client.depth(&[symbol]).await?;
    println!("\nDepth for {symbol}:");
    for d in &depths {
        println!("  {} bids, {} asks", d.bids.len(), d.asks.len());
        for b in d.bids.iter().take(3) {
            println!("    bid: {:?} x {:?}", b.price, b.size);
        }
        for a in d.asks.iter().take(3) {
            println!("    ask: {:?} x {:?}", a.price, a.size);
        }
    }

    // Historical trades
    let now = time::OffsetDateTime::now_utc();
    let hour_ago = now - time::Duration::HOUR;
    let trades = client.trades(symbol, hour_ago, now, 10, true).await?;
    println!(
        "\nTrades for {symbol} (last hour): {} trade(s)",
        trades.len()
    );
    for t in &trades {
        println!(
            "  {:?} @ {:?} size={:?} dir={:?}",
            t.symbol, t.price, t.size, t.tick_direction
        );
    }

    client.logout().await?;

    Ok(())
}
