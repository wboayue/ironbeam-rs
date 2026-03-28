use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::common::OrderError;
use super::{
    DurationType, OrderSide, OrderStatusType, OrderType, Symbol, option_datetime_rfc3339,
    option_timestamp_ms,
};

/// Order. Unified across REST and streaming.
///
/// REST uses full camelCase field names; streaming uses abbreviated aliases.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Order {
    /// Order identifier.
    #[serde(rename = "orderId", alias = "oid")]
    pub order_id: String,

    /// Strategy identifier.
    #[serde(rename = "strategyId", alias = "sid", default)]
    pub strategy_id: Option<i64>,

    /// Parent order identifier (for bracket orders).
    #[serde(rename = "parentOrderId", alias = "poid", default)]
    pub parent_order_id: Option<String>,

    /// Account identifier.
    #[serde(rename = "accountId", alias = "a")]
    pub account_id: String,

    /// Exchange-qualified symbol.
    #[serde(rename = "exchSym", alias = "s")]
    pub exch_sym: Symbol,

    /// Order status.
    #[serde(alias = "st")]
    pub status: OrderStatusType,

    /// Order side.
    #[serde(alias = "sd")]
    pub side: OrderSide,

    /// Order quantity.
    #[serde(alias = "q")]
    pub quantity: f64,

    /// Limit price.
    #[serde(rename = "limitPrice", alias = "lp", default)]
    pub limit_price: Option<f64>,

    /// Stop price.
    #[serde(rename = "stopPrice", alias = "sp", default)]
    pub stop_price: Option<f64>,

    /// Order type.
    #[serde(rename = "orderType", alias = "ot")]
    pub order_type: OrderType,

    /// Order duration.
    #[serde(alias = "dr")]
    pub duration: DurationType,

    /// Filled quantity.
    #[serde(rename = "fillQuantity", alias = "fq", default)]
    pub fill_quantity: Option<f64>,

    /// Average fill price.
    #[serde(rename = "fillPrice", alias = "fp", default)]
    pub fill_price: Option<f64>,

    /// Fill date.
    #[serde(
        rename = "fillDate",
        alias = "fd",
        default,
        with = "option_datetime_rfc3339"
    )]
    pub fill_date: Option<OffsetDateTime>,

    /// Child order IDs (stop loss / take profit).
    #[serde(rename = "childOrders", alias = "cor", default)]
    pub child_orders: Option<Vec<String>>,

    /// Order error details.
    #[serde(rename = "orderError", alias = "err", default)]
    pub order_error: Option<OrderError>,
}

/// Order fill. Unified across REST and streaming.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderFill {
    /// Order identifier.
    #[serde(rename = "orderId", alias = "oid")]
    pub order_id: String,

    /// Strategy identifier.
    #[serde(rename = "strategyId", alias = "sid", default)]
    pub strategy_id: Option<i64>,

    /// Account identifier.
    #[serde(rename = "accountId", alias = "a")]
    pub account_id: String,

    /// Exchange-qualified symbol.
    #[serde(rename = "exchSym", alias = "s")]
    pub exch_sym: Symbol,

    /// Order status.
    #[serde(alias = "st", default)]
    pub status: Option<OrderStatusType>,

    /// Order side.
    #[serde(alias = "sd", default)]
    pub side: Option<OrderSide>,

    /// Order quantity.
    #[serde(alias = "q", default)]
    pub quantity: Option<f64>,

    /// Trade price.
    #[serde(alias = "p", default)]
    pub price: Option<f64>,

    /// Fill quantity (this execution).
    #[serde(rename = "fillQuantity", alias = "fq", default)]
    pub fill_quantity: Option<f64>,

    /// Total filled quantity across all fills.
    #[serde(rename = "fillTotalQuantity", alias = "ftq", default)]
    pub fill_total_quantity: Option<f64>,

    /// Fill price (this execution).
    #[serde(rename = "fillPrice", alias = "fp", default)]
    pub fill_price: Option<f64>,

    /// Average fill price across all fills.
    #[serde(rename = "avgFillPrice", alias = "afp", default)]
    pub avg_fill_price: Option<f64>,

    /// Fill date.
    #[serde(
        rename = "fillDate",
        alias = "fd",
        default,
        with = "option_datetime_rfc3339"
    )]
    pub fill_date: Option<OffsetDateTime>,

    /// Time of order event.
    #[serde(
        rename = "timeOrderEvent",
        alias = "t",
        default,
        with = "option_timestamp_ms"
    )]
    pub time_order_event: Option<OffsetDateTime>,

    /// Order update identifier.
    #[serde(rename = "orderUpdateId", alias = "ouid", default)]
    pub order_update_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn order_from_rest() {
        let json = r#"{
            "orderId": "ORD001",
            "accountId": "ACC001",
            "exchSym": "XCME:ES.U16",
            "status": "NEW",
            "side": "BUY",
            "quantity": 5.0,
            "orderType": "2",
            "duration": "0",
            "limitPrice": 4500.0
        }"#;
        let o: Order = serde_json::from_str(json).unwrap();
        assert_eq!(o.order_id, "ORD001");
        assert_eq!(o.status, OrderStatusType::New);
        assert_eq!(o.order_type, OrderType::Limit);
        assert_eq!(o.duration, DurationType::Day);
        assert_eq!(o.limit_price, Some(4500.0));
    }

    #[test]
    fn order_from_streaming() {
        let json = r#"{
            "oid": "ORD001",
            "a": "ACC001",
            "s": "XCME:ES.U16",
            "st": "NEW",
            "sd": "BUY",
            "q": 5.0,
            "ot": "2",
            "dr": "0",
            "lp": 4500.0
        }"#;
        let o: Order = serde_json::from_str(json).unwrap();
        assert_eq!(o.order_id, "ORD001");
        assert_eq!(o.status, OrderStatusType::New);
        assert_eq!(o.order_type, OrderType::Limit);
        assert_eq!(o.limit_price, Some(4500.0));
    }

    #[test]
    fn order_fill_from_rest() {
        let json = r#"{
            "orderId": "ORD001",
            "accountId": "ACC001",
            "exchSym": "XCME:ES.U16",
            "status": "FILLED",
            "side": "BUY",
            "fillQuantity": 5.0,
            "fillPrice": 4500.25,
            "avgFillPrice": 4500.25
        }"#;
        let f: OrderFill = serde_json::from_str(json).unwrap();
        assert_eq!(f.order_id, "ORD001");
        assert_eq!(f.status, Some(OrderStatusType::Filled));
        assert_eq!(f.fill_price, Some(4500.25));
    }

    #[test]
    fn order_fill_from_streaming() {
        let json = r#"{
            "oid": "ORD001",
            "a": "ACC001",
            "s": "XCME:ES.U16",
            "st": "FILLED",
            "sd": "BUY",
            "fq": 5.0,
            "fp": 4500.25,
            "afp": 4500.25
        }"#;
        let f: OrderFill = serde_json::from_str(json).unwrap();
        assert_eq!(f.order_id, "ORD001");
        assert_eq!(f.fill_quantity, Some(5.0));
        assert_eq!(f.avg_fill_price, Some(4500.25));
    }
}
