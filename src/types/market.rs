use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::{
    AggressorSideType, BlockTrade, InvestigationStatus, Symbol, SystemPricedTrade, TickDirection,
    TickDirectionType, option_timestamp_ms,
};

/// Full market quote. Used in both REST and streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteFull {
    /// Exchange symbol.
    #[serde(rename = "s")]
    pub symbol: Symbol,

    /// Last trade price.
    #[serde(rename = "l", default)]
    pub last_price: Option<f64>,

    /// Last trade size.
    #[serde(rename = "sz", default)]
    pub last_size: Option<i32>,

    /// Change from previous settle.
    #[serde(rename = "ch", default)]
    pub change: Option<f64>,

    /// Open price.
    #[serde(rename = "op", default)]
    pub open: Option<f64>,

    /// High price.
    #[serde(rename = "hi", default)]
    pub high: Option<f64>,

    /// Low price.
    #[serde(rename = "lo", default)]
    pub low: Option<f64>,

    /// Aggressor side.
    #[serde(rename = "ags", default)]
    pub aggressor_side: Option<i32>,

    /// Tick direction.
    #[serde(rename = "td", default)]
    pub tick_direction: Option<i32>,

    /// Settlement price.
    #[serde(rename = "stt", default)]
    pub settlement_price: Option<f64>,

    /// Settlement trade date (YYYYMMDD).
    #[serde(rename = "stts", default)]
    pub settlement_date: Option<String>,

    /// Settlement send time.
    #[serde(rename = "sttst", default, with = "option_timestamp_ms")]
    pub settlement_time: Option<OffsetDateTime>,

    /// Previous settlement price.
    #[serde(rename = "pstt", default)]
    pub prev_settlement_price: Option<f64>,

    /// Previous settlement trade date (YYYYMMDD).
    #[serde(rename = "pstts", default)]
    pub prev_settlement_date: Option<String>,

    /// Settlement change.
    #[serde(rename = "sttch", default)]
    pub settlement_change: Option<f64>,

    /// High bid.
    #[serde(rename = "hb", default)]
    pub high_bid: Option<f64>,

    /// Low ask.
    #[serde(rename = "la", default)]
    pub low_ask: Option<f64>,

    /// Bid price.
    #[serde(rename = "b", default)]
    pub bid: Option<f64>,

    /// Bid time.
    #[serde(rename = "bt", default, with = "option_timestamp_ms")]
    pub bid_time: Option<OffsetDateTime>,

    /// Bid size.
    #[serde(rename = "bs", default)]
    pub bid_size: Option<i64>,

    /// Implied bid order count.
    #[serde(rename = "ibc", default)]
    pub implied_bid_count: Option<i64>,

    /// Implied bid size.
    #[serde(rename = "ibs", default)]
    pub implied_bid_size: Option<i32>,

    /// Ask price.
    #[serde(rename = "a", default)]
    pub ask: Option<f64>,

    /// Ask time.
    #[serde(rename = "at", default, with = "option_timestamp_ms")]
    pub ask_time: Option<OffsetDateTime>,

    /// Ask size.
    #[serde(rename = "as", default)]
    pub ask_size: Option<i64>,

    /// Implied ask size.
    #[serde(rename = "ias", default)]
    pub implied_ask_size: Option<i64>,

    /// Implied ask order count.
    #[serde(rename = "iac", default)]
    pub implied_ask_count: Option<i64>,

    /// Trade time.
    #[serde(rename = "tt", default, with = "option_timestamp_ms")]
    pub trade_time: Option<OffsetDateTime>,

    /// Trade date (YYYYMMDD).
    #[serde(rename = "tdt", default)]
    pub trade_date: Option<String>,

    /// Security status.
    #[serde(rename = "secs", default)]
    pub security_status: Option<i32>,

    /// Session date (YYYYMMDD).
    #[serde(rename = "sdt", default)]
    pub session_date: Option<String>,

    /// Open interest.
    #[serde(rename = "oi", default)]
    pub open_interest: Option<i32>,

    /// Total volume.
    #[serde(rename = "tv", default)]
    pub total_volume: Option<i32>,

    /// Block volume.
    #[serde(rename = "bv", default)]
    pub block_volume: Option<i32>,

    /// Swaps volume.
    #[serde(rename = "swv", default)]
    pub swaps_volume: Option<i32>,

    /// Physical volume.
    #[serde(rename = "pv", default)]
    pub physical_volume: Option<i32>,
}

/// Market depth (order book).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Depth {
    /// Exchange symbol.
    #[serde(rename = "s")]
    pub symbol: Symbol,

    /// Bid levels.
    #[serde(rename = "b", default)]
    pub bids: Vec<DepthLevel>,

    /// Ask levels.
    #[serde(rename = "a", default)]
    pub asks: Vec<DepthLevel>,
}

/// Single depth level in the order book.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DepthLevel {
    /// Level number (0-based).
    #[serde(rename = "l")]
    pub level: i32,

    /// Time.
    #[serde(rename = "t", default, with = "option_timestamp_ms")]
    pub time: Option<OffsetDateTime>,

    /// Side ("B" or "A").
    #[serde(rename = "s")]
    pub side: String,

    /// Price.
    #[serde(rename = "p")]
    pub price: f64,

    /// Order count.
    #[serde(rename = "o", default)]
    pub order_count: Option<i32>,

    /// Total size.
    #[serde(rename = "sz", default)]
    pub size: Option<f64>,

    /// Implied order count.
    #[serde(rename = "ioc", default)]
    pub implied_order_count: Option<i32>,

    /// Implied size.
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

    /// Send time.
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
    #[serde(rename = "s")]
    pub symbol: Symbol,

    /// Price.
    #[serde(rename = "p")]
    pub price: f64,

    /// Change.
    #[serde(rename = "ch", default)]
    pub change: Option<f64>,

    /// Size.
    #[serde(rename = "sz", default)]
    pub size: Option<f64>,

    /// Sequence number.
    #[serde(rename = "sq", default)]
    pub sequence_number: Option<i64>,

    /// Send time.
    #[serde(rename = "st", default, with = "option_timestamp_ms")]
    pub send_time: Option<OffsetDateTime>,

    /// Tick direction (integer-encoded).
    #[serde(rename = "td", default)]
    pub tick_direction: Option<TickDirectionType>,

    /// Aggressor side.
    #[serde(rename = "as", default)]
    pub aggressor_side: Option<AggressorSideType>,

    /// Trade date (YYYYMMDD).
    #[serde(rename = "tdt", default)]
    pub trade_date: Option<String>,

    /// Trade identifier.
    #[serde(rename = "tid", default)]
    pub trade_id: Option<i64>,

    /// Whether this is a settlement trade.
    #[serde(rename = "is", default)]
    pub is_settlement: Option<bool>,

    /// Whether trade is cancelled.
    #[serde(rename = "clx", default)]
    pub is_cancelled: Option<bool>,

    /// System-priced trade indicator.
    #[serde(rename = "spt", default)]
    pub system_priced: Option<SystemPricedTrade>,

    /// Investigation status.
    #[serde(rename = "ist", default)]
    pub investigation_status: Option<InvestigationStatus>,

    /// Block trade type.
    #[serde(rename = "bt", default)]
    pub block_trade: Option<BlockTrade>,
}

/// Generates a bar struct with OHLCV fields.
macro_rules! define_bar {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name {
            /// Time.
            #[serde(rename = "t", with = "option_timestamp_ms", default)]
            pub time: Option<OffsetDateTime>,

            /// Open price.
            #[serde(rename = "o", default)]
            pub open: Option<f64>,

            /// High price.
            #[serde(rename = "h", default)]
            pub high: Option<f64>,

            /// Low price.
            #[serde(rename = "l", default)]
            pub low: Option<f64>,

            /// Close price.
            #[serde(rename = "c", default)]
            pub close: Option<f64>,

            /// Volume.
            #[serde(rename = "v", default)]
            pub volume: Option<f64>,

            /// Trade count.
            #[serde(rename = "tc", default)]
            pub trade_count: Option<i64>,

            /// Delta.
            #[serde(rename = "d", default)]
            pub delta: Option<f64>,

            /// Indicator name.
            #[serde(rename = "i", default)]
            pub indicator: Option<String>,
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
        assert_eq!(q.symbol, "XCME:ES.U16");
        assert_eq!(q.last_price, Some(4500.25));
        assert_eq!(q.ask_size, Some(150));
    }

    #[test]
    fn quote_full_round_trip() {
        let json = r#"{"s":"XCME:ES.U16","l":4500.25,"b":4500.0,"a":4500.5}"#;
        let q: QuoteFull = serde_json::from_str(json).unwrap();
        assert_eq!(q.bid, Some(4500.0));
        assert_eq!(q.ask, Some(4500.5));
        let serialized = serde_json::to_string(&q).unwrap();
        assert!(serialized.contains(r#""s":"XCME:ES.U16""#));
        assert!(serialized.contains(r#""b":4500.0"#));
    }

    #[test]
    fn depth_deserialize() {
        let json = r#"{"s":"XCME:ES.U16","b":[{"l":0,"s":"B","p":4500.0}],"a":[]}"#;
        let d: Depth = serde_json::from_str(json).unwrap();
        assert_eq!(d.symbol, "XCME:ES.U16");
        assert_eq!(d.bids.len(), 1);
        assert_eq!(d.bids[0].level, 0);
        assert_eq!(d.bids[0].price, 4500.0);
        assert!(d.asks.is_empty());
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
        assert_eq!(t.symbol, "XCME:ES.U16");
        assert_eq!(t.tick_direction, Some(TickDirectionType::Plus));
        assert_eq!(t.aggressor_side, Some(AggressorSideType::Buy));
        assert_eq!(t.is_settlement, Some(false));
        assert_eq!(t.system_priced, Some(SystemPricedTrade::System));
    }

    #[test]
    fn trade_bar_deserialize() {
        let json =
            r#"{"t":1705322400000,"o":4500.0,"h":4510.0,"l":4495.0,"c":4505.0,"v":1000.0,"tc":50}"#;
        let bar: TradeBar = serde_json::from_str(json).unwrap();
        assert!(bar.time.is_some());
        assert_eq!(bar.open, Some(4500.0));
        assert_eq!(bar.trade_count, Some(50));
    }
}
