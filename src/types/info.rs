use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{Symbol, option_timestamp_ms};

/// Trader information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraderInfo {
    /// Associated account identifiers.
    #[serde(default)]
    pub accounts: Vec<String>,

    /// Whether this is a live (not demo) account.
    #[serde(rename = "isLive")]
    pub is_live: bool,

    /// Trader identifier.
    #[serde(rename = "traderId")]
    pub trader_id: String,
}

/// User general information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfo {
    /// Account category.
    #[serde(rename = "accountCategory", default)]
    pub account_category: Option<i32>,

    /// Account title.
    #[serde(rename = "accountTitle", default)]
    pub account_title: Option<String>,

    /// Primary email.
    #[serde(rename = "emailAddress1", default)]
    pub email_address_1: Option<String>,

    /// Secondary email.
    #[serde(rename = "emailAddress2", default)]
    pub email_address_2: Option<String>,

    /// Group name.
    #[serde(default)]
    pub group: Option<String>,

    /// Whether this is a clearing account.
    #[serde(rename = "isClearingAccount", default)]
    pub is_clearing_account: Option<bool>,

    /// Primary phone.
    #[serde(default)]
    pub phone1: Option<String>,

    /// Secondary phone.
    #[serde(default)]
    pub phone2: Option<String>,

    /// Sub-group name.
    #[serde(rename = "subGroup", default)]
    pub sub_group: Option<String>,

    /// Associated account identifiers.
    #[serde(default)]
    pub accounts: Vec<String>,
}

/// Symbol information from search.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SymbolInfo {
    /// Symbol.
    pub symbol: Symbol,

    /// Currency.
    #[serde(default)]
    pub currency: Option<String>,

    /// Description.
    #[serde(default)]
    pub description: Option<String>,

    /// Whether quotes are available.
    #[serde(rename = "hasQuotes", default)]
    pub has_quotes: Option<bool>,

    /// Pip value.
    #[serde(rename = "pipValue", default)]
    pub pip_value: Option<f64>,

    /// Pip size.
    #[serde(rename = "pipSize", default)]
    pub pip_size: Option<f64>,

    /// Minimum tick size.
    #[serde(rename = "minTick", default)]
    pub min_tick: Option<f64>,

    /// Quantity step.
    #[serde(rename = "qtyStep", default)]
    pub qty_step: Option<f64>,

    /// Symbol type (e.g. "Future", "Forex", "CFD").
    #[serde(rename = "symbolType", default)]
    pub symbol_type: Option<String>,
}

/// Future contract information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FutureInfo {
    /// Symbol.
    pub symbol: Symbol,

    /// Maturity month.
    #[serde(rename = "maturityMonth", default)]
    pub maturity_month: Option<String>,

    /// Maturity year.
    #[serde(rename = "maturityYear", default)]
    pub maturity_year: Option<i32>,

    /// Description.
    #[serde(default)]
    pub description: Option<String>,
}

/// Market complex grouping.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplexGroups {
    /// Complex name.
    #[serde(default)]
    pub name: Option<String>,

    /// Sub-groups.
    #[serde(default)]
    pub groups: Vec<ComplexGroupInfo>,
}

/// Market complex group information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComplexGroupInfo {
    /// Group identifier.
    #[serde(default)]
    pub group: Option<String>,

    /// Display name.
    #[serde(default)]
    pub name: Option<String>,
}

/// Option group information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionGroupInfo {
    /// Group identifier.
    #[serde(default)]
    pub group: Option<String>,

    /// Expiration timestamp.
    #[serde(default, with = "option_timestamp_ms")]
    pub expiration: Option<OffsetDateTime>,

    /// Description.
    #[serde(default)]
    pub description: Option<String>,
}

/// Domain result for option groups query. Separates transport (serde) from public API.
#[derive(Debug, Clone, PartialEq)]
pub struct SymbolOptionsResult {
    /// Group names.
    pub groups: Vec<String>,
    /// Option group details.
    pub option_groups: Vec<OptionGroupInfo>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trader_info_deserialize() {
        let json = r#"{"accounts":["ACC001","ACC002"],"isLive":true,"traderId":"T001"}"#;
        let t: TraderInfo = serde_json::from_str(json).unwrap();
        assert_eq!(t.accounts.len(), 2);
        assert!(t.is_live);
    }

    #[test]
    fn symbol_info_deserialize() {
        let json = r#"{
            "symbol": "XCME:ES.U16",
            "currency": "USD",
            "symbolType": "Future",
            "pipValue": 12.5
        }"#;
        let s: SymbolInfo = serde_json::from_str(json).unwrap();
        assert_eq!(s.symbol, "XCME:ES.U16");
        assert_eq!(s.symbol_type.as_deref(), Some("Future"));
    }
}
