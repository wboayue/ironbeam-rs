use std::env;
use std::error::Error;

use ironbeam_rs::client::{Client, Credentials, OrderBuilder};
use ironbeam_rs::types::{DurationType, OrderSide, OrderStatusType};

/// Demonstrate order API endpoints.
///
/// Set IRONBEAM_USERNAME, IRONBEAM_PASSWORD, and IRONBEAM_API_KEY environment variables before running:
///
/// ```sh
/// cargo run --example orders
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

    let accounts = client.all_accounts().await?;
    let account_id = accounts.first().expect("no accounts");
    println!("Using account: {account_id}");

    // List current orders
    let orders = client.orders(account_id, OrderStatusType::Any).await?;
    println!("\nCurrent orders: {}", orders.len());
    for o in &orders {
        println!(
            "  {} {} {:?} {:?} qty={} {:?}",
            o.order_id, o.exch_sym, o.side, o.order_type, o.quantity, o.status
        );
    }

    // Place a limit order (far from market to avoid fills)
    let order = OrderBuilder::limit("XCEC:GC.J26", OrderSide::Buy, 1.0, 1000.0, DurationType::Day);
    let resp = client.place_order(account_id, &order).await?;
    println!(
        "\nPlaced order: id={:?} strategy={:?}",
        resp.order_id, resp.strategy_id
    );

    // List orders again
    let orders = client.orders(account_id, OrderStatusType::Any).await?;
    println!("\nOrders after placement: {}", orders.len());
    for o in &orders {
        println!("  {} {:?} {:?}", o.order_id, o.side, o.status);
    }

    // Cancel the order
    if let Some(order_id) = resp.order_id.as_deref() {
        let cancelled = client.cancel_order(account_id, order_id).await?;
        println!("\nCancelled: {} order(s)", cancelled.len());
    }

    // Check fills
    let fills = client.order_fills(account_id).await?;
    println!("\nFills: {} fill(s)", fills.len());
    for f in &fills {
        println!(
            "  {} {:?} @ {:?}",
            f.exch_sym, f.fill_quantity, f.fill_price
        );
    }

    client.logout().await?;

    Ok(())
}
