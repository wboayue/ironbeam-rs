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

        if let Some(p) = self.p {
            events.push(StreamEvent::Ping(p));
        }
        if let Some(q) = self.q {
            events.push(StreamEvent::Quotes(q));
        }
        if let Some(d) = self.d {
            events.push(StreamEvent::Depth(d));
        }
        if let Some(tr) = self.tr {
            events.push(StreamEvent::Trades(tr));
        }
        if let Some(o) = self.o {
            events.push(StreamEvent::Orders(o));
        }
        if let Some(f) = self.f {
            events.push(StreamEvent::Fills(f));
        }
        if let Some(ps) = self.ps {
            events.push(StreamEvent::Positions(ps));
        }
        if let Some(psa) = self.psa {
            events.push(StreamEvent::AllPositions(psa));
        }
        if let Some(b) = self.b {
            events.push(StreamEvent::Balance(Box::new(b)));
        }
        if let Some(ba) = self.ba {
            events.push(StreamEvent::AllBalances(ba));
        }
        if let Some(ri) = self.ri {
            events.push(StreamEvent::Risk(ri));
        }
        if let Some(ria) = self.ria {
            events.push(StreamEvent::AllRisk(ria));
        }
        if let Some(tb) = self.tb {
            events.push(StreamEvent::TradeBars(tb));
        }
        if let Some(tc) = self.tc {
            events.push(StreamEvent::TickBars(tc));
        }
        if let Some(ti) = self.ti {
            events.push(StreamEvent::TimeBars(ti));
        }
        if let Some(vb) = self.vb {
            events.push(StreamEvent::VolumeBars(vb));
        }
        if let Some(i) = self.i {
            events.push(StreamEvent::Indicators(i));
        }
        if let Some(r) = self.r {
            events.push(StreamEvent::Notification(r));
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
        assert!(matches!(&events[0], StreamEvent::Quotes(q) if q[0].s == "XCME:ES.U25"));
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
