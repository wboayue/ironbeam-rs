use crate::types::account::AccountPositions;
use crate::types::common::{PingMessage, Response};
use crate::types::market::{Depth, QuoteFull, TickBar, TimeBar, TradeBar, TradeOpt, VolumeBar};
use crate::types::order::{Order, OrderFill};
use crate::types::streaming::{IndicatorValues, StreamResponse};
use crate::types::{Balance, Position, RiskInfo};

/// Typed event from a streaming WebSocket connection.
///
/// Each variant corresponds to a populated field in the raw [`StreamResponse`] envelope.
/// A single WebSocket message may produce multiple events.
#[derive(Debug, Clone, PartialEq)]
pub enum StreamEvent {
    /// Keepalive ping.
    Ping(PingMessage),
    /// Quote updates.
    Quotes(Vec<QuoteFull>),
    /// Depth (order book) updates.
    Depth(Vec<Depth>),
    /// Trade updates (streaming format).
    Trades(Vec<TradeOpt>),
    /// Order updates.
    Orders(Vec<Order>),
    /// Fill updates.
    Fills(Vec<OrderFill>),
    /// Position changes.
    Positions(Vec<Position>),
    /// Initial position snapshot (all accounts).
    AllPositions(Vec<AccountPositions>),
    /// Balance update.
    Balance(Box<Balance>),
    /// Initial balance snapshot (all accounts).
    AllBalances(Vec<Balance>),
    /// Risk info change.
    Risk(RiskInfo),
    /// Initial risk snapshot (all accounts).
    AllRisk(Vec<RiskInfo>),
    /// Trade bars.
    TradeBars(Vec<TradeBar>),
    /// Tick bars.
    TickBars(Vec<TickBar>),
    /// Time bars.
    TimeBars(Vec<TimeBar>),
    /// Volume bars.
    VolumeBars(Vec<VolumeBar>),
    /// Indicator values.
    Indicators(Vec<IndicatorValues>),
    /// Account/session notification.
    Notification(Response),
}

impl StreamResponse {
    /// Extract all populated fields into individual [`StreamEvent`] values.
    ///
    /// Uses chained iterators to avoid heap allocation on the hot path.
    pub(crate) fn into_events(self) -> impl Iterator<Item = StreamEvent> {
        std::iter::empty()
            .chain(self.ping.map(StreamEvent::Ping))
            .chain(self.quotes.map(StreamEvent::Quotes))
            .chain(self.depths.map(StreamEvent::Depth))
            .chain(self.trades.map(StreamEvent::Trades))
            .chain(self.orders.map(StreamEvent::Orders))
            .chain(self.fills.map(StreamEvent::Fills))
            .chain(self.positions.map(StreamEvent::Positions))
            .chain(self.all_positions.map(StreamEvent::AllPositions))
            .chain(self.balance.map(|v| StreamEvent::Balance(Box::new(v))))
            .chain(self.all_balances.map(StreamEvent::AllBalances))
            .chain(self.risk.map(StreamEvent::Risk))
            .chain(self.all_risk.map(StreamEvent::AllRisk))
            .chain(self.trade_bars.map(StreamEvent::TradeBars))
            .chain(self.tick_bars.map(StreamEvent::TickBars))
            .chain(self.time_bars.map(StreamEvent::TimeBars))
            .chain(self.volume_bars.map(StreamEvent::VolumeBars))
            .chain(self.indicators.map(StreamEvent::Indicators))
            .chain(self.notification.map(StreamEvent::Notification))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_event() {
        let json = r#"{"p":{"ping":"keepalive"}}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(
            matches!(&events[0], StreamEvent::Ping(p) if p.ping.as_deref() == Some("keepalive"))
        );
    }

    #[test]
    fn quotes_event() {
        let json = r#"{"q":[{"s":"XCME:ES.U25","l":4500.0}]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::Quotes(q) if q[0].symbol == "XCME:ES.U25"));
    }

    #[test]
    fn depth_event() {
        let json = r#"{"d":[{"s":"XCME:ES.U25","b":[],"a":[]}]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::Depth(..)));
    }

    #[test]
    fn order_event() {
        let json = r#"{"o":[{"oid":"O1","a":"A1","s":"ES","st":"NEW","sd":"BUY","q":1.0,"ot":"1","dr":"0"}]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::Orders(o) if o[0].order_id == "O1"));
    }

    #[test]
    fn notification_event() {
        let json = r#"{"r":{"status":"OK","message":"connected"}}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::Notification(..)));
    }

    #[test]
    fn multiple_fields() {
        let json = r#"{"p":{"ping":"keepalive"},"q":[{"s":"ES"}]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 2);
        assert!(matches!(&events[0], StreamEvent::Ping(..)));
        assert!(matches!(&events[1], StreamEvent::Quotes(..)));
    }

    #[test]
    fn order_event_rest_field_names() {
        let json = r#"{"o":[{
            "orderId":"O2","accountId":"A2","exchSym":"NQ",
            "status":"NEW","side":"BUY","quantity":2.0,
            "orderType":"1","duration":"0"
        }]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::Orders(o) if o[0].order_id == "O2"));
    }

    #[test]
    fn order_event_mixed_alias_fields() {
        // Real API can mix short streaming names and long REST names in a single payload.
        let json = r#"{"o":[{
            "orderId":"O3","a":"A3","exchSym":"ES",
            "st":"NEW","side":"SELL","q":3.0,
            "orderType":"2","dr":"1"
        }]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(
            matches!(&events[0], StreamEvent::Orders(o) if o[0].order_id == "O3" && o[0].account_id == "A3")
        );
    }

    #[test]
    fn balance_event() {
        let json = r#"{"b":{"a":"ACC1","cc":"USD","cb":10000.0}}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::Balance(b) if b.account_id == "ACC1"));
    }

    #[test]
    fn fills_event() {
        let json = r#"{"f":[{"oid":"F1","a":"ACC1","s":"ES","fp":4500.0}]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::Fills(f) if f[0].order_id == "F1"));
    }

    #[test]
    fn trade_bars_event() {
        let json = r#"{"tb":[{"t":1705322400000,"o":4500.0,"h":4510.0,"l":4495.0,"c":4505.0,"v":1000.0,"tc":50}]}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert_eq!(events.len(), 1);
        assert!(matches!(&events[0], StreamEvent::TradeBars(bars) if bars[0].open == Some(4500.0)));
    }

    #[test]
    fn empty_response() {
        let json = r#"{}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert!(events.is_empty());
    }
}
