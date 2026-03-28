use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// API response status.
///
/// REST sends string values (`"OK"`), streaming sends integers (`0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResponseStatus {
    Ok,
    Error,
    Warning,
    Info,
    Fatal,
    Unknown,
}

impl<'de> Deserialize<'de> for ResponseStatus {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct ResponseStatusVisitor;

        impl<'de> serde::de::Visitor<'de> for ResponseStatusVisitor {
            type Value = ResponseStatus;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or integer response status")
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<ResponseStatus, E> {
                match v {
                    0 => Ok(ResponseStatus::Ok),
                    1 => Ok(ResponseStatus::Error),
                    2 => Ok(ResponseStatus::Warning),
                    3 => Ok(ResponseStatus::Info),
                    4 => Ok(ResponseStatus::Fatal),
                    5 => Ok(ResponseStatus::Unknown),
                    _ => Err(E::custom(format!("unknown response status integer: {v}"))),
                }
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<ResponseStatus, E> {
                let v = u64::try_from(v).map_err(|_| E::custom(format!("negative response status: {v}")))?;
                self.visit_u64(v)
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<ResponseStatus, E> {
                match v {
                    "OK" => Ok(ResponseStatus::Ok),
                    "ERROR" => Ok(ResponseStatus::Error),
                    "WARNING" => Ok(ResponseStatus::Warning),
                    "INFO" => Ok(ResponseStatus::Info),
                    "FATAL" => Ok(ResponseStatus::Fatal),
                    "UNKNOWN" => Ok(ResponseStatus::Unknown),
                    _ => Err(E::custom(format!("unknown response status string: {v}"))),
                }
            }
        }

        deserializer.deserialize_any(ResponseStatusVisitor)
    }
}

/// Account balance type.
///
/// REST sends string values (`"CURRENT_OPEN"`), streaming sends integers (`0`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BalanceType {
    CurrentOpen,
    StartOfDay,
}

impl<'de> Deserialize<'de> for BalanceType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct BalanceTypeVisitor;

        impl<'de> serde::de::Visitor<'de> for BalanceTypeVisitor {
            type Value = BalanceType;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or integer balance type")
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<BalanceType, E> {
                match v {
                    0 => Ok(BalanceType::CurrentOpen),
                    1 => Ok(BalanceType::StartOfDay),
                    _ => Err(E::custom(format!("unknown balance type integer: {v}"))),
                }
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<BalanceType, E> {
                let v = u64::try_from(v).map_err(|_| E::custom(format!("negative balance type: {v}")))?;
                self.visit_u64(v)
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<BalanceType, E> {
                match v {
                    "CURRENT_OPEN" => Ok(BalanceType::CurrentOpen),
                    "START_OF_DAY" => Ok(BalanceType::StartOfDay),
                    _ => Err(E::custom(format!("unknown balance type string: {v}"))),
                }
            }
        }

        deserializer.deserialize_any(BalanceTypeVisitor)
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

/// Market side (short form).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SideShort {
    B,
    A,
}

/// Regulatory code type.
///
/// REST sends string values (`"COMBINED"`), streaming sends integers (`1`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RegCodeType {
    Invalid,
    Combined,
    Regulated,
    NonSecured,
    Secured,
}

impl<'de> Deserialize<'de> for RegCodeType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct RegCodeVisitor;

        impl<'de> serde::de::Visitor<'de> for RegCodeVisitor {
            type Value = RegCodeType;

            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("a string or integer regulatory code")
            }

            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<RegCodeType, E> {
                match v {
                    0 => Ok(RegCodeType::Invalid),
                    1 => Ok(RegCodeType::Combined),
                    2 => Ok(RegCodeType::Regulated),
                    3 => Ok(RegCodeType::NonSecured),
                    4 => Ok(RegCodeType::Secured),
                    _ => Err(E::custom(format!("unknown reg code integer: {v}"))),
                }
            }

            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<RegCodeType, E> {
                let v = u64::try_from(v).map_err(|_| E::custom(format!("negative reg code: {v}")))?;
                self.visit_u64(v)
            }

            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<RegCodeType, E> {
                match v {
                    "INVALID" => Ok(RegCodeType::Invalid),
                    "COMBINED" => Ok(RegCodeType::Combined),
                    "REGULATED" => Ok(RegCodeType::Regulated),
                    "NON_SECURED" => Ok(RegCodeType::NonSecured),
                    "SECURED" => Ok(RegCodeType::Secured),
                    _ => Err(E::custom(format!("unknown reg code string: {v}"))),
                }
            }
        }

        deserializer.deserialize_any(RegCodeVisitor)
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
}
