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
    pub(crate) fn into_events(self) -> impl Iterator<Item = StreamEvent> {
        let mut events = Vec::new();

        if let Some(v) = self.ping {
            events.push(StreamEvent::Ping(v));
        }
        if let Some(v) = self.quotes {
            events.push(StreamEvent::Quotes(v));
        }
        if let Some(v) = self.depths {
            events.push(StreamEvent::Depth(v));
        }
        if let Some(v) = self.trades {
            events.push(StreamEvent::Trades(v));
        }
        if let Some(v) = self.orders {
            events.push(StreamEvent::Orders(v));
        }
        if let Some(v) = self.fills {
            events.push(StreamEvent::Fills(v));
        }
        if let Some(v) = self.positions {
            events.push(StreamEvent::Positions(v));
        }
        if let Some(v) = self.all_positions {
            events.push(StreamEvent::AllPositions(v));
        }
        if let Some(v) = self.balance {
            events.push(StreamEvent::Balance(Box::new(v)));
        }
        if let Some(v) = self.all_balances {
            events.push(StreamEvent::AllBalances(v));
        }
        if let Some(v) = self.risk {
            events.push(StreamEvent::Risk(v));
        }
        if let Some(v) = self.all_risk {
            events.push(StreamEvent::AllRisk(v));
        }
        if let Some(v) = self.trade_bars {
            events.push(StreamEvent::TradeBars(v));
        }
        if let Some(v) = self.tick_bars {
            events.push(StreamEvent::TickBars(v));
        }
        if let Some(v) = self.time_bars {
            events.push(StreamEvent::TimeBars(v));
        }
        if let Some(v) = self.volume_bars {
            events.push(StreamEvent::VolumeBars(v));
        }
        if let Some(v) = self.indicators {
            events.push(StreamEvent::Indicators(v));
        }
        if let Some(v) = self.notification {
            events.push(StreamEvent::Notification(v));
        }

        events.into_iter()
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
        assert!(matches!(&events[0], StreamEvent::Ping(p) if p.ping.as_deref() == Some("keepalive")));
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
    fn empty_response() {
        let json = r#"{}"#;
        let sr: StreamResponse = serde_json::from_str(json).unwrap();
        let events: Vec<_> = sr.into_events().collect();
        assert!(events.is_empty());
    }
}
