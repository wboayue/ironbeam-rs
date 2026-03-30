mod connection;
pub(crate) mod handler;
mod subscriptions;

use hyper::header::{AUTHORIZATION, HeaderMap};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;

use crate::error::{Error, Result};
use crate::types::streaming::{IndicatorSubscribeResponse, StreamIdResponse, SubscribeBarsRequest};

use super::http::HttpTransport;
use super::{Client, RequestHelper};

pub use handler::StreamEvent;
use subscriptions::{BarKind, MarketFeed};

const DEFAULT_CHANNEL_CAPACITY: usize = 256;

// ---------------------------------------------------------------------------
// StreamBuilder
// ---------------------------------------------------------------------------

/// Builder for creating a streaming WebSocket connection.
///
/// Obtained via [`Client::stream()`]. Call [`start()`](StreamBuilder::start)
/// to create the stream session and open the WebSocket.
///
/// # Example
///
/// ```no_run
/// # use ironbeam_rs::client::{Client, Credentials};
/// # async fn example() -> ironbeam_rs::error::Result<()> {
/// # let client = Client::builder()
/// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
/// #     .connect().await?;
/// let mut stream = client.stream().start().await?;
/// stream.subscribe_quotes(&["XCME:ES.U26"]).await?;
/// # Ok(())
/// # }
/// ```
pub struct StreamBuilder<'a, H: HttpTransport> {
    client: &'a Client<H>,
    channel_capacity: usize,
}

impl<'a, H: HttpTransport> StreamBuilder<'a, H> {
    /// Set the event channel capacity (default: 256).
    ///
    /// Too small and the message loop blocks on send (backpressure);
    /// too large and stale quotes may buffer before consumption.
    #[must_use]
    pub fn channel_capacity(mut self, capacity: usize) -> Self {
        self.channel_capacity = capacity;
        self
    }

    /// Create the stream session, open the WebSocket, and return a live
    /// [`StreamHandle`] for subscribing to feeds and receiving events.
    pub async fn start(self) -> Result<StreamHandle<H>> {
        let (stream_id, token) = self.create_session().await?;

        let ws = match connection::connect(&self.client.request.base_url, &stream_id, &token).await
        {
            Ok(ws) => ws,
            Err(e) => {
                tracing::warn!(stream_id = %stream_id, error = %e, "websocket connect failed after stream session created");
                return Err(e);
            }
        };
        tracing::info!(stream_id = %stream_id, "websocket connected");

        self.spawn_loop(stream_id, ws)
    }

    /// Start with a pre-built WebSocket transport (test seam).
    #[cfg(test)]
    pub(crate) async fn start_with_ws<W: connection::WsTransport>(
        self,
        ws: W,
    ) -> Result<StreamHandle<H>> {
        let (stream_id, _token) = self.create_session().await?;
        self.spawn_loop(stream_id, ws)
    }

    /// Create the stream session via REST and extract the bearer token.
    async fn create_session(&self) -> Result<(String, String)> {
        let resp: StreamIdResponse = self.client.get("/stream/create").await?;
        let stream_id = resp.stream_id;
        tracing::info!(stream_id = %stream_id, "stream session created");
        let token = extract_token(&self.client.request.auth_headers)?;
        Ok((stream_id, token))
    }

    /// Spawn the message loop and build the [`StreamHandle`].
    fn spawn_loop<W: connection::WsTransport>(
        self,
        stream_id: String,
        ws: W,
    ) -> Result<StreamHandle<H>> {
        let (tx, rx) = mpsc::channel(self.channel_capacity);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let task = tokio::spawn(connection::message_loop(
            ws,
            tx,
            shutdown_rx,
            stream_id.clone(),
        ));

        Ok(StreamHandle {
            stream_id,
            request: self.client.request.clone(),
            rx,
            shutdown_tx,
            task: Some(task),
        })
    }
}

impl<H: HttpTransport> Client<H> {
    /// Begin building a streaming WebSocket connection.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let mut stream = client.stream().start().await?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn stream(&self) -> StreamBuilder<'_, H> {
        StreamBuilder {
            client: self,
            channel_capacity: DEFAULT_CHANNEL_CAPACITY,
        }
    }
}

// ---------------------------------------------------------------------------
// StreamHandle
// ---------------------------------------------------------------------------

/// A live streaming connection.
///
/// Receives [`StreamEvent`]s via [`next()`](StreamHandle::next) and manages
/// subscriptions via `subscribe_*` / `unsubscribe_*` methods.
///
/// # Example
///
/// ```no_run
/// # use ironbeam_rs::client::{Client, Credentials};
/// # use ironbeam_rs::client::stream::StreamEvent;
/// # async fn example() -> ironbeam_rs::error::Result<()> {
/// # let client = Client::builder()
/// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
/// #     .connect().await?;
/// let mut stream = client.stream().start().await?;
/// stream.subscribe_quotes(&["XCME:ES.U26"]).await?;
///
/// while let Some(event) = stream.next().await {
///     match event? {
///         StreamEvent::Quotes(quotes) => println!("{quotes:?}"),
///         _ => {}
///     }
/// }
/// # Ok(())
/// # }
/// ```
pub struct StreamHandle<H: HttpTransport> {
    stream_id: String,
    request: RequestHelper<H>,
    rx: mpsc::Receiver<Result<StreamEvent>>,
    shutdown_tx: watch::Sender<bool>,
    task: Option<JoinHandle<()>>,
}

impl<H: HttpTransport> StreamHandle<H> {
    /// Receive the next streaming event.
    ///
    /// Returns `None` when the stream is closed.
    pub async fn next(&mut self) -> Option<Result<StreamEvent>> {
        self.rx.recv().await
    }

    /// Gracefully close the stream and await the message loop.
    pub async fn close(mut self) -> Result<()> {
        tracing::info!(stream_id = %self.stream_id, "closing stream");
        let _ = self.shutdown_tx.send(true);
        if let Some(task) = self.task.take() {
            task.await.map_err(|e| Error::WebSocket(e.to_string()))?;
        }
        tracing::info!(stream_id = %self.stream_id, "stream closed");
        Ok(())
    }

    /// The stream session identifier.
    #[must_use]
    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    // -- helpers ------------------------------------------------------------

    async fn sub_market(&self, feed: MarketFeed, symbols: &[&str]) -> Result<()> {
        tracing::info!(stream_id = %self.stream_id, feed = feed.as_str(), ?symbols, "subscribing");
        subscriptions::subscribe_market(&self.request, feed, &self.stream_id, symbols).await
    }

    async fn unsub_market(&self, feed: MarketFeed, symbols: &[&str]) -> Result<()> {
        tracing::info!(stream_id = %self.stream_id, feed = feed.as_str(), ?symbols, "unsubscribing");
        subscriptions::unsubscribe_market(&self.request, feed, &self.stream_id, symbols).await
    }

    async fn sub_indicator(
        &self,
        kind: BarKind,
        req: &SubscribeBarsRequest,
    ) -> Result<IndicatorSubscribeResponse> {
        tracing::info!(stream_id = %self.stream_id, kind = kind.as_str(), symbol = %req.symbol, "subscribing indicator");
        subscriptions::subscribe_indicator(&self.request, kind, &self.stream_id, req).await
    }

    // -- Market data subscriptions ------------------------------------------

    /// Subscribe to quote updates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// # let mut stream = client.stream().start().await?;
    /// stream.subscribe_quotes(&["XCME:ES.U26", "XCME:NQ.U26"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe_quotes(&self, symbols: &[&str]) -> Result<()> {
        self.sub_market(MarketFeed::Quotes, symbols).await
    }

    /// Unsubscribe from quote updates.
    pub async fn unsubscribe_quotes(&self, symbols: &[&str]) -> Result<()> {
        self.unsub_market(MarketFeed::Quotes, symbols).await
    }

    /// Subscribe to depth (order book) updates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// # let mut stream = client.stream().start().await?;
    /// stream.subscribe_depth(&["XCME:ES.U26"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe_depth(&self, symbols: &[&str]) -> Result<()> {
        self.sub_market(MarketFeed::Depths, symbols).await
    }

    /// Unsubscribe from depth updates.
    pub async fn unsubscribe_depth(&self, symbols: &[&str]) -> Result<()> {
        self.unsub_market(MarketFeed::Depths, symbols).await
    }

    /// Subscribe to trade updates.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// # let mut stream = client.stream().start().await?;
    /// stream.subscribe_trades(&["XCME:ES.U26"]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe_trades(&self, symbols: &[&str]) -> Result<()> {
        self.sub_market(MarketFeed::Trades, symbols).await
    }

    /// Unsubscribe from trade updates.
    pub async fn unsubscribe_trades(&self, symbols: &[&str]) -> Result<()> {
        self.unsub_market(MarketFeed::Trades, symbols).await
    }

    // -- Indicator subscriptions --------------------------------------------

    /// Subscribe to trade bar indicators.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # use ironbeam_rs::types::streaming::SubscribeBarsRequest;
    /// # use ironbeam_rs::types::BarType;
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// # let mut stream = client.stream().start().await?;
    /// let resp = stream.subscribe_trade_bars(&SubscribeBarsRequest {
    ///     symbol: "XCME:ES.U26".into(),
    ///     period: 1,
    ///     bar_type: BarType::Minute,
    ///     load_size: 100,
    /// }).await?;
    /// println!("indicator_id: {}", resp.indicator_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe_trade_bars(
        &self,
        req: &SubscribeBarsRequest,
    ) -> Result<IndicatorSubscribeResponse> {
        self.sub_indicator(BarKind::Trade, req).await
    }

    /// Subscribe to tick bar indicators.
    pub async fn subscribe_tick_bars(
        &self,
        req: &SubscribeBarsRequest,
    ) -> Result<IndicatorSubscribeResponse> {
        self.sub_indicator(BarKind::Tick, req).await
    }

    /// Subscribe to time bar indicators.
    pub async fn subscribe_time_bars(
        &self,
        req: &SubscribeBarsRequest,
    ) -> Result<IndicatorSubscribeResponse> {
        self.sub_indicator(BarKind::Time, req).await
    }

    /// Subscribe to volume bar indicators.
    pub async fn subscribe_volume_bars(
        &self,
        req: &SubscribeBarsRequest,
    ) -> Result<IndicatorSubscribeResponse> {
        self.sub_indicator(BarKind::Volume, req).await
    }

    /// Unsubscribe from an indicator by its identifier.
    ///
    /// The `indicator_id` is returned by the `subscribe_*_bars` methods.
    pub async fn unsubscribe_indicator(&self, indicator_id: &str) -> Result<()> {
        tracing::info!(stream_id = %self.stream_id, indicator_id, "unsubscribing indicator");
        subscriptions::unsubscribe_indicator(&self.request, &self.stream_id, indicator_id).await
    }
}

/// Best-effort shutdown on drop. Signals the message loop to close but
/// cannot await completion. Prefer [`StreamHandle::close()`] for guaranteed cleanup.
impl<H: HttpTransport> Drop for StreamHandle<H> {
    fn drop(&mut self) {
        let _ = self.shutdown_tx.send(true);
    }
}

impl<H: HttpTransport> std::fmt::Debug for StreamHandle<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StreamHandle")
            .field("stream_id", &self.stream_id)
            .field("base_url", &self.request.base_url)
            .finish()
    }
}

/// Extract the bearer token from auth headers.
fn extract_token(headers: &HeaderMap) -> Result<String> {
    let value = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| Error::Auth("missing authorization header".into()))?
        .to_str()
        .map_err(|e| Error::Auth(e.to_string()))?;

    value
        .strip_prefix("Bearer ")
        .map(|t| t.to_owned())
        .ok_or_else(|| Error::Auth("invalid authorization header format".into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::http::mock::{MockHttp, MockResponse};
    use crate::client::test_support::test_client_with_auth;
    use connection::mock::MockWsTransport;

    #[test]
    fn extract_token_strips_bearer() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Bearer my_token".parse().unwrap());
        assert_eq!(extract_token(&headers).unwrap(), "my_token");
    }

    #[test]
    fn extract_token_missing_header() {
        let headers = HeaderMap::new();
        assert!(extract_token(&headers).is_err());
    }

    #[test]
    fn extract_token_invalid_format() {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, "Basic abc".parse().unwrap());
        assert!(extract_token(&headers).is_err());
    }

    #[tokio::test]
    async fn start_creates_session_and_streams_events() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","streamId":"s-123"}"#,
        )]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::from_json(&[r#"{"q":[{"s":"XCME:ES.U25"}]}"#]);

        let mut stream = client.stream().start_with_ws(ws).await.unwrap();

        assert_eq!(stream.stream_id(), "s-123");

        let event = stream.next().await.unwrap().unwrap();
        assert!(matches!(event, StreamEvent::Quotes(..)));

        // Verify REST call was made
        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs.len(), 1);
        assert!(reqs[0].uri.to_string().contains("/stream/create"));
    }

    #[tokio::test]
    async fn start_propagates_create_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            hyper::StatusCode::INTERNAL_SERVER_ERROR,
            r#"{"error1":"Server Error"}"#,
        )]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::from_json(&[]);

        let err = client.stream().start_with_ws(ws).await.unwrap_err();
        assert!(matches!(err, Error::Api { status: 500, .. }));
    }

    #[tokio::test]
    async fn stream_close_signals_shutdown() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","streamId":"s-456"}"#,
        )]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.close().await.unwrap();
    }

    #[tokio::test]
    async fn channel_capacity_is_applied() {
        // 2 responses: stream/create + subscribe_quotes
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-cap"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::from_json(&[r#"{"q":[{"s":"SYM"}]}"#]);

        let mut stream = client
            .stream()
            .channel_capacity(4)
            .start_with_ws(ws)
            .await
            .unwrap();

        let event = stream.next().await.unwrap().unwrap();
        assert!(matches!(event, StreamEvent::Quotes(..)));
    }

    #[tokio::test]
    async fn subscribe_quotes_via_handle() {
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.subscribe_quotes(&["XCME:ES.U25"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs.len(), 2);
        assert!(reqs[1].uri.to_string().contains("/market/quotes/subscribe/s-1"));
    }

    #[tokio::test]
    async fn unsubscribe_quotes_via_handle() {
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.unsubscribe_quotes(&["XCME:ES.U25"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/market/quotes/unsubscribe/s-1"));
    }

    #[tokio::test]
    async fn subscribe_depth_via_handle() {
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.subscribe_depth(&["XCME:ES.U25"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/market/depths/subscribe/s-1"));
    }

    #[tokio::test]
    async fn unsubscribe_depth_via_handle() {
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.unsubscribe_depth(&["XCME:ES.U25"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/market/depths/unsubscribe/s-1"));
    }

    #[tokio::test]
    async fn subscribe_trades_via_handle() {
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.subscribe_trades(&["XCME:ES.U25"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/market/trades/subscribe/s-1"));
    }

    #[tokio::test]
    async fn unsubscribe_trades_via_handle() {
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.unsubscribe_trades(&["XCME:ES.U25"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/market/trades/unsubscribe/s-1"));
    }

    #[tokio::test]
    async fn subscribe_trade_bars_via_handle() {
        use crate::types::{BarType, streaming::SubscribeBarsRequest};

        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(
                r#"{"indicatorId":"IND1","valueNames":["date"],"valueTypes":["date"]}"#,
            ),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        let resp = stream
            .subscribe_trade_bars(&SubscribeBarsRequest {
                symbol: "XCME:ES.U25".into(),
                period: 1,
                bar_type: BarType::Minute,
                load_size: 100,
            })
            .await
            .unwrap();

        assert_eq!(resp.indicator_id, "IND1");
        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/indicator/s-1/tradeBars/subscribe"));
    }

    #[tokio::test]
    async fn subscribe_tick_bars_via_handle() {
        use crate::types::{BarType, streaming::SubscribeBarsRequest};

        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(
                r#"{"indicatorId":"IND2","valueNames":["date"],"valueTypes":["date"]}"#,
            ),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        let resp = stream
            .subscribe_tick_bars(&SubscribeBarsRequest {
                symbol: "SYM".into(),
                period: 5,
                bar_type: BarType::Tick,
                load_size: 50,
            })
            .await
            .unwrap();

        assert_eq!(resp.indicator_id, "IND2");
        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/indicator/s-1/tickBars/subscribe"));
    }

    #[tokio::test]
    async fn subscribe_time_bars_via_handle() {
        use crate::types::{BarType, streaming::SubscribeBarsRequest};

        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(
                r#"{"indicatorId":"IND3","valueNames":["date"],"valueTypes":["date"]}"#,
            ),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        let resp = stream
            .subscribe_time_bars(&SubscribeBarsRequest {
                symbol: "SYM".into(),
                period: 1,
                bar_type: BarType::Minute,
                load_size: 10,
            })
            .await
            .unwrap();

        assert_eq!(resp.indicator_id, "IND3");
        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/indicator/s-1/timeBars/subscribe"));
    }

    #[tokio::test]
    async fn subscribe_volume_bars_via_handle() {
        use crate::types::{BarType, streaming::SubscribeBarsRequest};

        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(
                r#"{"indicatorId":"IND4","valueNames":["date"],"valueTypes":["date"]}"#,
            ),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        let resp = stream
            .subscribe_volume_bars(&SubscribeBarsRequest {
                symbol: "SYM".into(),
                period: 100,
                bar_type: BarType::Tick,
                load_size: 20,
            })
            .await
            .unwrap();

        assert_eq!(resp.indicator_id, "IND4");
        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/indicator/s-1/volumeBars/subscribe"));
    }

    #[tokio::test]
    async fn unsubscribe_indicator_via_handle() {
        let mock = MockHttp::new(vec![
            MockResponse::ok(r#"{"status":"OK","streamId":"s-1"}"#),
            MockResponse::ok(r#"{"status":"OK"}"#),
        ]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        stream.unsubscribe_indicator("IND1").await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert!(reqs[1].uri.to_string().contains("/indicator/s-1/unsubscribe/IND1"));
    }

    #[tokio::test]
    async fn stream_handle_debug() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"OK","streamId":"s-dbg"}"#,
        )]);
        let client = test_client_with_auth(mock);
        let ws = MockWsTransport::new(vec![]);

        let stream = client.stream().start_with_ws(ws).await.unwrap();
        let debug = format!("{stream:?}");
        assert!(debug.contains("s-dbg"));
        assert!(debug.contains("http://test"));
    }
}
