use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{
    option_timestamp_ms, AggressorSideType, BlockTrade, InvestigationStatus, Symbol,
    SystemPricedTrade, TickDirection, TickDirectionType,
};

/// Full market quote. Used in both REST and streaming (already uses short field names).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteFull {
    /// Exchange symbol.
    pub s: Symbol,

    /// Last trade price.
    #[serde(default)]
    pub l: Option<f64>,

    /// Last trade size.
    #[serde(default)]
    pub sz: Option<i32>,

    /// Change from previous settle.
    #[serde(default)]
    pub ch: Option<f64>,

    /// Open price.
    #[serde(default)]
    pub op: Option<f64>,

    /// High price.
    #[serde(default)]
    pub hi: Option<f64>,

    /// Low price.
    #[serde(default)]
    pub lo: Option<f64>,

    /// Aggressor side.
    #[serde(default)]
    pub ags: Option<i32>,

    /// Tick direction.
    #[serde(default)]
    pub td: Option<i32>,

    /// Settlement price.
    #[serde(default)]
    pub stt: Option<f64>,

    /// Settlement trade date (YYYYMMDD).
    #[serde(default)]
    pub stts: Option<String>,

    /// Settlement send time (ms since epoch).
    #[serde(default, with = "option_timestamp_ms")]
    pub sttst: Option<OffsetDateTime>,

    /// Previous settlement price.
    #[serde(default)]
    pub pstt: Option<f64>,

    /// Previous settlement trade date (YYYYMMDD).
    #[serde(default)]
    pub pstts: Option<String>,

    /// Settlement change.
    #[serde(default)]
    pub sttch: Option<f64>,

    /// High bid.
    #[serde(default)]
    pub hb: Option<f64>,

    /// Low ask.
    #[serde(default)]
    pub la: Option<f64>,

    /// Bid price.
    #[serde(default)]
    pub b: Option<f64>,

    /// Bid time (ms since epoch).
    #[serde(default, with = "option_timestamp_ms")]
    pub bt: Option<OffsetDateTime>,

    /// Bid size.
    #[serde(default)]
    pub bs: Option<i64>,

    /// Implied bid order count.
    #[serde(default)]
    pub ibc: Option<i64>,

    /// Implied bid size.
    #[serde(default)]
    pub ibs: Option<i32>,

    /// Ask price.
    #[serde(default)]
    pub a: Option<f64>,

    /// Ask time (ms since epoch).
    #[serde(default, with = "option_timestamp_ms")]
    pub at: Option<OffsetDateTime>,

    /// Ask size. Field name is `as` in JSON (Rust keyword).
    #[serde(rename = "as", default)]
    pub ask_size: Option<i64>,

    /// Implied ask size.
    #[serde(default)]
    pub ias: Option<i64>,

    /// Implied ask order count.
    #[serde(default)]
    pub iac: Option<i64>,

    /// Trade time (ms since epoch).
    #[serde(default, with = "option_timestamp_ms")]
    pub tt: Option<OffsetDateTime>,

    /// Trade date (YYYYMMDD).
    #[serde(default)]
    pub tdt: Option<String>,

    /// Security status.
    #[serde(default)]
    pub secs: Option<i32>,

    /// Session date (YYYYMMDD).
    #[serde(default)]
    pub sdt: Option<String>,

    /// Open interest.
    #[serde(default)]
    pub oi: Option<i32>,

    /// Total volume.
    #[serde(default)]
    pub tv: Option<i32>,

    /// Block volume.
    #[serde(default)]
    pub bv: Option<i32>,

    /// Swaps volume.
    #[serde(default)]
    pub swv: Option<i32>,

    /// Physical volume.
    #[serde(default)]
    pub pv: Option<i32>,
}

/// Market depth (order book).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Depth {
    /// Exchange symbol.
    pub s: Symbol,

    /// Bid levels.
    #[serde(default)]
    pub b: Vec<DepthLevel>,

    /// Ask levels.
    #[serde(default)]
    pub a: Vec<DepthLevel>,
}

/// Single depth level in the order book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthLevel {
    /// Level number (0-based).
    pub l: i32,

    /// Time (ms since epoch).
    #[serde(default, with = "option_timestamp_ms")]
    pub t: Option<OffsetDateTime>,

    /// Side ("B" or "A").
    pub s: String,

    /// Price.
    pub p: f64,

    /// Order count.
    #[serde(default)]
    pub o: Option<i32>,

    /// Total size.
    #[serde(default)]
    pub sz: Option<f64>,

    /// Implied order count.
    #[serde(default)]
    pub ioc: Option<i32>,

    /// Implied size. Field name is `is` in JSON (Rust keyword).
    #[serde(rename = "is", default)]
    pub implied_size: Option<f64>,
}

/// Trade from REST endpoint (full field names).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Trade {
    /// Symbol.
    pub symbol: String,

    /// Trade price.
    pub price: f64,

    /// Price change.
    #[serde(default)]
    pub change: Option<f64>,

    /// Trade size.
    #[serde(default)]
    pub size: Option<f64>,

    /// Sequence number.
    #[serde(rename = "sequenceNumber", default)]
    pub sequence_number: Option<i64>,

    /// Send time (ms since epoch).
    #[serde(rename = "sendTime", default, with = "option_timestamp_ms")]
    pub send_time: Option<OffsetDateTime>,

    /// Tick direction.
    #[serde(rename = "tickDirection", default)]
    pub tick_direction: Option<TickDirection>,

    /// Aggressor side.
    #[serde(rename = "aggressorSide", default)]
    pub aggressor_side: Option<AggressorSideType>,

    /// Trade date (YYYYMMDD).
    #[serde(rename = "tradeDate", default)]
    pub trade_date: Option<String>,

    /// Trade identifier.
    #[serde(rename = "tradeId", default)]
    pub trade_id: Option<i64>,

    /// Total volume.
    #[serde(rename = "totalVolume", default)]
    pub total_volume: Option<f64>,
}

/// Streaming trade (abbreviated field names, extra fields).
///
/// Separate from [`Trade`] because it uses `TickDirectionType` (integer) instead of
/// `TickDirection` (string) and has additional fields not present in REST.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeOpt {
    /// Symbol.
    pub s: Symbol,

    /// Price.
    pub p: f64,

    /// Change.
    #[serde(default)]
    pub ch: Option<f64>,

    /// Size.
    #[serde(default)]
    pub sz: Option<f64>,

    /// Sequence number.
    #[serde(default)]
    pub sq: Option<i64>,

    /// Send time (ms since epoch).
    #[serde(default, with = "option_timestamp_ms")]
    pub st: Option<OffsetDateTime>,

    /// Tick direction (integer-encoded).
    #[serde(default)]
    pub td: Option<TickDirectionType>,

    /// Aggressor side. Field name is `as` in JSON (Rust keyword).
    #[serde(rename = "as", default)]
    pub aggressor_side: Option<AggressorSideType>,

    /// Trade date (YYYYMMDD).
    #[serde(default)]
    pub tdt: Option<String>,

    /// Trade identifier.
    #[serde(default)]
    pub tid: Option<i64>,

    /// Whether this is a settlement trade. Field name is `is` in JSON (Rust keyword).
    #[serde(rename = "is", default)]
    pub is_settlement: Option<bool>,

    /// Whether trade is cancelled.
    #[serde(default)]
    pub clx: Option<bool>,

    /// System-priced trade indicator.
    #[serde(default)]
    pub spt: Option<SystemPricedTrade>,

    /// Investigation status.
    #[serde(default)]
    pub ist: Option<InvestigationStatus>,

    /// Block trade type.
    #[serde(default)]
    pub bt: Option<BlockTrade>,
}

/// Generates a bar struct with OHLCV fields.
macro_rules! define_bar {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            /// Time (ms since epoch).
            #[serde(with = "option_timestamp_ms", default)]
            pub t: Option<OffsetDateTime>,

            /// Open price.
            #[serde(default)]
            pub o: Option<f64>,

            /// High price.
            #[serde(default)]
            pub h: Option<f64>,

            /// Low price.
            #[serde(default)]
            pub l: Option<f64>,

            /// Close price.
            #[serde(default)]
            pub c: Option<f64>,

            /// Volume.
            #[serde(default)]
            pub v: Option<f64>,

            /// Trade count.
            #[serde(default)]
            pub tc: Option<i64>,

            /// Delta.
            #[serde(default)]
            pub d: Option<f64>,

            /// Indicator name.
            #[serde(default)]
            pub i: Option<String>,
        }
    };
}

define_bar!(
    /// Trade bar (OHLCV aggregated by trade activity).
    TradeBar
);

define_bar!(
    /// Tick bar (OHLCV aggregated by tick count).
    TickBar
);

define_bar!(
    /// Time bar (OHLCV aggregated by time period).
    TimeBar
);

define_bar!(
    /// Volume bar (OHLCV aggregated by volume).
    VolumeBar
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quote_full_deserialize() {
        let json = r#"{"s":"XCME:ES.U16","l":4500.25,"sz":10,"ch":5.0,"as":150}"#;
        let q: QuoteFull = serde_json::from_str(json).unwrap();
        assert_eq!(q.s, "XCME:ES.U16");
        assert_eq!(q.l, Some(4500.25));
        assert_eq!(q.ask_size, Some(150));
    }

    #[test]
    fn depth_level_implied_size() {
        let json = r#"{"l":0,"s":"B","p":4500.0,"is":25.0}"#;
        let dl: DepthLevel = serde_json::from_str(json).unwrap();
        assert_eq!(dl.implied_size, Some(25.0));
    }

    #[test]
    fn trade_rest_deserialize() {
        let json = r#"{
            "symbol":"XCME:ES.U16",
            "price":4500.0,
            "change":5.0,
            "tickDirection":"PLUS",
            "aggressorSide":1
        }"#;
        let t: Trade = serde_json::from_str(json).unwrap();
        assert_eq!(t.symbol, "XCME:ES.U16");
        assert_eq!(t.tick_direction, Some(TickDirection::Plus));
        assert_eq!(t.aggressor_side, Some(AggressorSideType::Buy));
    }

    #[test]
    fn trade_opt_streaming_deserialize() {
        let json = r#"{
            "s":"XCME:ES.U16",
            "p":4500.0,
            "td":0,
            "as":1,
            "is":false,
            "clx":false,
            "spt":"SYSTEM"
        }"#;
        let t: TradeOpt = serde_json::from_str(json).unwrap();
        assert_eq!(t.s, "XCME:ES.U16");
        assert_eq!(t.td, Some(TickDirectionType::Plus));
        assert_eq!(t.aggressor_side, Some(AggressorSideType::Buy));
        assert_eq!(t.is_settlement, Some(false));
        assert_eq!(t.spt, Some(SystemPricedTrade::System));
    }

    #[test]
    fn trade_bar_deserialize() {
        let json = r#"{"t":1705322400000,"o":4500.0,"h":4510.0,"l":4495.0,"c":4505.0,"v":1000.0,"tc":50}"#;
        let bar: TradeBar = serde_json::from_str(json).unwrap();
        assert!(bar.t.is_some());
        assert_eq!(bar.o, Some(4500.0));
        assert_eq!(bar.tc, Some(50));
    }
}
