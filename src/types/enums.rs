use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// Serde visitor that accepts both integer and string representations.
///
/// Used by [`dual_format_enum!`] to keep the per-enum macro expansion small.
struct DualFormatVisitor<T: Copy + 'static> {
    type_name: &'static str,
    int_map: &'static [(u64, T)],
    str_map: &'static [(&'static str, T)],
}

impl<'de, T: Copy> serde::de::Visitor<'de> for DualFormatVisitor<T> {
    type Value = T;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "a string or integer {}", self.type_name)
    }

    fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<T, E> {
        for &(k, val) in self.int_map {
            if k == v {
                return Ok(val);
            }
        }
        Err(E::custom(format!("unknown {} integer: {}", self.type_name, v)))
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<T, E> {
        let v = u64::try_from(v).map_err(|_| {
            E::custom(format!("negative {}: {}", self.type_name, v))
        })?;
        self.visit_u64(v)
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<T, E> {
        for &(k, val) in self.str_map {
            if k == v {
                return Ok(val);
            }
        }
        Err(E::custom(format!("unknown {} string: {}", self.type_name, v)))
    }
}

/// Generate a dual-format enum that deserializes from both strings (REST) and
/// integers (streaming). Serialization always uses the string form.
macro_rules! dual_format_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $( $(#[$vmeta:meta])* $Variant:ident = ($int:expr, $str:expr) ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
        $vis enum $Name {
            $( $(#[$vmeta])* #[serde(rename = $str)] $Variant ),+
        }

        impl<'de> Deserialize<'de> for $Name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                static INT_MAP: &[(u64, $Name)] = &[$( ($int, $Name::$Variant), )+];
                static STR_MAP: &[(&str, $Name)] = &[$( ($str, $Name::$Variant), )+];

                deserializer.deserialize_any(DualFormatVisitor {
                    type_name: stringify!($Name),
                    int_map: INT_MAP,
                    str_map: STR_MAP,
                })
            }
        }
    };
}

dual_format_enum! {
    /// API response status.
    ///
    /// REST sends string values (`"OK"`), streaming sends integers (`0`).
    pub enum ResponseStatus {
        Ok = (0, "OK"),
        Error = (1, "ERROR"),
        Warning = (2, "WARNING"),
        Info = (3, "INFO"),
        Fatal = (4, "FATAL"),
        Unknown = (5, "UNKNOWN"),
    }
}

dual_format_enum! {
    /// Account balance type.
    ///
    /// REST sends string values (`"CURRENT_OPEN"`), streaming sends integers (`0`).
    pub enum BalanceType {
        CurrentOpen = (0, "CURRENT_OPEN"),
        StartOfDay = (1, "START_OF_DAY"),
    }
}

/// Order side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
    Invalid,
}

/// Order type. Wire format uses string-encoded numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    /// Invalid / unset.
    #[serde(rename = "")]
    Invalid,
    /// Market order.
    #[serde(rename = "1")]
    Market,
    /// Limit order.
    #[serde(rename = "2")]
    Limit,
    /// Stop order.
    #[serde(rename = "3")]
    Stop,
    /// Stop-limit order.
    #[serde(rename = "4")]
    StopLimit,
}

/// Order duration. Wire format uses string-encoded numbers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DurationType {
    /// Invalid / unset.
    #[serde(rename = "")]
    Invalid,
    /// Day order.
    #[serde(rename = "0")]
    Day,
    /// Good till cancel.
    #[serde(rename = "1")]
    GoodTillCancel,
}

/// Order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatusType {
    Any,
    Invalid,
    Submitted,
    New,
    PartiallyFilled,
    Filled,
    DoneForDay,
    Cancelled,
    Replaced,
    PendingCancel,
    Stopped,
    Rejected,
    Suspended,
    PendingNew,
    Calculated,
    Expired,
    AcceptedForBidding,
    PendingReplace,
    CancelRejected,
    OrderNotFound,
    QueuedNew,
    QueuedCancel,
    Complete,
}

/// Position side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PositionSide {
    Long,
    Short,
}

/// Security type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SecurityType {
    Invalid,
    Fut,
    Opt,
    Mixed,
}

/// Option type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionType {
    Invalid,
    Put,
    Call,
}

/// Option expiration type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OptionExpirationType {
    Invalid,
    American,
    European,
}

/// Market side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Side {
    Bid,
    Ask,
}

dual_format_enum! {
    /// Depth side.
    ///
    /// REST/strings use `"B"` (bid) / `"A"` (ask), streaming sends integers (`0` / `1`).
    pub enum DepthSide {
        Bid = (0, "B"),
        Ask = (1, "A"),
    }
}

dual_format_enum! {
    /// Regulatory code type.
    ///
    /// REST sends string values (`"COMBINED"`), streaming sends integers (`1`).
    pub enum RegCodeType {
        Invalid = (0, "INVALID"),
        Combined = (1, "COMBINED"),
        Regulated = (2, "REGULATED"),
        NonSecured = (3, "NON_SECURED"),
        Secured = (4, "SECURED"),
    }
}

/// Tick direction (string enum).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TickDirection {
    Invalid,
    Plus,
    Minus,
    Same,
}

/// Bar type for indicator subscriptions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BarType {
    Daily,
    Hour,
    Minute,
    Tick,
}

/// System-priced trade indicator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SystemPricedTrade {
    Invalid,
    System,
    Crack,
}

/// Trade investigation status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InvestigationStatus {
    Invalid,
    Investigating,
    Completed,
}

/// Block trade type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BlockTrade {
    Invalid,
    Normal,
    #[serde(rename = "EFP")]
    Efp,
    #[serde(rename = "EFS")]
    Efs,
    OffExchange,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "CCX")]
    Ccx,
    #[serde(rename = "EFR")]
    Efr,
}

/// Exchange strategy type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ExchangeStrategyType {
    NONE,
    SP,
    FX,
    RT,
    EQ,
    BF,
    CF,
    FS,
    IS,
    PK,
    MP,
    PB,
    DF,
    PS,
    C1,
    FB,
    BS,
    SA,
    SB,
    WS,
    XS,
    DI,
    IV,
    EC,
    SI,
    SD,
    MS,
    #[serde(rename = "3W")]
    ThreeWay,
    #[serde(rename = "3C")]
    ThreeConv,
    #[serde(rename = "3P")]
    ThreePack,
    BX,
    BO,
    XT,
    CC,
    CO,
    DB,
    HO,
    DG,
    HS,
    IC,
    #[serde(rename = "12")]
    OneTwo,
    #[serde(rename = "13")]
    OneThree,
    #[serde(rename = "23")]
    TwoThree,
    RR,
    SS,
    ST,
    SG,
    SR,
    VT,
    JR,
    IB,
    GT,
    GN,
    DN,
}

/// Security trading status (integer-encoded).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SecurityStatusType {
    TradingHalt = 2,
    Closed = 4,
    PriceIndication = 15,
    Open = 17,
    Close = 18,
    Unknown = 20,
    PreOpen = 21,
    OpeningRotation = 22,
    PreCross = 24,
    Cross = 25,
    NoCancel = 26,
    Expired = 30,
    PreClose = 31,
    NoChange = 103,
    PostClose = 126,
}

/// Aggressor side (integer-encoded).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum AggressorSideType {
    Invalid = 0,
    Buy = 1,
    Sell = 2,
}

/// Tick direction (integer-encoded, used in streaming).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum TickDirectionType {
    Plus = 0,
    Same = 1,
    Minus = 2,
    Invalid = 255,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_type_round_trip() {
        assert_eq!(serde_json::to_string(&OrderType::Market).unwrap(), "\"1\"");
        assert_eq!(serde_json::to_string(&OrderType::Limit).unwrap(), "\"2\"");
        assert_eq!(serde_json::to_string(&OrderType::Stop).unwrap(), "\"3\"");
        assert_eq!(
            serde_json::to_string(&OrderType::StopLimit).unwrap(),
            "\"4\""
        );
        assert_eq!(serde_json::to_string(&OrderType::Invalid).unwrap(), "\"\"");

        assert_eq!(
            serde_json::from_str::<OrderType>("\"1\"").unwrap(),
            OrderType::Market
        );
        assert_eq!(
            serde_json::from_str::<OrderType>("\"\"").unwrap(),
            OrderType::Invalid
        );
    }

    #[test]
    fn duration_type_round_trip() {
        assert_eq!(serde_json::to_string(&DurationType::Day).unwrap(), "\"0\"");
        assert_eq!(
            serde_json::to_string(&DurationType::GoodTillCancel).unwrap(),
            "\"1\""
        );
        assert_eq!(
            serde_json::from_str::<DurationType>("\"0\"").unwrap(),
            DurationType::Day
        );
        assert_eq!(
            serde_json::from_str::<DurationType>("\"\"").unwrap(),
            DurationType::Invalid
        );
    }

    #[test]
    fn security_status_type_round_trip() {
        assert_eq!(
            serde_json::to_string(&SecurityStatusType::Open).unwrap(),
            "17"
        );
        assert_eq!(
            serde_json::from_str::<SecurityStatusType>("17").unwrap(),
            SecurityStatusType::Open
        );
        assert_eq!(
            serde_json::from_str::<SecurityStatusType>("126").unwrap(),
            SecurityStatusType::PostClose
        );
    }

    #[test]
    fn aggressor_side_round_trip() {
        assert_eq!(serde_json::to_string(&AggressorSideType::Buy).unwrap(), "1");
        assert_eq!(
            serde_json::from_str::<AggressorSideType>("2").unwrap(),
            AggressorSideType::Sell
        );
    }

    #[test]
    fn tick_direction_type_round_trip() {
        assert_eq!(
            serde_json::to_string(&TickDirectionType::Invalid).unwrap(),
            "255"
        );
        assert_eq!(
            serde_json::from_str::<TickDirectionType>("0").unwrap(),
            TickDirectionType::Plus
        );
    }

    #[test]
    fn order_status_round_trip() {
        assert_eq!(
            serde_json::to_string(&OrderStatusType::PartiallyFilled).unwrap(),
            "\"PARTIALLY_FILLED\""
        );
        assert_eq!(
            serde_json::from_str::<OrderStatusType>("\"PENDING_CANCEL\"").unwrap(),
            OrderStatusType::PendingCancel
        );
    }

    #[test]
    fn exchange_strategy_digit_prefix() {
        assert_eq!(
            serde_json::to_string(&ExchangeStrategyType::ThreeWay).unwrap(),
            "\"3W\""
        );
        assert_eq!(
            serde_json::from_str::<ExchangeStrategyType>("\"3W\"").unwrap(),
            ExchangeStrategyType::ThreeWay
        );
        assert_eq!(
            serde_json::to_string(&ExchangeStrategyType::OneTwo).unwrap(),
            "\"12\""
        );
    }

    #[test]
    fn block_trade_round_trip() {
        assert_eq!(serde_json::to_string(&BlockTrade::Efp).unwrap(), "\"EFP\"");
        assert_eq!(
            serde_json::to_string(&BlockTrade::OffExchange).unwrap(),
            "\"OFF_EXCHANGE\""
        );
    }

    #[test]
    fn response_status_round_trip() {
        assert_eq!(
            serde_json::to_string(&ResponseStatus::Ok).unwrap(),
            "\"OK\""
        );
        assert_eq!(
            serde_json::from_str::<ResponseStatus>("\"ERROR\"").unwrap(),
            ResponseStatus::Error
        );
    }

    #[test]
    fn response_status_from_integer() {
        assert_eq!(
            serde_json::from_str::<ResponseStatus>("0").unwrap(),
            ResponseStatus::Ok
        );
        assert_eq!(
            serde_json::from_str::<ResponseStatus>("1").unwrap(),
            ResponseStatus::Error
        );
    }

    #[test]
    fn balance_type_round_trip() {
        assert_eq!(
            serde_json::to_string(&BalanceType::CurrentOpen).unwrap(),
            "\"CURRENT_OPEN\""
        );
        assert_eq!(
            serde_json::from_str::<BalanceType>("\"START_OF_DAY\"").unwrap(),
            BalanceType::StartOfDay
        );
    }

    #[test]
    fn balance_type_from_integer() {
        assert_eq!(
            serde_json::from_str::<BalanceType>("0").unwrap(),
            BalanceType::CurrentOpen
        );
        assert_eq!(
            serde_json::from_str::<BalanceType>("1").unwrap(),
            BalanceType::StartOfDay
        );
    }

    #[test]
    fn balance_type_negative_integer_rejected() {
        assert!(serde_json::from_str::<BalanceType>("-1").is_err());
    }

    #[test]
    fn balance_type_unknown_integer_rejected() {
        assert!(serde_json::from_str::<BalanceType>("99").is_err());
    }

    #[test]
    fn reg_code_type_round_trip() {
        assert_eq!(
            serde_json::to_string(&RegCodeType::Combined).unwrap(),
            "\"COMBINED\""
        );
        assert_eq!(
            serde_json::from_str::<RegCodeType>("\"NON_SECURED\"").unwrap(),
            RegCodeType::NonSecured
        );
    }

    #[test]
    fn reg_code_type_from_integer() {
        assert_eq!(
            serde_json::from_str::<RegCodeType>("0").unwrap(),
            RegCodeType::Invalid
        );
        assert_eq!(
            serde_json::from_str::<RegCodeType>("1").unwrap(),
            RegCodeType::Combined
        );
        assert_eq!(
            serde_json::from_str::<RegCodeType>("4").unwrap(),
            RegCodeType::Secured
        );
    }

    #[test]
    fn reg_code_type_negative_integer_rejected() {
        assert!(serde_json::from_str::<RegCodeType>("-1").is_err());
    }

    #[test]
    fn reg_code_type_unknown_integer_rejected() {
        assert!(serde_json::from_str::<RegCodeType>("99").is_err());
    }

    #[test]
    fn depth_side_round_trip() {
        assert_eq!(serde_json::to_string(&DepthSide::Bid).unwrap(), "\"B\"");
        assert_eq!(
            serde_json::from_str::<DepthSide>("\"A\"").unwrap(),
            DepthSide::Ask
        );
    }

    #[test]
    fn depth_side_from_integer() {
        assert_eq!(serde_json::from_str::<DepthSide>("0").unwrap(), DepthSide::Bid);
        assert_eq!(serde_json::from_str::<DepthSide>("1").unwrap(), DepthSide::Ask);
    }

    #[test]
    fn depth_side_negative_integer_rejected() {
        assert!(serde_json::from_str::<DepthSide>("-1").is_err());
    }

    #[test]
    fn depth_side_unknown_integer_rejected() {
        assert!(serde_json::from_str::<DepthSide>("99").is_err());
    }
}
