use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::{Error, Result};
use crate::types::{
    DurationType, Order, OrderBaseResponse, OrderCancelMultipleRequest, OrderFill, OrderRequest,
    OrderSide, OrderStatusType, OrderType, OrderUpdateRequest, OrdersFillsResponse,
    OrdersResponse, Symbol,
};

// ---------------------------------------------------------------------------
// OrderBuilder — ergonomic order construction
// ---------------------------------------------------------------------------

/// Builder for constructing order requests.
///
/// Use the type-specific constructors to avoid invalid field combinations:
///
/// # Example
///
/// ```
/// use ironbeam_rs::client::OrderBuilder;
/// use ironbeam_rs::types::{OrderSide, DurationType};
///
/// let order = OrderBuilder::limit("XCME:ES.U16", OrderSide::Buy, 1.0, 4500.0, DurationType::Day)
///     .stop_loss(4480.0)
///     .take_profit(4550.0);
/// ```
#[derive(Debug, Clone)]
pub struct OrderBuilder {
    exch_sym: Symbol,
    side: OrderSide,
    quantity: f64,
    order_type: OrderType,
    duration: DurationType,
    limit_price: Option<f64>,
    stop_price: Option<f64>,
    stop_loss: Option<f64>,
    take_profit: Option<f64>,
    stop_loss_offset: Option<f32>,
    take_profit_offset: Option<f32>,
    trailing_stop: Option<f32>,
    wait_for_order_id: Option<bool>,
}

impl OrderBuilder {
    /// Create a market order.
    #[must_use]
    pub fn market(
        symbol: impl Into<Symbol>,
        side: OrderSide,
        quantity: f64,
        duration: DurationType,
    ) -> Self {
        Self {
            exch_sym: symbol.into(),
            side,
            quantity,
            order_type: OrderType::Market,
            duration,
            limit_price: None,
            stop_price: None,
            stop_loss: None,
            take_profit: None,
            stop_loss_offset: None,
            take_profit_offset: None,
            trailing_stop: None,
            wait_for_order_id: None,
        }
    }

    /// Create a limit order.
    #[must_use]
    pub fn limit(
        symbol: impl Into<Symbol>,
        side: OrderSide,
        quantity: f64,
        price: f64,
        duration: DurationType,
    ) -> Self {
        let mut builder = Self::market(symbol, side, quantity, duration);
        builder.order_type = OrderType::Limit;
        builder.limit_price = Some(price);
        builder
    }

    /// Create a stop order.
    #[must_use]
    pub fn stop(
        symbol: impl Into<Symbol>,
        side: OrderSide,
        quantity: f64,
        stop_price: f64,
        duration: DurationType,
    ) -> Self {
        let mut builder = Self::market(symbol, side, quantity, duration);
        builder.order_type = OrderType::Stop;
        builder.stop_price = Some(stop_price);
        builder
    }

    /// Create a stop-limit order.
    #[must_use]
    pub fn stop_limit(
        symbol: impl Into<Symbol>,
        side: OrderSide,
        quantity: f64,
        limit_price: f64,
        stop_price: f64,
        duration: DurationType,
    ) -> Self {
        let mut builder = Self::market(symbol, side, quantity, duration);
        builder.order_type = OrderType::StopLimit;
        builder.limit_price = Some(limit_price);
        builder.stop_price = Some(stop_price);
        builder
    }

    /// Set bracket stop loss price.
    #[must_use]
    pub fn stop_loss(mut self, price: f64) -> Self {
        self.stop_loss = Some(price);
        self
    }

    /// Set bracket take profit price.
    #[must_use]
    pub fn take_profit(mut self, price: f64) -> Self {
        self.take_profit = Some(price);
        self
    }

    /// Set stop loss offset in pips.
    #[must_use]
    pub fn stop_loss_offset(mut self, pips: f32) -> Self {
        self.stop_loss_offset = Some(pips);
        self
    }

    /// Set take profit offset in pips.
    #[must_use]
    pub fn take_profit_offset(mut self, pips: f32) -> Self {
        self.take_profit_offset = Some(pips);
        self
    }

    /// Whether to wait for the exchange to assign an order ID (default: true).
    #[must_use]
    pub fn wait_for_order_id(mut self, wait: bool) -> Self {
        self.wait_for_order_id = Some(wait);
        self
    }

    fn to_request(&self) -> OrderRequest {
        OrderRequest {
            exch_sym: self.exch_sym.clone(),
            side: self.side,
            quantity: self.quantity,
            order_type: self.order_type,
            duration: self.duration,
            limit_price: self.limit_price,
            stop_price: self.stop_price,
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
            stop_loss_offset: self.stop_loss_offset,
            take_profit_offset: self.take_profit_offset,
            trailing_stop: self.trailing_stop,
            wait_for_order_id: self.wait_for_order_id,
        }
    }
}

// ---------------------------------------------------------------------------
// OrderUpdate — fields that can be changed on an existing order
// ---------------------------------------------------------------------------

/// Fields to update on an existing order.
///
/// Only `quantity` is required; price fields are optional.
#[derive(Debug, Clone)]
pub struct OrderUpdate {
    /// New quantity.
    pub quantity: i32,
    /// New limit price.
    pub limit_price: Option<f64>,
    /// New stop price.
    pub stop_price: Option<f64>,
    /// New stop loss price.
    pub stop_loss: Option<f64>,
    /// New take profit price.
    pub take_profit: Option<f64>,
    /// New stop loss offset in pips.
    pub stop_loss_offset: Option<f32>,
    /// New take profit offset in pips.
    pub take_profit_offset: Option<f32>,
}

impl OrderUpdate {
    /// Create an update with a new quantity.
    #[must_use]
    pub fn new(quantity: i32) -> Self {
        Self {
            quantity,
            limit_price: None,
            stop_price: None,
            stop_loss: None,
            take_profit: None,
            stop_loss_offset: None,
            take_profit_offset: None,
        }
    }

    fn to_request(&self, order_id: &str) -> OrderUpdateRequest {
        OrderUpdateRequest {
            order_id: order_id.to_string(),
            quantity: self.quantity,
            limit_price: self.limit_price,
            stop_price: self.stop_price,
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
            stop_loss_offset: self.stop_loss_offset,
            take_profit_offset: self.take_profit_offset,
        }
    }
}

// ---------------------------------------------------------------------------
// Client methods
// ---------------------------------------------------------------------------

impl<H: HttpTransport> Client<H> {
    /// Place a new order.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials, OrderBuilder};
    /// # use ironbeam_rs::types::{OrderSide, DurationType};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let order = OrderBuilder::limit("XCME:ES.U16", OrderSide::Buy, 1.0, 4500.0, DurationType::Day);
    /// let resp = client.place_order("ACC001", &order).await?;
    /// println!("Order ID: {:?}", resp.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_order(
        &self,
        account_id: &str,
        order: &OrderBuilder,
    ) -> Result<OrderBaseResponse> {
        let account_id = urlencoding::encode(account_id);
        let req = order.to_request();
        self.post(&format!("/order/{account_id}/place"), &req).await
    }

    /// Update an existing order.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials, OrderUpdate};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let update = OrderUpdate::new(2);
    /// let orders = client.update_order("ACC001", "ORD001", &update).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn update_order(
        &self,
        account_id: &str,
        order_id: &str,
        update: &OrderUpdate,
    ) -> Result<Vec<Order>> {
        let account_id = urlencoding::encode(account_id);
        let order_id_enc = urlencoding::encode(order_id);
        let req = update.to_request(order_id);
        let resp: OrdersResponse = self
            .put(
                &format!("/order/{account_id}/update/{order_id_enc}"),
                &req,
            )
            .await?;
        Ok(resp.orders)
    }

    /// Cancel a single order.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let orders = client.cancel_order("ACC001", "ORD001").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_order(&self, account_id: &str, order_id: &str) -> Result<Vec<Order>> {
        let account_id = urlencoding::encode(account_id);
        let order_id = urlencoding::encode(order_id);
        let resp: OrdersResponse = self
            .delete(&format!("/order/{account_id}/cancel/{order_id}"))
            .await?;
        Ok(resp.orders)
    }

    /// Cancel multiple orders.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let orders = client.cancel_orders("ACC001", &["ORD001", "ORD002"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_orders(
        &self,
        account_id: &str,
        order_ids: &[&str],
    ) -> Result<Vec<Order>> {
        if order_ids.is_empty() {
            return Err(Error::Other("order_ids must not be empty".into()));
        }
        let account_id_enc = urlencoding::encode(account_id);
        let req = OrderCancelMultipleRequest {
            account_id: account_id.to_string(),
            order_ids: order_ids.iter().map(|s| s.to_string()).collect(),
        };
        let resp: OrdersResponse = self
            .delete_with_body(
                &format!("/order/{account_id_enc}/cancelMultiple"),
                &req,
            )
            .await?;
        Ok(resp.orders)
    }

    /// Get orders by status.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::OrderStatusType;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let orders = client.orders("ACC001", OrderStatusType::Any).await?;
    /// for o in &orders {
    ///     println!("{}: {} {:?} {:?}", o.order_id, o.exch_sym, o.side, o.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn orders(
        &self,
        account_id: &str,
        status: OrderStatusType,
    ) -> Result<Vec<Order>> {
        let account_id = urlencoding::encode(account_id);
        let status_str =
            serde_json::to_value(status).map_err(|e| Error::Other(e.to_string()))?;
        let status_str = status_str.as_str().unwrap_or("ANY");
        let resp: OrdersResponse = self
            .get(&format!("/order/{account_id}/{status_str}"))
            .await?;
        Ok(resp.orders)
    }

    /// Get order fills for an account.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let fills = client.order_fills("ACC001").await?;
    /// for f in &fills {
    ///     println!("{}: {} {:?} @ {:?}", f.order_id, f.exch_sym, f.fill_quantity, f.fill_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_fills(&self, account_id: &str) -> Result<Vec<OrderFill>> {
        let account_id = urlencoding::encode(account_id);
        let resp: OrdersFillsResponse =
            self.get(&format!("/order/{account_id}/fills")).await?;
        Ok(resp.fills)
    }

    /// Convert a strategy ID to an order ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let resp = client.order_id_from_strategy("ACC001", 12345).await?;
    /// println!("Order ID: {:?}", resp.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn order_id_from_strategy(
        &self,
        account_id: &str,
        strategy_id: i64,
    ) -> Result<OrderBaseResponse> {
        let account_id = urlencoding::encode(account_id);
        self.get(&format!("/order/{account_id}/toorderid/{strategy_id}"))
            .await
    }

    /// Convert an order ID to a strategy ID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let resp = client.strategy_id_from_order("ACC001", "ORD001").await?;
    /// println!("Strategy ID: {:?}", resp.strategy_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn strategy_id_from_order(
        &self,
        account_id: &str,
        order_id: &str,
    ) -> Result<OrderBaseResponse> {
        let account_id = urlencoding::encode(account_id);
        let order_id = urlencoding::encode(order_id);
        self.get(&format!(
            "/order/{account_id}/tostrategyId/{order_id}"
        ))
        .await
    }
}

#[cfg(test)]
mod tests {
    use hyper::Method;
    use hyper::StatusCode;
    use hyper::header::AUTHORIZATION;

    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::client::test_support::test_client_with_auth;
    use crate::error::Error;
    use crate::types::{DurationType, OrderSide, OrderStatusType, OrderType};

    use super::{OrderBuilder, OrderUpdate};

    // --- OrderBuilder ---

    #[test]
    fn builder_market_sets_fields() {
        let order = OrderBuilder::market("XCME:ES.U16", OrderSide::Buy, 1.0, DurationType::Day);
        let req = order.to_request();
        assert_eq!(req.exch_sym, "XCME:ES.U16");
        assert_eq!(req.side, OrderSide::Buy);
        assert_eq!(req.quantity, 1.0);
        assert_eq!(req.order_type, OrderType::Market);
        assert_eq!(req.duration, DurationType::Day);
        assert!(req.limit_price.is_none());
        assert!(req.stop_price.is_none());
    }

    #[test]
    fn builder_limit_sets_price() {
        let order =
            OrderBuilder::limit("XCME:ES.U16", OrderSide::Sell, 2.0, 4500.0, DurationType::Day);
        let req = order.to_request();
        assert_eq!(req.order_type, OrderType::Limit);
        assert_eq!(req.limit_price, Some(4500.0));
        assert!(req.stop_price.is_none());
    }

    #[test]
    fn builder_stop_sets_price() {
        let order =
            OrderBuilder::stop("XCME:ES.U16", OrderSide::Sell, 1.0, 4400.0, DurationType::Day);
        let req = order.to_request();
        assert_eq!(req.order_type, OrderType::Stop);
        assert_eq!(req.stop_price, Some(4400.0));
        assert!(req.limit_price.is_none());
    }

    #[test]
    fn builder_stop_limit_sets_both_prices() {
        let order = OrderBuilder::stop_limit(
            "XCME:ES.U16",
            OrderSide::Buy,
            1.0,
            4500.0,
            4400.0,
            DurationType::GoodTillCancel,
        );
        let req = order.to_request();
        assert_eq!(req.order_type, OrderType::StopLimit);
        assert_eq!(req.limit_price, Some(4500.0));
        assert_eq!(req.stop_price, Some(4400.0));
        assert_eq!(req.duration, DurationType::GoodTillCancel);
    }

    #[test]
    fn builder_optional_setters() {
        let order = OrderBuilder::market("XCME:ES.U16", OrderSide::Buy, 1.0, DurationType::Day)
            .stop_loss(4480.0)
            .take_profit(4550.0)
            .wait_for_order_id(false);
        let req = order.to_request();
        assert_eq!(req.stop_loss, Some(4480.0));
        assert_eq!(req.take_profit, Some(4550.0));
        assert_eq!(req.wait_for_order_id, Some(false));
    }

    // --- place_order ---

    #[tokio::test]
    async fn place_order_returns_response() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"orderId":"ORD001","strategyId":100}"#,
        )]);
        let client = test_client_with_auth(mock);

        let order = OrderBuilder::market("XCME:ES.U16", OrderSide::Buy, 1.0, DurationType::Day);
        let resp = client.place_order("ACC1", &order).await.unwrap();

        assert_eq!(resp.order_id.as_deref(), Some("ORD001"));
        assert_eq!(resp.strategy_id, Some(100));
    }

    #[tokio::test]
    async fn place_order_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"orderId":"ORD001"}"#)]);
        let client = test_client_with_auth(mock);

        let order =
            OrderBuilder::limit("XCME:ES.U16", OrderSide::Buy, 1.0, 4500.0, DurationType::Day);
        client.place_order("ACC1", &order).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::POST);
        assert!(reqs[0].uri.to_string().ends_with("/order/ACC1/place"));
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["exchSym"], "XCME:ES.U16");
        assert_eq!(body["side"], "BUY");
        assert_eq!(body["orderType"], "2");
        assert_eq!(body["limitPrice"], 4500.0);
    }

    #[tokio::test]
    async fn place_order_omits_unset_optional_fields() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"orderId":"ORD001"}"#)]);
        let client = test_client_with_auth(mock);

        let order = OrderBuilder::market("XCME:ES.U16", OrderSide::Buy, 1.0, DurationType::Day);
        client.place_order("ACC1", &order).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert!(body.get("limitPrice").is_none());
        assert!(body.get("stopPrice").is_none());
        assert!(body.get("stopLoss").is_none());
        assert!(body.get("takeProfit").is_none());
        assert!(body.get("waitForOrderId").is_none());
    }

    // --- update_order ---

    #[tokio::test]
    async fn update_order_returns_orders() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"orders":[{"orderId":"ORD001","accountId":"ACC1","exchSym":"XCME:ES.U16","status":"NEW","side":"BUY","quantity":2.0,"orderType":"2","duration":"0"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let update = OrderUpdate::new(2);
        let orders = client.update_order("ACC1", "ORD001", &update).await.unwrap();

        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_id, "ORD001");
    }

    #[tokio::test]
    async fn update_order_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"orders":[]}"#)]);
        let client = test_client_with_auth(mock);

        let mut update = OrderUpdate::new(5);
        update.limit_price = Some(4600.0);
        update.stop_price = Some(4400.0);
        client
            .update_order("ACC1", "ORD001", &update)
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::PUT);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/order/ACC1/update/ORD001"));
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["orderId"], "ORD001");
        assert_eq!(body["quantity"], 5);
        assert_eq!(body["limitPrice"], 4600.0);
        assert_eq!(body["stopPrice"], 4400.0);
    }

    // --- cancel_order ---

    #[tokio::test]
    async fn cancel_order_sends_delete() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"orders":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.cancel_order("ACC1", "ORD001").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::DELETE);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/order/ACC1/cancel/ORD001"));
    }

    // --- cancel_orders ---

    #[tokio::test]
    async fn cancel_orders_sends_delete_with_body() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"orders":[]}"#)]);
        let client = test_client_with_auth(mock);

        client
            .cancel_orders("ACC1", &["ORD001", "ORD002"])
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::DELETE);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/order/ACC1/cancelMultiple"));
        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["accountId"], "ACC1");
        assert_eq!(body["orderIds"], serde_json::json!(["ORD001", "ORD002"]));
    }

    #[tokio::test]
    async fn cancel_orders_rejects_empty() {
        let mock = MockHttp::new(vec![]);
        let client = test_client_with_auth(mock);

        let err = client.cancel_orders("ACC1", &[]).await.unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("empty")));
    }

    // --- orders ---

    #[tokio::test]
    async fn orders_returns_list() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"orders":[{"orderId":"ORD001","accountId":"ACC1","exchSym":"XCME:ES.U16","status":"NEW","side":"BUY","quantity":1.0,"orderType":"1","duration":"0"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let orders = client.orders("ACC1", OrderStatusType::Any).await.unwrap();

        assert_eq!(orders.len(), 1);
        assert_eq!(orders[0].order_id, "ORD001");
    }

    #[tokio::test]
    async fn orders_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"orders":[]}"#)]);
        let client = test_client_with_auth(mock);

        client
            .orders("ACC1", OrderStatusType::Filled)
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/order/ACC1/FILLED"));
    }

    // --- order_fills ---

    #[tokio::test]
    async fn order_fills_returns_fills() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"fills":[{"orderId":"ORD001","accountId":"ACC1","exchSym":"XCME:ES.U16"}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let fills = client.order_fills("ACC1").await.unwrap();

        assert_eq!(fills.len(), 1);
        assert_eq!(fills[0].order_id, "ORD001");
    }

    #[tokio::test]
    async fn order_fills_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"fills":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.order_fills("ACC1").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0].uri.to_string().ends_with("/order/ACC1/fills"));
    }

    // --- order_id_from_strategy ---

    #[tokio::test]
    async fn order_id_from_strategy_returns_response() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"orderId":"ORD001","strategyId":100}"#,
        )]);
        let client = test_client_with_auth(mock);

        let resp = client.order_id_from_strategy("ACC1", 100).await.unwrap();

        assert_eq!(resp.order_id.as_deref(), Some("ORD001"));
    }

    #[tokio::test]
    async fn order_id_from_strategy_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"orderId":"ORD001","strategyId":100}"#,
        )]);
        let client = test_client_with_auth(mock);

        client.order_id_from_strategy("ACC1", 100).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/order/ACC1/toorderid/100"));
    }

    // --- strategy_id_from_order ---

    #[tokio::test]
    async fn strategy_id_from_order_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"orderId":"ORD001","strategyId":100}"#,
        )]);
        let client = test_client_with_auth(mock);

        client
            .strategy_id_from_order("ACC1", "ORD001")
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        assert!(reqs[0]
            .uri
            .to_string()
            .ends_with("/order/ACC1/tostrategyId/ORD001"));
    }

    // --- cross-cutting ---

    #[tokio::test]
    async fn orders_sends_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"orders":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.orders("ACC1", OrderStatusType::Any).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    #[tokio::test]
    async fn orders_maps_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::BAD_REQUEST,
            r#"{"error1":"Invalid order"}"#,
        )]);
        let client = test_client_with_auth(mock);

        let err = client.orders("ACC1", OrderStatusType::Any).await.unwrap_err();

        match err {
            Error::Api { status, message } => {
                assert_eq!(status, 400);
                assert_eq!(message, "Invalid order");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }
}
