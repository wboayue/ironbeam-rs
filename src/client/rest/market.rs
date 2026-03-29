use time::OffsetDateTime;

use crate::client::Client;
use crate::client::http::HttpTransport;
use crate::error::{Error, Result};
use crate::types::{
    Depth, DepthResponse, QuoteFull, QuotesResponse, Trade, TradesResponse,
};

impl<H: HttpTransport> Client<H> {
    /// Get quotes for symbols (max 10).
    ///
    /// Symbols use `EXCHANGE:SYMBOL` format, e.g. `"XCME:ES.U16"`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let quotes = client.quotes(&["XCME:ES.U16"]).await?;
    /// for q in &quotes {
    ///     println!("{}: last={:?} bid={:?} ask={:?}", q.symbol, q.last_price, q.bid, q.ask);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn quotes(&self, symbols: &[&str]) -> Result<Vec<QuoteFull>> {
        let resp: QuotesResponse = self.symbol_query("/market/quotes", symbols).await?;
        Ok(resp.quotes)
    }

    /// Get market depth (order book) for symbols (max 10).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let depths = client.depth(&["XCME:ES.U16"]).await?;
    /// for d in &depths {
    ///     println!("{}: {} bids, {} asks", d.symbol, d.bids.len(), d.asks.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn depth(&self, symbols: &[&str]) -> Result<Vec<Depth>> {
        let resp: DepthResponse = self.symbol_query("/market/depth", symbols).await?;
        Ok(resp.depths)
    }

    /// Get historical trades for a symbol.
    ///
    /// Returns up to `max` trades (1–100) in the time range `from..to`.
    /// Set `earlier` to `true` to search backward from `to`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use ironbeam_rs::client::{Client, Credentials};
    /// # async fn example() -> ironbeam_rs::error::Result<()> {
    /// # let client = Client::builder()
    /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
    /// #     .connect().await?;
    /// let now = time::OffsetDateTime::now_utc();
    /// let hour_ago = now - time::Duration::hours(1);
    /// let trades = client.trades("XCME:ES.U16", hour_ago, now, 50, true).await?;
    /// for t in &trades {
    ///     println!("{}: {:?} @ {:?}", t.symbol, t.size, t.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn trades(
        &self,
        symbol: &str,
        from: OffsetDateTime,
        to: OffsetDateTime,
        max: u32,
        earlier: bool,
    ) -> Result<Vec<Trade>> {
        if max == 0 || max > 100 {
            return Err(Error::Other("max must be 1–100".into()));
        }
        let symbol = urlencoding::encode(symbol);
        let from_ms = from.unix_timestamp() * 1000 + from.millisecond() as i64;
        let to_ms = to.unix_timestamp() * 1000 + to.millisecond() as i64;
        let resp: TradesResponse = self
            .get(&format!(
                "/market/trades/{symbol}/{from_ms}/{to_ms}/{max}/{earlier}"
            ))
            .await?;
        Ok(resp.traders)
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

    // --- quotes ---

    #[tokio::test]
    async fn quotes_returns_results() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"Quotes":[{"s":"XCME:ES.U16","l":4500.0,"b":4499.75,"a":4500.25}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let quotes = client.quotes(&["XCME:ES.U16"]).await.unwrap();

        assert_eq!(quotes.len(), 1);
        assert_eq!(quotes[0].symbol, "XCME:ES.U16");
        assert_eq!(quotes[0].last_price, Some(4500.0));
    }

    #[tokio::test]
    async fn quotes_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"Quotes":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.quotes(&["XCME:ES.U16"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/market/quotes?symbols="));
        assert!(uri.contains("XCME%3AES.U16"));
    }

    #[tokio::test]
    async fn quotes_rejects_empty_symbols() {
        let mock = MockHttp::new(vec![]);
        let client = test_client_with_auth(mock);

        let err = client.quotes(&[]).await.unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("empty")));
    }

    // --- depth ---

    #[tokio::test]
    async fn depth_returns_results() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"Depths":[{"s":"XCME:ES.U16","b":[{"l":0,"s":"B","p":4499.75,"sz":10.0}],"a":[{"l":0,"s":"A","p":4500.25,"sz":5.0}]}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let depths = client.depth(&["XCME:ES.U16"]).await.unwrap();

        assert_eq!(depths.len(), 1);
        assert_eq!(depths[0].symbol, "XCME:ES.U16");
        assert_eq!(depths[0].bids.len(), 1);
        assert_eq!(depths[0].asks.len(), 1);
    }

    #[tokio::test]
    async fn depth_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"Depths":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.depth(&["XCME:ES.U16"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/market/depth?symbols="));
    }

    // --- trades ---

    #[tokio::test]
    async fn trades_returns_results() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"traders":[{"symbol":"XCME:ES.U16","price":4500.0,"size":1.0}]}"#,
        )]);
        let client = test_client_with_auth(mock);

        let from = time::OffsetDateTime::from_unix_timestamp(1700000000).unwrap();
        let to = time::OffsetDateTime::from_unix_timestamp(1700003600).unwrap();
        let trades = client
            .trades("XCME:ES.U16", from, to, 50, true)
            .await
            .unwrap();

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].symbol, "XCME:ES.U16");
        assert_eq!(trades[0].price, 4500.0);
    }

    #[tokio::test]
    async fn trades_sends_correct_uri() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"traders":[]}"#)]);
        let client = test_client_with_auth(mock);

        let from = time::OffsetDateTime::from_unix_timestamp(1700000000).unwrap();
        let to = time::OffsetDateTime::from_unix_timestamp(1700003600).unwrap();
        client
            .trades("XCME:ES.U16", from, to, 10, false)
            .await
            .unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(reqs[0].method, Method::GET);
        let uri = reqs[0].uri.to_string();
        assert!(uri.contains("/market/trades/XCME%3AES.U16/1700000000000/1700003600000/10/false"));
    }

    #[tokio::test]
    async fn trades_rejects_zero_max() {
        let mock = MockHttp::new(vec![]);
        let client = test_client_with_auth(mock);

        let from = time::OffsetDateTime::from_unix_timestamp(1700000000).unwrap();
        let to = time::OffsetDateTime::from_unix_timestamp(1700003600).unwrap();
        let err = client
            .trades("XCME:ES.U16", from, to, 0, true)
            .await
            .unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("1–100")));
    }

    #[tokio::test]
    async fn trades_rejects_over_100_max() {
        let mock = MockHttp::new(vec![]);
        let client = test_client_with_auth(mock);

        let from = time::OffsetDateTime::from_unix_timestamp(1700000000).unwrap();
        let to = time::OffsetDateTime::from_unix_timestamp(1700003600).unwrap();
        let err = client
            .trades("XCME:ES.U16", from, to, 101, true)
            .await
            .unwrap_err();
        assert!(matches!(err, Error::Other(msg) if msg.contains("1–100")));
    }

    // --- cross-cutting ---

    #[tokio::test]
    async fn market_sends_auth_header() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"Quotes":[]}"#)]);
        let client = test_client_with_auth(mock);

        client.quotes(&["XCME:ES.U16"]).await.unwrap();

        let reqs = client.request.http.recorded_requests();
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    #[tokio::test]
    async fn market_maps_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            StatusCode::NOT_FOUND,
            r#"{"error1":"Not Found"}"#,
        )]);
        let client = test_client_with_auth(mock);

        let err = client.quotes(&["XCME:ES.U16"]).await.unwrap_err();

        match err {
            Error::Api { status, message } => {
                assert_eq!(status, 404);
                assert_eq!(message, "Not Found");
            }
            other => panic!("expected Api error, got {other:?}"),
        }
    }
}
