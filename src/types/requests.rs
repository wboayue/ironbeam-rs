use serde::{Deserialize, Serialize};

use super::{DurationType, OrderSide, OrderType, Symbol};

/// Authentication request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// Username.
    pub username: String,

    /// Password.
    pub password: String,

    /// API key.
    #[serde(rename = "apiKey")]
    pub api_key: String,
}

/// Place order request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderRequest {
    /// Exchange-qualified symbol.
    #[serde(rename = "exchSym")]
    pub exch_sym: Symbol,

    /// Order side.
    pub side: OrderSide,

    /// Quantity.
    pub quantity: f64,

    /// Order type.
    #[serde(rename = "orderType")]
    pub order_type: OrderType,

    /// Duration.
    pub duration: DurationType,

    /// Limit price (required for LIMIT, STOP_LIMIT).
    #[serde(rename = "limitPrice", default, skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,

    /// Stop price (required for STOP, STOP_LIMIT).
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    /// Bracket stop loss price.
    #[serde(rename = "stopLoss", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<f64>,

    /// Bracket take profit price.
    #[serde(rename = "takeProfit", default, skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<f64>,

    /// Stop loss offset in pips.
    #[serde(rename = "stopLossOffset", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_offset: Option<f32>,

    /// Take profit offset in pips.
    #[serde(rename = "takeProfitOffset", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_offset: Option<f32>,

    /// Trailing stop (not yet supported).
    #[serde(rename = "trailingStop", default, skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<f32>,

    /// Wait for exchange order ID (default: true).
    #[serde(rename = "waitForOrderId", default, skip_serializing_if = "Option::is_none")]
    pub wait_for_order_id: Option<bool>,
}

/// Update order request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderUpdateRequest {
    /// Order identifier.
    #[serde(rename = "orderId")]
    pub order_id: String,

    /// New quantity.
    pub quantity: i32,

    /// New limit price.
    #[serde(rename = "limitPrice", default, skip_serializing_if = "Option::is_none")]
    pub limit_price: Option<f64>,

    /// New stop price.
    #[serde(rename = "stopPrice", default, skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,

    /// New stop loss price.
    #[serde(rename = "stopLoss", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<f64>,

    /// New take profit price.
    #[serde(rename = "takeProfit", default, skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<f64>,

    /// New stop loss offset.
    #[serde(rename = "stopLossOffset", default, skip_serializing_if = "Option::is_none")]
    pub stop_loss_offset: Option<f32>,

    /// New take profit offset.
    #[serde(rename = "takeProfitOffset", default, skip_serializing_if = "Option::is_none")]
    pub take_profit_offset: Option<f32>,
}

/// Cancel multiple orders request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderCancelMultipleRequest {
    /// Account identifier.
    #[serde(rename = "accountId")]
    pub account_id: String,

    /// Order IDs to cancel.
    #[serde(rename = "orderIds")]
    pub order_ids: Vec<String>,
}

// --- Simulation requests (demo only, PascalCase fields) ---

/// Create a simulated trader.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatedTraderCreateRequest {
    #[serde(rename = "FirstName")]
    pub first_name: String,
    #[serde(rename = "LastName")]
    pub last_name: String,
    #[serde(rename = "Address1")]
    pub address1: String,
    #[serde(rename = "Address2", default, skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(rename = "City")]
    pub city: String,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Country")]
    pub country: String,
    #[serde(rename = "ZipCode")]
    pub zip_code: String,
    #[serde(rename = "Phone")]
    pub phone: String,
    #[serde(rename = "Email")]
    pub email: String,
    #[serde(rename = "Password")]
    pub password: String,
    /// Template: XAP50 ($50k), XAP100 ($100k), XAP150 ($150k).
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

/// Add account to existing simulated trader.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedTraderAddAccountRequest {
    #[serde(rename = "TraderId")]
    pub trader_id: String,
    #[serde(rename = "Password")]
    pub password: String,
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

/// Reset simulated account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedAccountResetRequest {
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "TemplateId")]
    pub template_id: String,
}

/// Expire simulated account.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedAccountExpireRequest {
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

/// Add cash to simulated account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatedAccountAddCashRequest {
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "Amount")]
    pub amount: f32,
}

/// Set risk parameters for simulated account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatedAccountSetRiskRequest {
    #[serde(rename = "AccountId")]
    pub account_id: String,
    #[serde(rename = "LiquidationAccountValue", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_account_value: Option<f64>,
    #[serde(rename = "LiquidationLossFromStartOfDay", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_loss_from_start_of_day: Option<f64>,
    #[serde(rename = "LiquidationLossFromHighOfDay", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_loss_from_high_of_day: Option<f64>,
    #[serde(rename = "LiquidationLossFromHighOfMultiday", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_loss_from_high_of_multiday: Option<f64>,
    #[serde(rename = "LiquidationPctLossFromStartOfDay", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_pct_loss_from_start_of_day: Option<f64>,
    #[serde(rename = "LiquidationPctLossFromHighOfDay", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_pct_loss_from_high_of_day: Option<f64>,
    #[serde(rename = "LiquidationPctLossFromHighOfMultiday", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_pct_loss_from_high_of_multiday: Option<f64>,
    #[serde(rename = "LiquidationPctMarginDeficiency", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_pct_margin_deficiency: Option<f64>,
    #[serde(rename = "LiquidationMaxValueOverride", default, skip_serializing_if = "Option::is_none")]
    pub liquidation_max_value_override: Option<f64>,
    #[serde(rename = "ReducePositionsOnly", default, skip_serializing_if = "Option::is_none")]
    pub reduce_positions_only: Option<bool>,
    #[serde(rename = "RestoreTrading", default, skip_serializing_if = "Option::is_none")]
    pub restore_trading: Option<bool>,
    #[serde(rename = "MarginScheduleName", default, skip_serializing_if = "Option::is_none")]
    pub margin_schedule_name: Option<String>,
    #[serde(rename = "TemplateId", default, skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
}

/// Liquidate simulated accounts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatedAccountLiquidateRequest {
    #[serde(rename = "Accounts", default, skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    #[serde(rename = "Groups", default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "ExceptAccounts", default, skip_serializing_if = "Option::is_none")]
    pub except_accounts: Option<Vec<String>>,
    #[serde(rename = "ForceManualLiquidation", default, skip_serializing_if = "Option::is_none")]
    pub force_manual_liquidation: Option<bool>,
    #[serde(rename = "UseManualLiquidationForIlliquidMarkets", default, skip_serializing_if = "Option::is_none")]
    pub use_manual_liquidation_for_illiquid_markets: Option<bool>,
    #[serde(rename = "SendAccountEmail", default, skip_serializing_if = "Option::is_none")]
    pub send_account_email: Option<bool>,
    #[serde(rename = "SendOfficeEmail", default, skip_serializing_if = "Option::is_none")]
    pub send_office_email: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_request_serialize() {
        let req = OrderRequest {
            exch_sym: "XCME:ES.U16".into(),
            side: OrderSide::Buy,
            quantity: 5.0,
            order_type: OrderType::Limit,
            duration: DurationType::Day,
            limit_price: Some(4500.0),
            stop_price: None,
            stop_loss: None,
            take_profit: None,
            stop_loss_offset: None,
            take_profit_offset: None,
            trailing_stop: None,
            wait_for_order_id: None,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"exchSym\":\"XCME:ES.U16\""));
        assert!(json.contains("\"orderType\":\"2\""));
        assert!(json.contains("\"limitPrice\":4500.0"));
        assert!(!json.contains("stopPrice"));
    }

    #[test]
    fn auth_request_serialize() {
        let req = AuthorizationRequest {
            username: "user1".into(),
            password: "pass1".into(),
            api_key: "key123".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"apiKey\":\"key123\""));
    }
}
