use std::env;
use std::error::Error;

use ironbeam_rs::client::stream::StreamEvent;
use ironbeam_rs::client::{Client, Credentials};
use ironbeam_rs::types::BarType;
use ironbeam_rs::types::streaming::SubscribeBarsRequest;

/// Stream real-time indicator (bar) data from the Ironbeam API.
///
/// Usage:
///
/// ```sh
/// cargo run --example streaming_indicators -- [trade|tick|time|volume]
/// ```
///
/// Defaults to `trade` if no argument is given.
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let bar_kind = env::args().nth(1).unwrap_or_else(|| "trade".into());

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
    let symbol = "XCME:ES.U26";

    let req = SubscribeBarsRequest {
        symbol: symbol.into(),
        period: 1,
        bar_type: BarType::Minute,
        load_size: 100,
    };

    let resp = match bar_kind.as_str() {
        "trade" => stream.subscribe_trade_bars(&req).await?,
        "tick" => stream.subscribe_tick_bars(&req).await?,
        "time" => stream.subscribe_time_bars(&req).await?,
        "volume" => stream.subscribe_volume_bars(&req).await?,
        other => {
            eprintln!("Unknown bar kind: {other}. Use trade, tick, time, or volume.");
            std::process::exit(1);
        }
    };
    println!(
        "Subscribed to {bar_kind} bars: id={} names={:?}",
        resp.indicator_id, resp.value_names
    );

    while let Some(event) = stream.next().await {
        match event? {
            StreamEvent::TradeBars(bars) => {
                for b in &bars {
                    println!(
                        "TradeBar: o={:?} h={:?} l={:?} c={:?} v={:?}",
                        b.open, b.high, b.low, b.close, b.volume
                    );
                }
            }
            StreamEvent::TickBars(bars) => {
                for b in &bars {
                    println!(
                        "TickBar: o={:?} h={:?} l={:?} c={:?} v={:?}",
                        b.open, b.high, b.low, b.close, b.volume
                    );
                }
            }
            StreamEvent::TimeBars(bars) => {
                for b in &bars {
                    println!(
                        "TimeBar: o={:?} h={:?} l={:?} c={:?} v={:?}",
                        b.open, b.high, b.low, b.close, b.volume
                    );
                }
            }
            StreamEvent::VolumeBars(bars) => {
                for b in &bars {
                    println!(
                        "VolumeBar: o={:?} h={:?} l={:?} c={:?} v={:?}",
                        b.open, b.high, b.low, b.close, b.volume
                    );
                }
            }
            StreamEvent::Indicators(indicators) => {
                for ind in &indicators {
                    println!("Indicator {}: {} values", ind.name, ind.values.len());
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
