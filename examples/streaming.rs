use std::env;
use std::error::Error;

use ironbeam_rs::client::stream::StreamEvent;
use ironbeam_rs::client::{Client, Credentials};

/// Stream real-time quotes from the Ironbeam API.
///
/// Set IRONBEAM_USERNAME, IRONBEAM_PASSWORD, and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example streaming
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

    let mut stream = client.stream().start().await?;
    println!("Stream created: {}", stream.stream_id());

    // Update to the current front-month contract (e.g. ES.Z26 for Dec 2026).
    stream.subscribe_quotes(&["XCME:ES.U26"]).await?;
    println!("Subscribed to quotes");

    let mut count = 0;
    while let Some(event) = stream.next().await {
        match event? {
            StreamEvent::Quotes(quotes) => {
                for q in &quotes {
                    println!("Quote {}: last={:?} bid={:?} ask={:?}", q.symbol, q.last_price, q.bid, q.ask);
                }
                count += 1;
                if count >= 5 {
                    break;
                }
            }
            StreamEvent::Ping(_) => println!("keepalive"),
            StreamEvent::Notification(r) => println!("notification: {:?} {:?}", r.status, r.message),
            event => {
                println!("other event: {:?}", event);
            }
        }
    }

    stream.close().await?;
    client.logout().await?;

    Ok(())
}
