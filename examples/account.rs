use std::env;
use std::error::Error;

use ironbeam_rs::client::{Client, Credentials};
use ironbeam_rs::types::BalanceType;

/// Demonstrate account API endpoints.
///
/// Set IRONBEAM_USERNAME, IRONBEAM_PASSWORD, and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example account
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

    // List accounts
    let accounts = client.all_accounts().await?;
    println!("Accounts: {accounts:?}");

    // Per-account queries using first account
    if let Some(account_id) = accounts.first() {
        let balances = client.balance(account_id, BalanceType::CurrentOpen).await?;
        println!("\nBalances for {account_id}:");
        for b in &balances {
            println!(
                "  {} cash={:?} equity={:?}",
                b.currency_code, b.cash_balance, b.total_equity
            );
        }

        let positions = client.positions(account_id).await?;
        println!("\nPositions for {account_id}:");
        for p in &positions {
            println!(
                "  {:?} {:?} {:?} @ {:?}",
                p.exch_sym, p.side, p.quantity, p.price
            );
        }

        let risks = client.risk(account_id).await?;
        println!("\nRisk for {account_id}:");
        for r in &risks {
            println!(
                "  {:?} net_liq={:?}",
                r.currency_code, r.current_net_liquidation_value
            );
        }

        let fills = client.fills(account_id).await?;
        println!("\nFills for {account_id}: {} fill(s)", fills.len());
        for f in &fills {
            println!(
                "  {} {:?} @ {:?}",
                f.exch_sym, f.fill_quantity, f.fill_price
            );
        }
    }

    // All-account queries
    let all_balances = client.all_balances(BalanceType::CurrentOpen).await?;
    println!("\nAll balances: {} entries", all_balances.len());

    let all_positions = client.all_positions().await?;
    println!("All positions: {} accounts", all_positions.len());

    let all_risks = client.all_risk().await?;
    println!("All risk: {} entries", all_risks.len());

    let all_fills = client.all_fills().await?;
    println!("All fills: {} entries", all_fills.len());

    client.logout().await?;

    Ok(())
}
