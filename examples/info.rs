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
        .connect()
        .await?;

    // Trader info
    let trader = client.trader_info(None).await?;
    println!("Trader: {:?}, accounts: {:?}", trader.trader_id, trader.accounts);

    // User info
    let user = client.user_info(None).await?;
    println!("User: {:?}, email: {:?}", user.account_title, user.email_address_1);

    // Exchanges → complexes
    let exchanges = client.exchange_sources().await?;
    println!("\nExchanges: {exchanges:?}");

    if let Some(exchange) = exchanges.first() {
        let complexes = client.complexes(exchange).await?;
        println!("Complexes for {exchange}:");
        for c in &complexes {
            println!("  {:?}: {} groups", c.name, c.groups.len());
        }
    }

    // Symbol search
    let params = SymbolSearchParams::new().text("ES").limit(5).prefer_active(true);
    let symbols = client.symbols(&params).await?;
    println!("\nSymbol search 'ES':");
    for s in &symbols {
        println!("  {}: {:?} ({:?})", s.symbol, s.description, s.symbol_type);
    }

    // Security definitions
    if let Some(sym) = symbols.first() {
        let defs = client.security_definitions(&[&sym.symbol]).await?;
        println!("\nSecurity definition for {}:", sym.symbol);
        for d in &defs {
            println!(
                "  {}: {:?}, type={:?}",
                d.exch_sym, d.product_description, d.security_type
            );
        }

        let margins = client.security_margin(&[&sym.symbol]).await?;
        println!("\nMargin info for {}:", sym.symbol);
        for m in &margins {
            println!(
                "  {}: init_long={:?} init_short={:?}",
                m.exch_sym, m.initial_margin_long, m.initial_margin_short
            );
        }

        let statuses = client.security_status(&[&sym.symbol]).await?;
        println!("\nStatus for {}:", sym.symbol);
        for s in &statuses {
            println!("  {}: {:?}", s.exch_sym, s.status);
        }
    }

    // Strategy ID
    let strategy = client.strategy_id().await?;
    println!(
        "\nStrategy ID: {}, range: {}..{}",
        strategy.id, strategy.minimum, strategy.maximum
    );

    client.logout().await?;

    Ok(())
}
