use serde::{Deserialize, Serialize};

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
        Err(E::custom(format!(
            "unknown {} integer: {}",
            self.type_name, v
        )))
    }

    fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<T, E> {
        let v = u64::try_from(v)
            .map_err(|_| E::custom(format!("negative {}: {}", self.type_name, v)))?;
        self.visit_u64(v)
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<T, E> {
        for &(k, val) in self.str_map {
            if k == v {
                return Ok(val);
            }
        }
        Err(E::custom(format!(
            "unknown {} string: {}",
            self.type_name, v
        )))
    }
}

/// Generate a dual-format enum that deserializes from both strings (REST) and
/// integers (streaming). Serialization always uses the string form.
///
/// Two forms:
/// - `Variant = (int, "STR")` — single string mapping
/// - `Variant = (int, "STR", ["ALIAS1", "ALIAS2"])` — canonical string + aliases
macro_rules! dual_format_enum {
    // Arm with optional aliases: each variant is (int, "canonical" [, ["alias", ...]])
    (
        $(#[$meta:meta])*
        $vis:vis enum $Name:ident {
            $( $(#[$vmeta:meta])* $Variant:ident = ($int:expr, $str:expr $(, [$($alias:expr),* $(,)?])? ) ),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
        $vis enum $Name {
            $( $(#[$vmeta])* #[serde(rename = $str)] $Variant ),+
        }

        impl $Name {
            /// Wire-format string for use in query parameters.
            pub fn as_str(&self) -> &'static str {
                match self { $( Self::$Variant => $str, )+ }
            }
        }

        impl<'de> Deserialize<'de> for $Name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                static INT_MAP: &[(u64, $Name)] = &[$( ($int, $Name::$Variant), )+];
                static STR_MAP: &[(&str, $Name)] = &[
                    $( ($str, $Name::$Variant), $( $( ($alias, $Name::$Variant), )* )? )+
                ];

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

dual_format_enum! {
    /// Order type.
    ///
    /// REST sends string names (`"LIMIT"`), streaming sends string-encoded numbers (`"2"`).
    pub enum OrderType {
        /// Invalid / unset.
        Invalid = (0, "", ["INVALID"]),
        /// Market order.
        Market = (1, "1", ["MARKET"]),
        /// Limit order.
        Limit = (2, "2", ["LIMIT"]),
        /// Stop order.
        Stop = (3, "3", ["STOP"]),
        /// Stop-limit order.
        StopLimit = (4, "4", ["STOP_LIMIT"]),
    }
}

dual_format_enum! {
    /// Order duration.
    ///
    /// REST sends string names (`"DAY"`), streaming sends string-encoded numbers (`"0"`).
    pub enum DurationType {
        /// Invalid / unset.
        Invalid = (99, "", ["INVALID"]),
        /// Day order.
        Day = (0, "0", ["DAY"]),
        /// Good till cancel.
        GoodTillCancel = (1, "1", ["GOOD_TILL_CANCEL", "GTC"]),
    }
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

impl OrderStatusType {
    /// Wire-format string for use in URL path segments.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Any => "ANY",
            Self::Invalid => "INVALID",
            Self::Submitted => "SUBMITTED",
            Self::New => "NEW",
            Self::PartiallyFilled => "PARTIALLY_FILLED",
            Self::Filled => "FILLED",
            Self::DoneForDay => "DONE_FOR_DAY",
            Self::Cancelled => "CANCELLED",
            Self::Replaced => "REPLACED",
            Self::PendingCancel => "PENDING_CANCEL",
            Self::Stopped => "STOPPED",
            Self::Rejected => "REJECTED",
            Self::Suspended => "SUSPENDED",
            Self::PendingNew => "PENDING_NEW",
            Self::Calculated => "CALCULATED",
            Self::Expired => "EXPIRED",
            Self::AcceptedForBidding => "ACCEPTED_FOR_BIDDING",
            Self::PendingReplace => "PENDING_REPLACE",
            Self::CancelRejected => "CANCEL_REJECTED",
            Self::OrderNotFound => "ORDER_NOT_FOUND",
            Self::QueuedNew => "QUEUED_NEW",
            Self::QueuedCancel => "QUEUED_CANCEL",
            Self::Complete => "COMPLETE",
        }
    }
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

dual_format_enum! {
    /// System-priced trade indicator.
    ///
    /// REST sends strings (`"INVALID"`), streaming sends integers (`0`).
    pub enum SystemPricedTrade {
        Invalid = (0, "INVALID"),
        System = (1, "SYSTEM"),
        Crack = (2, "CRACK"),
    }
}

dual_format_enum! {
    /// Trade investigation status.
    ///
    /// REST sends strings (`"INVALID"`), streaming sends integers (`0`).
    pub enum InvestigationStatus {
        Invalid = (0, "INVALID"),
        Investigating = (1, "INVESTIGATING"),
        Completed = (2, "COMPLETED"),
    }
}

dual_format_enum! {
    /// Block trade type.
    ///
    /// REST sends strings (`"INVALID"`), streaming sends integers (`0`).
    pub enum BlockTrade {
        Invalid = (0, "INVALID"),
        Normal = (1, "NORMAL"),
        Efp = (2, "EFP"),
        Efs = (3, "EFS"),
        OffExchange = (4, "OFF_EXCHANGE"),
        Ng = (5, "NG"),
        Ccx = (6, "CCX"),
        Efr = (7, "EFR"),
    }
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

dual_format_enum! {
    /// Security trading status.
    ///
    /// REST sends strings (`"CLOSED"`), streaming sends integers (`4`).
    pub enum SecurityStatusType {
        TradingHalt = (2, "TRADING_HALT"),
        Closed = (4, "CLOSED"),
        PriceIndication = (15, "PRICE_INDICATION"),
        Open = (17, "OPEN"),
        Close = (18, "CLOSE"),
        Unknown = (20, "UNKNOWN"),
        PreOpen = (21, "PRE_OPEN"),
        OpeningRotation = (22, "OPENING_ROTATION"),
        PreCross = (24, "PRE_CROSS"),
        Cross = (25, "CROSS"),
        NoCancel = (26, "NO_CANCEL"),
        Expired = (30, "EXPIRED"),
        PreClose = (31, "PRE_CLOSE"),
        NoChange = (103, "NO_CHANGE"),
        PostClose = (126, "POST_CLOSE"),
    }
}

dual_format_enum! {
    /// Aggressor side.
    ///
    /// REST sends strings (`"BUY"`), streaming sends integers (`1`).
    pub enum AggressorSideType {
        Invalid = (0, "INVALID"),
        Buy = (1, "BUY"),
        Sell = (2, "SELL"),
    }
}

dual_format_enum! {
    /// Tick direction.
    ///
    /// REST sends strings (`"PLUS"`), streaming sends integers (`0`).
    pub enum TickDirectionType {
        Plus = (0, "PLUS"),
        Same = (1, "SAME"),
        Minus = (2, "MINUS"),
        ZeroMinus = (3, "ZERO_MINUS"),
        Invalid = (255, "INVALID"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Simple serde round-trip tests (string-only enums) ---

    macro_rules! test_serde_round_trip {
        ($($name:ident: $type:ty => [$(($variant:expr, $json:expr)),+ $(,)?]),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    $(
                        let json = serde_json::to_string(&$variant).unwrap();
                        assert_eq!(json, $json, "serialize {:?}", $variant);
                        let parsed: $type = serde_json::from_str($json).unwrap();
                        assert_eq!(parsed, $variant, "deserialize {}", $json);
                    )+
                }
            )+
        };
    }

    test_serde_round_trip! {
        position_side_round_trip: PositionSide => [
            (PositionSide::Long, r#""LONG""#),
            (PositionSide::Short, r#""SHORT""#),
        ],
        security_type_round_trip: SecurityType => [
            (SecurityType::Fut, r#""FUT""#),
            (SecurityType::Opt, r#""OPT""#),
        ],
        option_type_round_trip: OptionType => [
            (OptionType::Call, r#""CALL""#),
            (OptionType::Put, r#""PUT""#),
        ],
        option_expiration_type_round_trip: OptionExpirationType => [
            (OptionExpirationType::American, r#""AMERICAN""#),
            (OptionExpirationType::European, r#""EUROPEAN""#),
        ],
        side_round_trip: Side => [
            (Side::Bid, r#""BID""#),
            (Side::Ask, r#""ASK""#),
        ],
        tick_direction_round_trip: TickDirection => [
            (TickDirection::Plus, r#""PLUS""#),
            (TickDirection::Minus, r#""MINUS""#),
        ],
        bar_type_round_trip: BarType => [
            (BarType::Daily, r#""DAILY""#),
            (BarType::Minute, r#""MINUTE""#),
        ],
    }

    // --- Dual-format round-trip tests (string + integer deserialization) ---

    macro_rules! test_dual_format {
        ($($name:ident: $type:ty => {
            ser: [$(($ser_variant:expr, $ser_json:expr)),+ $(,)?],
            deser: [$(($deser_input:expr, $deser_expected:expr)),+ $(,)?]
        }),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    // Serialization
                    $(
                        let json = serde_json::to_string(&$ser_variant).unwrap();
                        assert_eq!(json, $ser_json, "serialize {:?}", $ser_variant);
                    )+
                    // Deserialization (string and/or integer inputs)
                    $(
                        let parsed: $type = serde_json::from_str($deser_input).unwrap();
                        assert_eq!(parsed, $deser_expected, "deserialize {}", $deser_input);
                    )+
                }
            )+
        };
    }

    test_dual_format! {
        order_type_round_trip: OrderType => {
            ser: [
                (OrderType::Market, "\"1\""),
                (OrderType::Limit, "\"2\""),
                (OrderType::Stop, "\"3\""),
                (OrderType::StopLimit, "\"4\""),
                (OrderType::Invalid, "\"\""),
            ],
            deser: [
                ("\"1\"", OrderType::Market),
                ("\"\"", OrderType::Invalid),
            ]
        },
        duration_type_round_trip: DurationType => {
            ser: [
                (DurationType::Day, "\"0\""),
                (DurationType::GoodTillCancel, "\"1\""),
            ],
            deser: [
                ("\"0\"", DurationType::Day),
                ("\"\"", DurationType::Invalid),
            ]
        },
        security_status_type_round_trip: SecurityStatusType => {
            ser: [
                (SecurityStatusType::Open, r#""OPEN""#),
            ],
            deser: [
                (r#""CLOSED""#, SecurityStatusType::Closed),
                ("17", SecurityStatusType::Open),
                ("126", SecurityStatusType::PostClose),
            ]
        },
        aggressor_side_round_trip: AggressorSideType => {
            ser: [
                (AggressorSideType::Buy, r#""BUY""#),
            ],
            deser: [
                ("2", AggressorSideType::Sell),
                (r#""BUY""#, AggressorSideType::Buy),
            ]
        },
        tick_direction_type_round_trip: TickDirectionType => {
            ser: [
                (TickDirectionType::Invalid, r#""INVALID""#),
            ],
            deser: [
                ("0", TickDirectionType::Plus),
                (r#""SAME""#, TickDirectionType::Same),
            ]
        },
        response_status_round_trip: ResponseStatus => {
            ser: [
                (ResponseStatus::Ok, "\"OK\""),
            ],
            deser: [
                ("\"ERROR\"", ResponseStatus::Error),
                ("0", ResponseStatus::Ok),
                ("1", ResponseStatus::Error),
            ]
        },
        balance_type_round_trip: BalanceType => {
            ser: [
                (BalanceType::CurrentOpen, "\"CURRENT_OPEN\""),
            ],
            deser: [
                ("\"START_OF_DAY\"", BalanceType::StartOfDay),
                ("0", BalanceType::CurrentOpen),
                ("1", BalanceType::StartOfDay),
            ]
        },
        reg_code_type_round_trip: RegCodeType => {
            ser: [
                (RegCodeType::Combined, "\"COMBINED\""),
            ],
            deser: [
                ("\"NON_SECURED\"", RegCodeType::NonSecured),
                ("0", RegCodeType::Invalid),
                ("1", RegCodeType::Combined),
                ("4", RegCodeType::Secured),
            ]
        },
        depth_side_round_trip: DepthSide => {
            ser: [
                (DepthSide::Bid, "\"B\""),
            ],
            deser: [
                ("\"A\"", DepthSide::Ask),
                ("0", DepthSide::Bid),
                ("1", DepthSide::Ask),
            ]
        },
        block_trade_round_trip: BlockTrade => {
            ser: [
                (BlockTrade::Efp, "\"EFP\""),
                (BlockTrade::OffExchange, "\"OFF_EXCHANGE\""),
            ],
            deser: [
                ("0", BlockTrade::Invalid),
                ("2", BlockTrade::Efp),
            ]
        },
        system_priced_trade_round_trip: SystemPricedTrade => {
            ser: [
                (SystemPricedTrade::System, "\"SYSTEM\""),
            ],
            deser: [
                ("0", SystemPricedTrade::Invalid),
                ("\"CRACK\"", SystemPricedTrade::Crack),
            ]
        },
        investigation_status_round_trip: InvestigationStatus => {
            ser: [
                (InvestigationStatus::Investigating, "\"INVESTIGATING\""),
            ],
            deser: [
                ("0", InvestigationStatus::Invalid),
                ("\"COMPLETED\"", InvestigationStatus::Completed),
            ]
        },
    }

    // --- Rejection tests (invalid inputs) ---

    macro_rules! test_deser_rejected {
        ($($name:ident: $type:ty => [$($input:expr),+ $(,)?]),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    $(
                        assert!(serde_json::from_str::<$type>($input).is_err(), "expected error for {}", $input);
                    )+
                }
            )+
        };
    }

    test_deser_rejected! {
        balance_type_rejected: BalanceType => ["-1", "99"],
        reg_code_type_rejected: RegCodeType => ["-1", "99"],
        depth_side_rejected: DepthSide => ["-1", "99"],
        unknown_string_rejected: ResponseStatus => [r#""BOGUS""#],
        unknown_order_type_rejected: OrderType => [r#""BOGUS""#],
        unknown_depth_side_rejected: DepthSide => [r#""X""#],
    }

    // --- Alias deserialization tests ---

    macro_rules! test_alias_deser {
        ($($name:ident: $type:ty => [$(($input:expr, $expected:expr)),+ $(,)?]),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    $(
                        let parsed: $type = serde_json::from_str($input).unwrap();
                        assert_eq!(parsed, $expected, "alias deserialize {}", $input);
                    )+
                }
            )+
        };
    }

    test_alias_deser! {
        order_type_alias_deserialization: OrderType => [
            (r#""MARKET""#, OrderType::Market),
            (r#""LIMIT""#, OrderType::Limit),
            (r#""STOP""#, OrderType::Stop),
            (r#""STOP_LIMIT""#, OrderType::StopLimit),
            (r#""INVALID""#, OrderType::Invalid),
        ],
        duration_type_alias_deserialization: DurationType => [
            (r#""DAY""#, DurationType::Day),
            (r#""GTC""#, DurationType::GoodTillCancel),
            (r#""GOOD_TILL_CANCEL""#, DurationType::GoodTillCancel),
            (r#""INVALID""#, DurationType::Invalid),
        ],
    }

    // --- as_str tests ---

    macro_rules! test_as_str {
        ($($name:ident: $type:ty => [$(($variant:expr, $expected:expr)),+ $(,)?]),+ $(,)?) => {
            $(
                #[test]
                fn $name() {
                    $(
                        assert_eq!($variant.as_str(), $expected, "as_str for {:?}", $variant);
                    )+
                }
            )+
        };
    }

    test_as_str! {
        order_type_as_str: OrderType => [
            (OrderType::Market, "1"),
            (OrderType::Limit, "2"),
            (OrderType::Stop, "3"),
            (OrderType::StopLimit, "4"),
            (OrderType::Invalid, ""),
        ],
        duration_type_as_str: DurationType => [
            (DurationType::Day, "0"),
            (DurationType::GoodTillCancel, "1"),
            (DurationType::Invalid, ""),
        ],
    }

    // --- Unique tests kept as-is ---

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
    fn order_status_as_str_all_variants() {
        let cases = [
            (OrderStatusType::Any, "ANY"),
            (OrderStatusType::Invalid, "INVALID"),
            (OrderStatusType::Submitted, "SUBMITTED"),
            (OrderStatusType::New, "NEW"),
            (OrderStatusType::PartiallyFilled, "PARTIALLY_FILLED"),
            (OrderStatusType::Filled, "FILLED"),
            (OrderStatusType::DoneForDay, "DONE_FOR_DAY"),
            (OrderStatusType::Cancelled, "CANCELLED"),
            (OrderStatusType::Replaced, "REPLACED"),
            (OrderStatusType::PendingCancel, "PENDING_CANCEL"),
            (OrderStatusType::Stopped, "STOPPED"),
            (OrderStatusType::Rejected, "REJECTED"),
            (OrderStatusType::Suspended, "SUSPENDED"),
            (OrderStatusType::PendingNew, "PENDING_NEW"),
            (OrderStatusType::Calculated, "CALCULATED"),
            (OrderStatusType::Expired, "EXPIRED"),
            (OrderStatusType::AcceptedForBidding, "ACCEPTED_FOR_BIDDING"),
            (OrderStatusType::PendingReplace, "PENDING_REPLACE"),
            (OrderStatusType::CancelRejected, "CANCEL_REJECTED"),
            (OrderStatusType::OrderNotFound, "ORDER_NOT_FOUND"),
            (OrderStatusType::QueuedNew, "QUEUED_NEW"),
            (OrderStatusType::QueuedCancel, "QUEUED_CANCEL"),
            (OrderStatusType::Complete, "COMPLETE"),
        ];
        for (variant, expected) in cases {
            assert_eq!(variant.as_str(), expected);
        }
    }

    #[test]
    fn dual_format_from_signed_integer() {
        // Exercises visit_i64 → visit_u64 path
        // serde_json parses "0" as u64, but we can force i64 via serde_json::Value
        let val: serde_json::Value = serde_json::from_str("0").unwrap();
        let status: ResponseStatus = serde_json::from_value(val).unwrap();
        assert_eq!(status, ResponseStatus::Ok);
    }
}
