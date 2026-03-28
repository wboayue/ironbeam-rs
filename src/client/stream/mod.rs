mod connection;
pub(crate) mod handler;
mod subscriptions;

use hyper::header::{AUTHORIZATION, HeaderMap};
use tokio::sync::{mpsc, watch};
use tokio::task::JoinHandle;

use crate::error::{Error, Result};
use crate::types::streaming::{IndicatorSubscribeResponse, StreamIdResponse, SubscribeBarsRequest};

use super::http::HttpTransport;
use super::Client;

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
        // 1. Create stream session via REST.
        let resp: StreamIdResponse = self.client.get("/stream/create").await?;
        let stream_id = resp.stream_id;

        // 2. Extract bearer token for WebSocket URL.
        let token = extract_token(&self.client.auth_headers)?;

        // 3. Open WebSocket connection.
        let ws = connection::connect(&self.client.base_url, &stream_id, &token).await?;

        // 4. Spawn message loop.
        let (tx, rx) = mpsc::channel(self.channel_capacity);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);
        let task = tokio::spawn(connection::message_loop(ws, tx, shutdown_rx));

        Ok(StreamHandle {
            stream_id,
            http: self.client.http.clone(),
            base_url: self.client.base_url.clone(),
            auth_headers: self.client.auth_headers.clone(),
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
    http: H,
    base_url: String,
    auth_headers: HeaderMap,
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
        let _ = self.shutdown_tx.send(true);
        if let Some(task) = self.task.take() {
            task.await.map_err(|e| Error::WebSocket(e.to_string()))?;
        }
        Ok(())
    }

    /// The stream session identifier.
    #[must_use]
    pub fn stream_id(&self) -> &str {
        &self.stream_id
    }

    // -- helpers ------------------------------------------------------------

    async fn sub_market(&self, feed: MarketFeed, symbols: &[&str]) -> Result<()> {
        subscriptions::subscribe_market(
            &self.http, &self.base_url, &self.auth_headers, feed, &self.stream_id, symbols,
        )
        .await
    }

    async fn unsub_market(&self, feed: MarketFeed, symbols: &[&str]) -> Result<()> {
        subscriptions::unsubscribe_market(
            &self.http, &self.base_url, &self.auth_headers, feed, &self.stream_id, symbols,
        )
        .await
    }

    async fn sub_indicator(
        &self,
        kind: BarKind,
        req: &SubscribeBarsRequest,
    ) -> Result<IndicatorSubscribeResponse> {
        subscriptions::subscribe_indicator(
            &self.http, &self.base_url, &self.auth_headers, kind, &self.stream_id, req,
        )
        .await
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
        subscriptions::unsubscribe_indicator(
            &self.http,
            &self.base_url,
            &self.auth_headers,
            &self.stream_id,
            indicator_id,
        )
        .await
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
            .field("base_url", &self.base_url)
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
}
