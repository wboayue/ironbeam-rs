use std::env;
use std::error::Error;

use ironbeam_rs::client::{Client, Credentials, SymbolSearchParams};

/// Demonstrate info API endpoints.
///
/// Set IRONBEAM_USERNAME, IRONBEAM_PASSWORD, and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example info
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

    // Trader info
    let trader = client.trader_info(None).await?;
    println!(
        "Trader: {:?}, accounts: {:?}",
        trader.trader_id, trader.accounts
    );

    // User info
    let user = client.user_info(None).await?;
    println!(
        "User: {:?}, email: {:?}",
        user.account_title, user.email_address_1
    );

    // Exchanges
    let exchanges = client.exchange_sources().await?;
    println!("\nExchanges: {exchanges:?}");

    // Futures search — returns unqualified symbols (e.g. "ES.M26")
    let futures = client.futures_symbols("CME", "ES").await?;
    println!("\nFutures for CME ES ({} contracts):", futures.len());
    for f in futures.iter().take(4) {
        println!(
            "  {} {:?} ({:?}/{:?})",
            f.symbol, f.description, f.maturity_month, f.maturity_year
        );
    }

    // Symbol search
    let params = SymbolSearchParams::new().text("GOLD").limit(5);
    let symbols = client.symbols(&params).await?;
    println!("\nSymbol search 'GOLD':");
    for s in &symbols {
        println!("  {}: {:?} ({:?})", s.symbol, s.description, s.symbol_type);
    }

    // Security definitions — use a known exchange-qualified symbol
    let lookup_sym = symbols
        .first()
        .map(|s| s.symbol.clone())
        .unwrap_or_else(|| "XNYM:GCM.H26".into());

    let defs = client.security_definitions(&[&lookup_sym]).await?;
    println!("\nSecurity definition for {lookup_sym}:");
    for d in &defs {
        println!(
            "  {}: {:?}, type={:?}",
            d.exch_sym, d.product_description, d.security_type
        );
    }

    let margins = client.security_margin(&[&lookup_sym]).await?;
    println!("\nMargin info for {lookup_sym}:");
    for m in &margins {
        println!(
            "  {}: init_long={:?} init_short={:?}",
            m.exch_sym, m.initial_margin_long, m.initial_margin_short
        );
    }

    let statuses = client.security_status(&[&lookup_sym]).await?;
    println!("\nStatus for {lookup_sym}:");
    for s in &statuses {
        println!("  {}: {:?}", s.exch_sym, s.status);
    }

    // Strategy ID
    let strategy = client.strategy_id().await?;
    println!(
        "\nStrategy ID: {}, range: {}..{}",
        strategy.id, strategy.minimum, strategy.maximum
    );

    // Explicit logout preferred over drop-based cleanup for guaranteed session teardown.
    client.logout().await?;

    Ok(())
}
