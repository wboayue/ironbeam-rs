use serde::{Deserialize, Serialize};

use super::{
    Balance, ComplexGroupInfo, ComplexGroups, FutureInfo, OptionGroupInfo, OrderFill,
    Position, RiskInfo, SecurityDefinition, SecurityMarginAndValue, SecurityStatus, Spread,
    Symbol, SymbolInfo, TraderInfo, UserInfo,
};
use super::account::AccountPositions;
use super::common::Response;
use super::market::{Depth, QuoteFull, Trade};
use super::order::Order;

/// Authentication response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    pub status: super::ResponseStatus,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub token: Option<String>,
}

/// Account balance response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountBalanceResponse {
    #[serde(default)]
    pub balances: Vec<Balance>,
}

/// Positions for a single account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PositionsResponse {
    #[serde(rename = "accountId", default)]
    pub account_id: Option<String>,
    #[serde(default)]
    pub positions: Vec<Position>,
}

/// Positions across all accounts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountsPositionsResponse {
    #[serde(default)]
    pub positions: Vec<AccountPositions>,
}

/// Account risk response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountRiskResponse {
    #[serde(default)]
    pub risks: Vec<RiskInfo>,
}

/// Account fills response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccountFillsResponse {
    #[serde(default)]
    pub fills: Vec<OrderFill>,
}

/// All accounts response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AllAccountsResponse {
    #[serde(default)]
    pub accounts: Vec<String>,
}

/// Order base response (place/cancel).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OrderBaseResponse {
    #[serde(rename = "orderId", default)]
    pub order_id: Option<String>,
    #[serde(rename = "strategyId", default)]
    pub strategy_id: Option<i64>,
}

/// Orders response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdersResponse {
    #[serde(default)]
    pub orders: Vec<Order>,
}

/// Order fills response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrdersFillsResponse {
    #[serde(default)]
    pub fills: Vec<OrderFill>,
}

/// Quotes response. Note: API uses PascalCase `Quotes`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotesResponse {
    #[serde(rename = "Quotes", default)]
    pub quotes: Vec<QuoteFull>,
}

/// Depth response. Note: API uses PascalCase `Depths`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthResponse {
    #[serde(rename = "Depths", default)]
    pub depths: Vec<Depth>,
}

/// Trades response. Note: API field is `traders` (API typo preserved).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradesResponse {
    #[serde(default)]
    pub traders: Vec<Trade>,
}

/// Security definitions response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityDefinitionsResponse {
    #[serde(rename = "securityDefinitions", default)]
    pub security_definitions: Vec<SecurityDefinition>,
}

/// Security margin and value response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityMarginAndValueResponse {
    #[serde(rename = "securityMarginAndValues", default)]
    pub security_margin_and_values: Vec<SecurityMarginAndValue>,
}

/// Security status response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityStatusResponse {
    #[serde(rename = "securityStatuses", default)]
    pub security_statuses: Vec<SecurityStatus>,
}

/// Symbols search response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolsResponse {
    #[serde(default)]
    pub symbols: Vec<SymbolInfo>,
}

/// Exchange sources response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExchangeSourcesResponse {
    #[serde(default)]
    pub exchanges: Vec<String>,
}

/// Market complexes response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplexesResponse {
    #[serde(rename = "marketComplexes", default)]
    pub market_complexes: Vec<ComplexGroups>,
}

/// Complex groups response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplexGroupsResponse {
    #[serde(rename = "symbolGroups", default)]
    pub symbol_groups: Vec<ComplexGroupInfo>,
}

/// Futures search response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolFuturesResponse {
    #[serde(default)]
    pub symbols: Vec<FutureInfo>,
}

/// Options groups response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolOptionsResponse {
    #[serde(default)]
    pub groups: Vec<String>,
    #[serde(rename = "optionGroups", default)]
    pub option_groups: Vec<OptionGroupInfo>,
}

/// Options search response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolSearchOptionsResponse {
    #[serde(rename = "symbolOptions", default)]
    pub symbol_options: Vec<Symbol>,
}

/// Option spreads response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SymbolOptionSpreadsResponse {
    #[serde(rename = "symbolSpreads", default)]
    pub symbol_spreads: Vec<Spread>,
}

/// Strategy ID response. Note: API uses PascalCase.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StrategyIdResponse {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Minimum")]
    pub minimum: i64,
    #[serde(rename = "Maximum")]
    pub maximum: i64,
}

/// Trader info response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraderInfoResponse {
    #[serde(flatten)]
    pub info: TraderInfo,
}

/// User info response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfoResponse {
    #[serde(flatten)]
    pub info: UserInfo,
}

// --- Simulation responses ---

/// Simulated trader create response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedTraderCreateResponse {
    #[serde(rename = "TraderId")]
    pub trader_id: String,
}

/// Simulated account add response.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SimulatedTraderAddAccountResponse {
    #[serde(rename = "AccountId")]
    pub account_id: String,
}

/// Cash report entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CashReportEntry {
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(rename = "entryDate", default)]
    pub entry_date: Option<i64>,
    #[serde(rename = "availableDate", default)]
    pub available_date: Option<i64>,
}

/// Simulated account cash report response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SimulatedAccountCashReportResponse {
    #[serde(rename = "AccountId", default)]
    pub account_id: Option<String>,
    #[serde(rename = "CashReport", default)]
    pub cash_report: Vec<CashReportEntry>,
}

/// Generic success response for simulation endpoints.
pub type SimulationSuccessResponse = Response;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_response_deserialize() {
        let json = r#"{"status":"OK","message":"Authenticated","token":"abc123"}"#;
        let r: AuthorizationResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.token.as_deref(), Some("abc123"));
    }

    #[test]
    fn quotes_response_pascal_case() {
        let json = r#"{"Quotes":[{"s":"XCME:ES.U16","l":4500.0}]}"#;
        let r: QuotesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.quotes.len(), 1);
    }

    #[test]
    fn trades_response_typo_preserved() {
        let json = r#"{"traders":[{"symbol":"XCME:ES.U16","price":4500.0}]}"#;
        let r: TradesResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.traders.len(), 1);
    }

    #[test]
    fn strategy_id_pascal_case() {
        let json = r#"{"Id":12345,"Minimum":10000,"Maximum":20000}"#;
        let r: StrategyIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.id, 12345);
    }

    #[test]
    fn order_base_response() {
        let json = r#"{"orderId":"ORD001","strategyId":100}"#;
        let r: OrderBaseResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.order_id.as_deref(), Some("ORD001"));
        assert_eq!(r.strategy_id, Some(100));
    }
}
