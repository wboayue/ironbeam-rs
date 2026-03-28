use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};

use super::{BalanceType, PositionSide, RegCodeType, Symbol, option_date_yyyymmdd, option_timestamp_ms};

/// Account balance. Unified across REST and streaming.
///
/// REST uses full camelCase field names; streaming uses abbreviated aliases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Balance {
    /// Account identifier.
    #[serde(rename = "accountId", alias = "a")]
    pub account_id: String,

    /// Currency code (e.g. "USD").
    #[serde(rename = "currencyCode", alias = "cc")]
    pub currency_code: String,

    /// Cash balance.
    #[serde(rename = "cashBalance", alias = "cb", default)]
    pub cash_balance: Option<f64>,

    /// Available cash balance.
    #[serde(rename = "cashBalanceAvailable", alias = "cba", default)]
    pub cash_balance_available: Option<f64>,

    /// Open trade equity.
    #[serde(rename = "openTradeEquity", alias = "ote", default)]
    pub open_trade_equity: Option<f64>,

    /// Total equity.
    #[serde(rename = "totalEquity", alias = "te", default)]
    pub total_equity: Option<f64>,

    /// Cash added today.
    #[serde(rename = "cashAddedToday", alias = "cbta", default)]
    pub cash_added_today: Option<f64>,

    /// Net liquidity.
    #[serde(rename = "netLiquidity", alias = "nl", default)]
    pub net_liquidity: Option<f64>,

    /// Available net liquidity.
    #[serde(rename = "netLiquidityAvailable", alias = "nla", default)]
    pub net_liquidity_available: Option<f64>,

    /// Days on margin call.
    #[serde(rename = "daysOnCall", alias = "dc", default)]
    pub days_on_call: Option<i64>,

    /// Balance type.
    #[serde(rename = "balanceType", alias = "bt", default)]
    pub balance_type: Option<BalanceType>,

    /// Margin information.
    #[serde(rename = "marginInfo", alias = "mi", default)]
    pub margin_info: Option<MarginInfo>,
}

/// Margin information. Unified across REST and streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginInfo {
    /// Account identifier.
    #[serde(rename = "accountId", alias = "a")]
    pub account_id: String,

    /// Currency code.
    #[serde(rename = "currencyCode", alias = "cc")]
    pub currency_code: String,

    /// Margin detail (positions only).
    #[serde(rename = "marginO", alias = "mo", default)]
    pub margin_o: Option<MarginDetail>,

    /// Margin detail (with orders).
    #[serde(rename = "marginOW", alias = "mow", default)]
    pub margin_ow: Option<MarginDetail>,

    /// Margin detail (with orders and implied).
    #[serde(rename = "marginOWI", alias = "mowi", default)]
    pub margin_owi: Option<MarginDetail>,
}

/// Margin calculation detail. Unified across REST and streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginDetail {
    /// Margin error message.
    #[serde(rename = "marginError", alias = "me", default)]
    pub margin_error: Option<String>,

    /// Symbols with errors.
    #[serde(rename = "errorSymbols", alias = "es", default)]
    pub error_symbols: Option<String>,

    /// Initial risk margin.
    #[serde(rename = "initialRiskMargin", alias = "irm", default)]
    pub initial_risk_margin: Option<f64>,

    /// Maintenance risk margin.
    #[serde(rename = "maintenanceRiskMargin", alias = "mrm", default)]
    pub maintenance_risk_margin: Option<f64>,

    /// Initial total margin.
    #[serde(rename = "initialTotalMargin", alias = "itm", default)]
    pub initial_total_margin: Option<f64>,

    /// Maintenance total margin.
    #[serde(rename = "maintenanceTotalMargin", alias = "mtm", default)]
    pub maintenance_total_margin: Option<f64>,

    /// Whether margin is estimated.
    #[serde(rename = "isEstimated", alias = "ie", default)]
    pub is_estimated: Option<bool>,

    /// Timestamp of margin calculation.
    #[serde(rename = "asOfTime", alias = "t", default, with = "option_timestamp_ms")]
    pub as_of_time: Option<OffsetDateTime>,
}

/// Account position. Unified across REST and streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Position {
    /// Account identifier.
    #[serde(rename = "accountId", alias = "a")]
    pub account_id: String,

    /// Currency code.
    #[serde(rename = "currencyCode", alias = "cc", default)]
    pub currency_code: Option<String>,

    /// Exchange-qualified symbol.
    #[serde(rename = "exchSym", alias = "s", default)]
    pub exch_sym: Option<Symbol>,

    /// Position identifier.
    #[serde(rename = "positionId", alias = "pId", default)]
    pub position_id: Option<String>,

    /// Position quantity.
    #[serde(alias = "q", default)]
    pub quantity: Option<f64>,

    /// Entry price.
    #[serde(alias = "p", default)]
    pub price: Option<f64>,

    /// Date position was opened.
    #[serde(rename = "dateOpened", alias = "do", default, with = "option_date_yyyymmdd")]
    pub date_opened: Option<Date>,

    /// Position side (LONG or SHORT).
    #[serde(alias = "sd", default)]
    pub side: Option<PositionSide>,

    /// Unrealized profit/loss.
    #[serde(rename = "unrealizedPL", alias = "upl", default)]
    pub unrealized_pl: Option<f64>,
}

/// Account risk information. Unified across REST and streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RiskInfo {
    /// Account identifier.
    #[serde(rename = "accountId", alias = "a")]
    pub account_id: String,

    /// Regulatory code.
    #[serde(rename = "regCode", alias = "rc", default)]
    pub reg_code: Option<RegCodeType>,

    /// Currency code.
    #[serde(rename = "currencyCode", alias = "cc", default)]
    pub currency_code: Option<String>,

    /// Liquidation trigger value.
    #[serde(rename = "liquidationValue", alias = "lv", default)]
    pub liquidation_value: Option<f64>,

    /// Start-of-day net liquidation value.
    #[serde(rename = "startNetLiquidationValue", alias = "snlv", default)]
    pub start_net_liquidation_value: Option<f64>,

    /// Current net liquidation value.
    #[serde(rename = "currentNetLiquidationValue", alias = "cnlv", default)]
    pub current_net_liquidation_value: Option<f64>,

    /// Max net liquidation value (intraday).
    #[serde(rename = "maxNetLiquidationValue", alias = "mnlv", default)]
    pub max_net_liquidation_value: Option<f64>,

    /// Max net liquidation value (multi-day). REST only.
    #[serde(rename = "maxNetLiquidationValueMultiDay", default)]
    pub max_net_liquidation_value_multi_day: Option<f64>,

    /// Liquidation event codes.
    #[serde(rename = "liquidationEvents", alias = "le", default)]
    pub liquidation_events: Option<Vec<i32>>,
}

/// Streaming snapshot: all positions for an account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountPositions {
    /// Account identifier.
    #[serde(rename = "accountId", alias = "a")]
    pub account_id: String,

    /// Positions for this account.
    #[serde(alias = "p", default)]
    pub positions: Vec<Position>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn balance_from_rest() {
        let json = r#"{
            "accountId": "ACC001",
            "currencyCode": "USD",
            "cashBalance": 50000.0,
            "totalEquity": 55000.0,
            "balanceType": "CURRENT_OPEN"
        }"#;
        let b: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(b.account_id, "ACC001");
        assert_eq!(b.cash_balance, Some(50000.0));
        assert_eq!(b.balance_type, Some(BalanceType::CurrentOpen));
    }

    #[test]
    fn balance_from_streaming() {
        let json = r#"{
            "a": "ACC001",
            "cc": "USD",
            "cb": 50000.0,
            "te": 55000.0,
            "bt": "CURRENT_OPEN"
        }"#;
        let b: Balance = serde_json::from_str(json).unwrap();
        assert_eq!(b.account_id, "ACC001");
        assert_eq!(b.cash_balance, Some(50000.0));
        assert_eq!(b.balance_type, Some(BalanceType::CurrentOpen));
    }

    #[test]
    fn position_from_rest() {
        let json = r#"{
            "accountId": "ACC001",
            "exchSym": "XCME:ES.U16",
            "quantity": 2.0,
            "price": 4500.0,
            "side": "LONG"
        }"#;
        let p: Position = serde_json::from_str(json).unwrap();
        assert_eq!(p.account_id, "ACC001");
        assert_eq!(p.side, Some(PositionSide::Long));
    }

    #[test]
    fn position_from_streaming() {
        let json = r#"{
            "a": "ACC001",
            "s": "XCME:ES.U16",
            "q": 2.0,
            "p": 4500.0,
            "sd": "LONG"
        }"#;
        let p: Position = serde_json::from_str(json).unwrap();
        assert_eq!(p.account_id, "ACC001");
        assert_eq!(p.quantity, Some(2.0));
        assert_eq!(p.side, Some(PositionSide::Long));
    }

    #[test]
    fn risk_info_from_streaming() {
        let json = r#"{
            "a": "ACC001",
            "rc": "COMBINED",
            "cc": "USD",
            "lv": 10000.0,
            "cnlv": 55000.0
        }"#;
        let r: RiskInfo = serde_json::from_str(json).unwrap();
        assert_eq!(r.account_id, "ACC001");
        assert_eq!(r.reg_code, Some(RegCodeType::Combined));
        assert_eq!(r.liquidation_value, Some(10000.0));
    }
}
