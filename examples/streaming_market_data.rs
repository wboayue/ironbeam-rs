use std::env;
use std::error::Error;

use ironbeam_rs::client::stream::StreamEvent;
use ironbeam_rs::client::{Client, Credentials};

/// Stream real-time market data (quotes, depth, trades) from the Ironbeam API.
///
/// Usage:
///
/// ```sh
/// cargo run --example streaming_market_data -- [quote|depth|trades]
/// ```
///
/// Defaults to `quote` if no argument is given.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let stream_type = env::args().nth(1).unwrap_or_else(|| "quote".into());

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

    let mut stream = client.stream().start().await?;
    println!("Stream created: {}", stream.stream_id());

    // Update to the current front-month contract (e.g. ES.Z26 for Dec 2026).
    let symbols = &["XCME:ES.M26"];

    match stream_type.as_str() {
        "quote" => {
            stream.subscribe_quotes(symbols).await?;
            println!("Subscribed to quotes");
        }
        "depth" => {
            stream.subscribe_depth(symbols).await?;
            println!("Subscribed to depth");
        }
        "trades" => {
            stream.subscribe_trades(symbols).await?;
            println!("Subscribed to trades");
        }
        other => {
            eprintln!("Unknown stream type: {other}. Use quote, depth, or trades.");
            std::process::exit(1);
        }
    }

    while let Some(event) = stream.next().await {
        match event? {
            StreamEvent::Quotes(quotes) => {
                for q in &quotes {
                    println!(
                        "Quote {}: last={:?} bid={:?} ask={:?}",
                        q.symbol, q.last_price, q.bid, q.ask
                    );
                }
            }
            StreamEvent::Depth(depths) => {
                for d in &depths {
                    println!(
                        "Depth {}: bids={} {:?} asks={} {:?}",
                        d.symbol,
                        d.bids.len(),
                        d.bids.first().map(|b| b.price),
                        d.asks.len(),
                        d.asks.first().map(|a| a.price)
                    );
                }
            }
            StreamEvent::Trades(trades) => {
                for t in &trades {
                    println!(
                        "Trade {}: price={} change={:?} size={:?} dir={:?} side={:?} seq={:?} time={:?}",
                        t.symbol,
                        t.price,
                        t.change,
                        t.size,
                        t.tick_direction,
                        t.aggressor_side,
                        t.sequence_number,
                        t.send_time.map(|ts| ts.time()),
                    );
                }
            }
            StreamEvent::Ping(_) => println!("keepalive"),
            StreamEvent::Notification(r) => {
                println!("notification: {:?} {:?}", r.status, r.message)
            }
            _ => {}
        }
    }

    client.logout().await?;

    Ok(())
}
