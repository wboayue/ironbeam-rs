//! [![github]](https://github.com/wboayue/ironbeam-rs)  [![crates-io]](https://crates.io/crates/ironbeam-rs)  [![license]](https://opensource.org/licenses/MIT)
//!
//! [github]: https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github
//! [crates-io]: https://img.shields.io/badge/crates.io-fc8d62?style=for-the-badge&labelColor=555555&logo=rust
//! [license]: https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge&labelColor=555555
//!
//! <br>
//!
//! An async Rust client for the [Ironbeam](https://www.ironbeam.com/) futures trading API,
//! providing both REST and WebSocket streaming interfaces for account management,
//! order execution, and real-time market data.
//!
//! # Quick Start
//!
//! Connect to the demo environment and query account balances:
//!
//! ```no_run
//! use ironbeam_rs::client::{Client, Credentials};
//! use ironbeam_rs::types::BalanceType;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::builder()
//!         .credentials(Credentials {
//!             username: "user".into(),
//!             password: "pass".into(),
//!             api_key: "key".into(),
//!         })
//!         .demo()
//!         .connect()
//!         .await?;
//!
//!     let accounts = client.all_accounts().await?;
//!     let balance = client.balance(&accounts[0], BalanceType::CurrentOpen).await?;
//!     println!("{balance:?}");
//!
//!     client.logout().await?;
//!     Ok(())
//! }
//! ```
//!
//! # Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`client`] | [`Client`](client::Client) builder, connection, and REST API methods |
//! | [`client::stream`] | WebSocket streaming for real-time quotes, depth, trades, orders, and indicator bars |
//! | [`types`] | Domain types — accounts, orders, market data, enums, and serde helpers |
//! | [`error`] | [`Error`](error::Error) enum and [`Result`](error::Result) alias |
//!
//! # REST API
//!
//! All REST methods are on [`Client`](client::Client). Authentication is handled
//! automatically by [`ClientBuilder::connect()`](client::ClientBuilder::connect).
//!
//! ```no_run
//! # use ironbeam_rs::client::{Client, Credentials};
//! # async fn example() -> ironbeam_rs::error::Result<()> {
//! # let client = Client::builder()
//! #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
//! #     .demo().connect().await?;
//! // Account data
//! let accounts = client.all_accounts().await?;
//! let positions = client.positions(&accounts[0]).await?;
//!
//! // Market data
//! let quotes = client.quotes(&["ESM5"]).await?;
//! let depth = client.depth(&["ESM5"]).await?;
//!
//! // Orders
//! use ironbeam_rs::client::OrderBuilder;
//! use ironbeam_rs::types::{OrderSide, DurationType};
//! let order = OrderBuilder::limit("ESM5", OrderSide::Buy, 1.0, 5000.0, DurationType::Day);
//! let receipt = client.place_order(&accounts[0], &order).await?;
//!
//! // Reference data
//! let exchanges = client.exchange_sources().await?;
//! let info = client.trader_info(None).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Streaming
//!
//! Real-time data via WebSocket. Subscribe to quotes, depth, trades, account
//! updates, and indicator bars (trade/tick/time/volume).
//!
//! ```no_run
//! # use ironbeam_rs::client::{Client, Credentials};
//! # use ironbeam_rs::client::stream::StreamEvent;
//! # async fn example() -> ironbeam_rs::error::Result<()> {
//! # let client = Client::builder()
//! #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
//! #     .demo().connect().await?;
//! let mut stream = client.stream().start().await?;
//!
//! stream.subscribe_quotes(&["ESM5"]).await?;
//! stream.subscribe_depth(&["ESM5"]).await?;
//!
//! while let Some(Ok(event)) = stream.next().await {
//!     match event {
//!         StreamEvent::Quotes(quotes) => println!("{quotes:?}"),
//!         StreamEvent::Depth(depth) => println!("{depth:?}"),
//!         _ => {}
//!     }
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Error Handling
//!
//! All methods return [`error::Result<T>`]. The [`Error`](error::Error) enum
//! covers HTTP transport, JSON parsing, API-level errors (with status code),
//! authentication failures, and WebSocket errors.
//!
//! ```no_run
//! # use ironbeam_rs::client::{Client, Credentials};
//! # use ironbeam_rs::error::Error;
//! # async fn example() -> ironbeam_rs::error::Result<()> {
//! # let client = Client::builder()
//! #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
//! #     .demo().connect().await?;
//! match client.quotes(&["INVALID"]).await {
//!     Ok(quotes) => println!("{quotes:?}"),
//!     Err(Error::Api { status: 429, .. }) => eprintln!("rate limited, back off"),
//!     Err(e) => eprintln!("error: {e}"),
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Rate Limiting
//!
//! The API enforces a hard limit of 10 requests/second. The client has optional
//! built-in rate limiting via [`ClientBuilder::rate_limit()`](client::ClientBuilder::rate_limit):
//!
//! ```no_run
//! # use ironbeam_rs::client::{Client, Credentials};
//! # async fn example() -> ironbeam_rs::error::Result<()> {
//! let client = Client::builder()
//!     .credentials(Credentials {
//!         username: "u".into(),
//!         password: "p".into(),
//!         api_key: "k".into(),
//!     })
//!     .demo()
//!     .rate_limit(8) // stay under the 10/sec hard limit
//!     .connect()
//!     .await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Feature Highlights
//!
//! - **Async/await** — built on `tokio`, `hyper`, and `fastwebsockets`
//! - **Unified types** — single structs deserialize from both REST (camelCase) and streaming (abbreviated) wire formats via `serde` aliases
//! - **Builder pattern** — fluent construction for clients, orders, and queries
//! - **Typed errors** — no `unwrap()` in library code; all failures surface as [`Error`](error::Error) variants
//! - **Rate limiting** — optional client-side throttling to avoid 429 cooldowns
//! - **Secure by default** — TLS via `rustls`, auth headers redacted in `Debug` output

/// Authenticated client, builders, and REST/streaming API methods.
pub mod client;
/// Error types and result alias.
pub mod error;
/// Domain model — accounts, orders, market data, enums, and serde helpers.
pub mod types;
