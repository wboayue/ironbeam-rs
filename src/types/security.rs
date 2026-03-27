use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{
    option_timestamp_ms, ExchangeStrategyType, OptionExpirationType, OptionType, RegCodeType,
    SecurityStatusType, SecurityType, Side, Symbol,
};

/// Security definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityDefinition {
    /// Exchange-qualified symbol.
    #[serde(rename = "exchSym")]
    pub exch_sym: Symbol,

    /// Exchange source.
    #[serde(rename = "exchangeSource", default)]
    pub exchange_source: Option<String>,

    /// Activation time.
    #[serde(rename = "activationTime", default, with = "option_timestamp_ms")]
    pub activation_time: Option<OffsetDateTime>,

    /// Expiration time.
    #[serde(rename = "expirationTime", default, with = "option_timestamp_ms")]
    pub expiration_time: Option<OffsetDateTime>,

    /// Market complex.
    #[serde(rename = "marketComplex", default)]
    pub market_complex: Option<String>,

    /// Market group.
    #[serde(rename = "marketGroup", default)]
    pub market_group: Option<String>,

    /// Market symbol.
    #[serde(rename = "marketSymbol", default)]
    pub market_symbol: Option<String>,

    /// CFI code.
    #[serde(rename = "cfiCode", default)]
    pub cfi_code: Option<String>,

    /// Whether open orders are allowed.
    #[serde(rename = "allowOpenOrders", default)]
    pub allow_open_orders: Option<bool>,

    /// Maturity month (1-12).
    #[serde(rename = "maturityMonth", default)]
    pub maturity_month: Option<i32>,

    /// Maturity year (2000-2100).
    #[serde(rename = "maturityYear", default)]
    pub maturity_year: Option<i32>,

    /// Product description.
    #[serde(rename = "productDescription", default)]
    pub product_description: Option<String>,

    // Note: API typo preserved for compatibility.
    /// Whether the security is user-defined.
    #[serde(rename = "userDefinded", default)]
    pub user_defined: Option<bool>,

    // Note: API typo preserved for compatibility.
    /// Whether intraday is defined.
    #[serde(rename = "intradayDefinded", default)]
    pub intraday_defined: Option<bool>,

    /// Option type.
    #[serde(rename = "optionType", default)]
    pub option_type: Option<OptionType>,

    /// Option expiration type.
    #[serde(rename = "optionExpirationType", default)]
    pub option_expiration_type: Option<OptionExpirationType>,

    /// Strike price.
    #[serde(rename = "strikePrice", default)]
    pub strike_price: Option<f64>,

    /// Underlying symbol.
    #[serde(rename = "underlyingSymbol", default)]
    pub underlying_symbol: Option<Symbol>,

    /// Variable tick table code.
    #[serde(rename = "variableTickTableCode", default)]
    pub variable_tick_table_code: Option<i32>,

    /// Exchange strategy type.
    #[serde(rename = "exchangeStrategyType", default)]
    pub exchange_strategy_type: Option<ExchangeStrategyType>,

    /// Security type.
    #[serde(rename = "securityType", default)]
    pub security_type: Option<SecurityType>,

    /// Security identifier.
    #[serde(rename = "securityId", default)]
    pub security_id: Option<String>,

    /// Spread legs.
    #[serde(default)]
    pub legs: Option<Vec<SecurityDefinitionLeg>>,

    /// Depth levels available.
    #[serde(rename = "depthLevels", default)]
    pub depth_levels: Option<i32>,

    /// Main fraction.
    #[serde(rename = "mainFraction", default)]
    pub main_fraction: Option<f64>,

    /// Sub fraction.
    #[serde(rename = "subFraction", default)]
    pub sub_fraction: Option<f64>,

    /// Scale factor.
    #[serde(default)]
    pub scale: Option<i32>,

    /// Minimum price increment.
    #[serde(rename = "minPriceIncrement", default)]
    pub min_price_increment: Option<f64>,

    /// Minimum price increment value.
    #[serde(rename = "minPriceIncrementValue", default)]
    pub min_price_increment_value: Option<f64>,

    /// Regulatory code.
    #[serde(rename = "regCode", default)]
    pub reg_code: Option<RegCodeType>,

    /// Currency code.
    #[serde(rename = "currencyCode", default)]
    pub currency_code: Option<String>,

    /// Display factor.
    #[serde(rename = "displayFactor", default)]
    pub display_factor: Option<f64>,

    /// Whether trading is allowed.
    #[serde(rename = "allowTrading", default)]
    pub allow_trading: Option<bool>,

    /// Scaling factor for screen display.
    #[serde(rename = "scalingFactorScreen", default)]
    pub scaling_factor_screen: Option<f64>,

    /// Exchange symbol.
    #[serde(rename = "exchangeSymbol", default)]
    pub exchange_symbol: Option<Symbol>,

    /// Creation date.
    #[serde(rename = "creationDate", default, with = "option_timestamp_ms")]
    pub creation_date: Option<OffsetDateTime>,
}

/// Leg of a spread security definition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityDefinitionLeg {
    /// Leg symbol.
    pub symbol: Symbol,

    /// Ratio.
    #[serde(default)]
    pub ratio: Option<i32>,

    /// Side (BID or ASK).
    #[serde(default)]
    pub side: Option<Side>,

    /// Security identifier.
    #[serde(rename = "securityId", default)]
    pub security_id: Option<String>,

    /// Exchange.
    #[serde(default)]
    pub exchange: Option<String>,

    /// Leg exchange symbol.
    #[serde(rename = "legExchangeSymbol", default)]
    pub leg_exchange_symbol: Option<Symbol>,
}

/// Security margin and value information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityMarginAndValue {
    /// Exchange-qualified symbol.
    #[serde(rename = "exchSym")]
    pub exch_sym: Symbol,

    /// Current price.
    #[serde(rename = "currentPrice", default)]
    pub current_price: Option<f64>,

    /// Current time.
    #[serde(rename = "currentTime", default, with = "option_timestamp_ms")]
    pub current_time: Option<OffsetDateTime>,

    /// Current value.
    #[serde(rename = "currentValue", default)]
    pub current_value: Option<f64>,

    /// Initial margin for long positions.
    #[serde(rename = "initialMarginLong", default)]
    pub initial_margin_long: Option<f64>,

    // Note: API typo preserved for compatibility ("Magin" instead of "Margin").
    /// Initial margin for short positions.
    #[serde(rename = "initialMaginShort", default)]
    pub initial_margin_short: Option<f64>,

    /// Maintenance margin for long positions.
    #[serde(rename = "maintMarginLong", default)]
    pub maint_margin_long: Option<f64>,

    /// Maintenance margin for short positions.
    #[serde(rename = "maintMarginShort", default)]
    pub maint_margin_short: Option<f64>,

    /// SPAN settle price.
    #[serde(rename = "spanSettlePrice", default)]
    pub span_settle_price: Option<f64>,

    /// SPAN settle value.
    #[serde(rename = "spanSettleValue", default)]
    pub span_settle_value: Option<f64>,

    /// Margin schedule details.
    #[serde(rename = "marginScheduleDetails", default)]
    pub margin_schedule_details: Option<Vec<MarginScheduleDetail>>,
}

/// Margin schedule detail entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarginScheduleDetail {
    /// Schedule start time.
    #[serde(rename = "startTime", default, with = "option_timestamp_ms")]
    pub start_time: Option<OffsetDateTime>,

    /// Schedule end time.
    #[serde(rename = "endTime", default, with = "option_timestamp_ms")]
    pub end_time: Option<OffsetDateTime>,

    /// Margin amount.
    #[serde(default)]
    pub margin: Option<f64>,
}

/// Security trading status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecurityStatus {
    /// Exchange-qualified symbol.
    #[serde(rename = "exchSym")]
    pub exch_sym: Symbol,

    /// Trading status.
    #[serde(default)]
    pub status: Option<SecurityStatusType>,

    /// Status value (raw integer).
    #[serde(rename = "statusValue", default)]
    pub status_value: Option<i32>,

    /// Status timestamp.
    #[serde(rename = "dateTime", default, with = "option_timestamp_ms")]
    pub date_time: Option<OffsetDateTime>,

    /// Trade date.
    #[serde(rename = "tradeDate", default, with = "option_timestamp_ms")]
    pub trade_date: Option<OffsetDateTime>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn security_definition_with_typos() {
        let json = r#"{
            "exchSym": "XCME:ES.U16",
            "userDefinded": true,
            "intradayDefinded": false,
            "securityType": "FUT"
        }"#;
        let sd: SecurityDefinition = serde_json::from_str(json).unwrap();
        assert_eq!(sd.exch_sym, "XCME:ES.U16");
        assert_eq!(sd.user_defined, Some(true));
        assert_eq!(sd.intraday_defined, Some(false));
        assert_eq!(sd.security_type, Some(SecurityType::Fut));
    }

    #[test]
    fn security_margin_typo_preserved() {
        let json = r#"{
            "exchSym": "XCME:ES.U16",
            "initialMarginLong": 5000.0,
            "initialMaginShort": 5000.0
        }"#;
        let sm: SecurityMarginAndValue = serde_json::from_str(json).unwrap();
        assert_eq!(sm.initial_margin_long, Some(5000.0));
        assert_eq!(sm.initial_margin_short, Some(5000.0));
    }

    #[test]
    fn security_status_deserialize() {
        let json = r#"{
            "exchSym": "XCME:ES.U16",
            "status": 17,
            "statusValue": 17
        }"#;
        let ss: SecurityStatus = serde_json::from_str(json).unwrap();
        assert_eq!(ss.status, Some(SecurityStatusType::Open));
    }
}
