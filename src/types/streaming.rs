use serde::{Deserialize, Serialize};

use super::{
    Balance, BarType, Depth, OrderFill, Position, QuoteFull, RiskInfo, Symbol, TickBar, TimeBar,
    TradeBar, TradeOpt, VolumeBar,
};
use super::account::AccountPositions;
use super::common::{PingMessage, Response};
use super::order::Order;

/// WebSocket stream response envelope.
///
/// All fields are optional; only the relevant field is populated for a given message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StreamResponse {
    /// Keepalive ping.
    #[serde(default)]
    pub p: Option<PingMessage>,

    /// Quote updates.
    #[serde(default)]
    pub q: Option<Vec<QuoteFull>>,

    /// Depth updates.
    #[serde(default)]
    pub d: Option<Vec<Depth>>,

    /// Trade updates (streaming format).
    #[serde(default)]
    pub tr: Option<Vec<TradeOpt>>,

    /// Order updates.
    #[serde(default)]
    pub o: Option<Vec<Order>>,

    /// Fill updates.
    #[serde(default)]
    pub f: Option<Vec<OrderFill>>,

    /// Position changes.
    #[serde(default)]
    pub ps: Option<Vec<Position>>,

    /// Initial position snapshot (all accounts).
    #[serde(default)]
    pub psa: Option<Vec<AccountPositions>>,

    /// Balance update.
    #[serde(default)]
    pub b: Option<Balance>,

    /// Initial balance snapshot (all accounts).
    #[serde(default)]
    pub ba: Option<Vec<Balance>>,

    /// Risk info change.
    #[serde(default)]
    pub ri: Option<RiskInfo>,

    /// Initial risk snapshot (all accounts).
    #[serde(default)]
    pub ria: Option<Vec<RiskInfo>>,

    /// Trade bars.
    #[serde(default)]
    pub tb: Option<Vec<TradeBar>>,

    /// Tick bars.
    #[serde(default)]
    pub tc: Option<Vec<TickBar>>,

    /// Time bars.
    #[serde(default)]
    pub ti: Option<Vec<TimeBar>>,

    /// Volume bars.
    #[serde(default)]
    pub vb: Option<Vec<VolumeBar>>,

    /// Indicator values.
    #[serde(default)]
    pub i: Option<Vec<IndicatorValues>>,

    /// Account/session change notification.
    #[serde(default)]
    pub r: Option<Response>,
}

/// Indicator values from streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IndicatorValues {
    /// Unique indicator value name.
    pub n: String,

    /// From index.
    #[serde(default)]
    pub fi: Option<i64>,

    /// 2D array of values.
    #[serde(default)]
    pub v: Vec<Vec<String>>,
}

/// Response when subscribing to an indicator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IndicatorSubscribeResponse {
    /// Indicator identifier (use to unsubscribe).
    #[serde(rename = "indicatorId")]
    pub indicator_id: String,

    /// Value names (e.g. ["date","open","close","high","low","volume"]).
    #[serde(rename = "valueNames", default)]
    pub value_names: Vec<String>,

    /// Value types (e.g. ["date","number","string"]).
    #[serde(rename = "valueTypes", default)]
    pub value_types: Vec<String>,
}

/// Request body for bar indicator subscriptions (trade/tick/time/volume bars).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribeBarsRequest {
    /// Symbol to subscribe.
    pub symbol: Symbol,

    /// Bar period.
    pub period: i32,

    /// Bar type.
    #[serde(rename = "barType")]
    pub bar_type: BarType,

    /// Initial history load size.
    #[serde(rename = "loadSize")]
    pub load_size: i32,
}

/// Response from stream creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamIdResponse {
    /// Stream session identifier (UUID).
    #[serde(rename = "streamId")]
    pub stream_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_response_ping() {
        let json = r#"{"p":{"ping":"keepalive"}}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        assert!(sr.p.is_some());
        assert!(sr.q.is_none());
    }

    #[test]
    fn stream_response_quotes() {
        let json = r#"{"q":[{"s":"XCME:ES.U16","l":4500.0}]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        assert_eq!(sr.q.as_ref().unwrap().len(), 1);
        assert_eq!(sr.q.as_ref().unwrap()[0].s, "XCME:ES.U16");
    }

    #[test]
    fn stream_response_order_with_aliases() {
        let json = r#"{"o":[{
            "oid":"ORD001",
            "a":"ACC001",
            "s":"XCME:ES.U16",
            "st":"NEW",
            "sd":"BUY",
            "q":5.0,
            "ot":"2",
            "dr":"0"
        }]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let orders = sr.o.unwrap();
        assert_eq!(orders[0].order_id, "ORD001");
    }

    #[test]
    fn stream_id_response() {
        let json = r#"{"streamId":"550e8400-e29b-41d4-a716-446655440000"}"#;
        let r: StreamIdResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.stream_id, "550e8400-e29b-41d4-a716-446655440000");
    }

    #[test]
    fn indicator_subscribe_response() {
        let json = r#"{
            "indicatorId":"IND001",
            "valueNames":["date","open","close"],
            "valueTypes":["date","number","number"]
        }"#;
        let r: IndicatorSubscribeResponse = serde_json::from_str(json).unwrap();
        assert_eq!(r.indicator_id, "IND001");
        assert_eq!(r.value_names.len(), 3);
    }
}
