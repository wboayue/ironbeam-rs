[![Build](https://github.com/wboayue/ironbeam-rs/workflows/CI/badge.svg)](https://github.com/wboayue/ironbeam-rs/actions/workflows/ci.yml)
[![License:MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![crates.io](https://img.shields.io/crates/v/ironbeam-rs.svg)](https://crates.io/crates/ironbeam-rs)
[![Documentation](https://img.shields.io/badge/Documentation-green.svg)](https://docs.rs/ironbeam-rs/latest/ironbeam_rs/)
[![Coverage Status](https://coveralls.io/repos/github/wboayue/ironbeam-rs/badge.png?branch=main)](https://coveralls.io/github/wboayue/ironbeam-rs?branch=main)

# ironbeam-rs

Async Rust client for the [Ironbeam](https://www.ironbeam.com/) futures trading API. Targets low-latency order execution and real-time streaming.

## Features

- **REST API** &mdash; accounts, market data, orders, info, simulation
- **WebSocket streaming** &mdash; real-time quotes, depth, trades, indicators
- **Built-in rate limiting** &mdash; configurable requests-per-second throttle
- **Type-safe builders** &mdash; `OrderBuilder`, `SymbolSearchParams`
- Fully async (`tokio`), zero `unwrap()` in library code

## Quick Start

```toml
[dependencies]
ironbeam-rs = "0.1"
tokio = { version = "1", features = ["rt-multi-thread", "macros"] }
```

```rust
use ironbeam_rs::client::{Client, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .credentials(Credentials {
            username: "user".into(),
            password: "pass".into(),
            api_key: "key".into(),
        })
        .demo()
        .rate_limit(8)
        .connect()
        .await?;

    let accounts = client.all_accounts().await?;
    println!("Accounts: {accounts:?}");

    client.logout().await?;
    Ok(())
}
```

Set credentials via environment variables:

```sh
export IRONBEAM_USERNAME=...
export IRONBEAM_PASSWORD=...
export IRONBEAM_API_KEY=...
```

## Account Data

```rust
use ironbeam_rs::types::BalanceType;

let accounts = client.all_accounts().await?;
let account_id = &accounts[0];

let balances = client.balance(account_id, BalanceType::CurrentOpen).await?;
for b in &balances {
    println!("{}: cash={:?} equity={:?}", b.currency_code, b.cash_balance, b.total_equity);
}

let positions = client.positions(account_id).await?;
let risks = client.risk(account_id).await?;
let fills = client.fills(account_id).await?;
```

## Market Data

```rust
use ironbeam_rs::client::SymbolSearchParams;

// Quotes and depth
let quotes = client.quotes(&["XCME:ES.U26"]).await?;
let depths = client.depth(&["XCME:ES.U26"]).await?;

// Historical trades
let now = time::OffsetDateTime::now_utc();
let trades = client.trades("XCME:ES.U26", now - time::Duration::HOUR, now, 50, true).await?;

// Symbol search
let params = SymbolSearchParams::new().text("GOLD").limit(5);
let symbols = client.symbols(&params).await?;
```

## Orders

```rust
use ironbeam_rs::client::OrderBuilder;
use ironbeam_rs::types::{OrderSide, DurationType, OrderStatusType};

// Place a limit order
let order = OrderBuilder::limit("XCME:ES.U26", OrderSide::Buy, 1.0, 4500.0, DurationType::Day)
    .stop_loss(4480.0)
    .take_profit(4550.0);
let resp = client.place_order("ACC001", &order).await?;

// Query and cancel
let orders = client.orders("ACC001", OrderStatusType::Any).await?;
if let Some(id) = resp.order_id.as_deref() {
    client.cancel_order("ACC001", id).await?;
}
```

Order types: `OrderBuilder::market(...)`, `limit(...)`, `stop(...)`, `stop_limit(...)`.

## Streaming

```rust
use ironbeam_rs::client::stream::StreamEvent;

let mut stream = client.stream().start().await?;
stream.subscribe_quotes(&["XCME:ES.U26"]).await?;

while let Some(event) = stream.next().await {
    match event? {
        StreamEvent::Quotes(quotes) => {
            for q in &quotes {
                println!("{}: last={:?} bid={:?} ask={:?}", q.symbol, q.last_price, q.bid, q.ask);
            }
        }
        StreamEvent::Depth(depths) => { /* ... */ }
        StreamEvent::Trades(trades) => { /* ... */ }
        _ => {}
    }
}
```

## Examples

```sh
cargo run --example account
cargo run --example info
cargo run --example market
cargo run --example orders
cargo run --example streaming_market_data -- quote
cargo run --example streaming_indicators
cargo run --example simulation
```

## License

MIT &mdash; see [LICENSE](LICENSE).
