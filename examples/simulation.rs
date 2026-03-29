use std::env;
use std::error::Error;

use ironbeam_rs::client::{Client, Credentials};
use ironbeam_rs::types::{
    SimulatedAccountAddCashRequest, SimulatedAccountExpireRequest, SimulatedAccountResetRequest,
    SimulatedTraderAddAccountRequest, SimulatedTraderCreateRequest,
};

/// Simulated account management workflow (demo only, enterprise feature).
///
/// Set IRONBEAM_USERNAME, IRONBEAM_PASSWORD, and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example simulation
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

    // 1. Create a simulated trader
    let trader_id = client
        .simulated_trader_create(&SimulatedTraderCreateRequest {
            first_name: "John".into(),
            last_name: "Doe".into(),
            address1: "123 Main St".into(),
            address2: None,
            city: "Chicago".into(),
            state: "IL".into(),
            country: "US".into(),
            zip_code: "60601".into(),
            phone: "555-0100".into(),
            email: "john@example.com".into(),
            password: "demo_password".into(),
            template_id: "XAP100".into(),
        })
        .await?;
    println!("Created trader: {trader_id}");

    // 2. Add an account to the trader
    let account_id = client
        .simulated_account_add(&SimulatedTraderAddAccountRequest {
            trader_id: trader_id.clone(),
            password: "demo_password".into(),
            template_id: "XAP50".into(),
        })
        .await?;
    println!("Added account: {account_id}");

    // 3. Add cash to the account
    client
        .simulated_account_add_cash(&SimulatedAccountAddCashRequest {
            account_id: account_id.clone(),
            amount: 25_000.0,
        })
        .await?;
    println!("Added $25,000 to {account_id}");

    // 4. Get cash report
    let start = time::Date::from_calendar_date(2025, time::Month::January, 1)?;
    let end = time::Date::from_calendar_date(2025, time::Month::December, 31)?;
    let report = client
        .simulated_account_cash_report(&account_id, start, end)
        .await?;
    println!("Cash report ({} entries):", report.cash_report.len());
    for entry in &report.cash_report {
        println!("  {entry:?}");
    }

    // 5. Reset the account
    client
        .simulated_account_reset(&SimulatedAccountResetRequest {
            account_id: account_id.clone(),
            template_id: "XAP100".into(),
        })
        .await?;
    println!("Reset {account_id}");

    // 6. Expire the account
    client
        .simulated_account_expire(&SimulatedAccountExpireRequest {
            account_id: account_id.clone(),
        })
        .await?;
    println!("Expired {account_id}");

    client.logout().await?;

    Ok(())
}
