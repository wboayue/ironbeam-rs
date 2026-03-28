use crate::client::RequestHelper;
use crate::client::http::HttpTransport;
use crate::error::{Error, Result};
use crate::types::common::SuccessResponse;
use crate::types::streaming::{IndicatorSubscribeResponse, SubscribeBarsRequest};

/// Market data feed type for subscribe/unsubscribe.
pub(crate) enum MarketFeed {
    Quotes,
    Depths,
    Trades,
}

impl MarketFeed {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            MarketFeed::Quotes => "quotes",
            MarketFeed::Depths => "depths",
            MarketFeed::Trades => "trades",
        }
    }
}

/// Indicator bar type for subscribe endpoints.
pub(crate) enum BarKind {
    Trade,
    Tick,
    Time,
    Volume,
}

impl BarKind {
    pub(crate) fn as_str(&self) -> &'static str {
        match self {
            BarKind::Trade => "tradeBars",
            BarKind::Tick => "tickBars",
            BarKind::Time => "timeBars",
            BarKind::Volume => "volumeBars",
        }
    }
}

/// `GET /market/{feed}/{action}/{streamId}?symbols=SYM1,SYM2`
async fn market_request<H: HttpTransport>(
    request: &RequestHelper<H>,
    feed: MarketFeed,
    action: &str,
    stream_id: &str,
    symbols: &[&str],
) -> Result<()> {
    let symbols_param: String = symbols
        .iter()
        .map(|s| urlencoding::encode(s))
        .collect::<Vec<_>>()
        .join(",");
    let path = format!(
        "/market/{}/{action}/{stream_id}?symbols={symbols_param}",
        feed.as_str()
    );

    let resp: SuccessResponse = request.get(&path).await?;
    if resp.status != crate::types::ResponseStatus::Ok {
        return Err(Error::Api {
            status: 200,
            message: resp.message.unwrap_or_default(),
        });
    }

    Ok(())
}

/// `GET /market/{feed}/subscribe/{streamId}?symbols=SYM1,SYM2`
pub(crate) async fn subscribe_market<H: HttpTransport>(
    request: &RequestHelper<H>,
    feed: MarketFeed,
    stream_id: &str,
    symbols: &[&str],
) -> Result<()> {
    market_request(request, feed, "subscribe", stream_id, symbols).await
}

/// `GET /market/{feed}/unsubscribe/{streamId}?symbols=SYM1,SYM2`
pub(crate) async fn unsubscribe_market<H: HttpTransport>(
    request: &RequestHelper<H>,
    feed: MarketFeed,
    stream_id: &str,
    symbols: &[&str],
) -> Result<()> {
    market_request(request, feed, "unsubscribe", stream_id, symbols).await
}

/// `POST /indicator/{streamId}/{barKind}/subscribe`
pub(crate) async fn subscribe_indicator<H: HttpTransport>(
    request: &RequestHelper<H>,
    kind: BarKind,
    stream_id: &str,
    req: &SubscribeBarsRequest,
) -> Result<IndicatorSubscribeResponse> {
    let path = format!("/indicator/{stream_id}/{}/subscribe", kind.as_str());
    request.post(&path, req).await
}

/// `DELETE /indicator/{streamId}/unsubscribe/{indicatorId}`
pub(crate) async fn unsubscribe_indicator<H: HttpTransport>(
    request: &RequestHelper<H>,
    stream_id: &str,
    indicator_id: &str,
) -> Result<()> {
    let path = format!("/indicator/{stream_id}/unsubscribe/{indicator_id}");
    let _resp: SuccessResponse = request.delete(&path).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use hyper::header::{AUTHORIZATION, HeaderMap, HeaderValue};

    use crate::client::RequestHelper;
    use crate::client::http::mock::{MockHttp, MockResponse};

    use super::*;

    fn test_request(mock: MockHttp) -> RequestHelper<MockHttp> {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer tok_test"));
        RequestHelper {
            http: mock,
            base_url: "http://test".into(),
            auth_headers: headers,
        }
    }

    #[tokio::test]
    async fn subscribe_quotes_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let req = test_request(mock);
        subscribe_market(
            &req,
            MarketFeed::Quotes,
            "stream-123",
            &["XCME:ES.U25", "XCME:NQ.U25"],
        )
        .await
        .unwrap();

        let reqs = req.http.recorded_requests();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].method, hyper::Method::GET);
        assert_eq!(
            reqs[0].uri.to_string(),
            "http://test/market/quotes/subscribe/stream-123?symbols=XCME%3AES.U25,XCME%3ANQ.U25"
        );
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    #[tokio::test]
    async fn unsubscribe_depths_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let req = test_request(mock);
        unsubscribe_market(&req, MarketFeed::Depths, "stream-456", &["XCME:ES.U25"])
            .await
            .unwrap();

        let reqs = req.http.recorded_requests();
        assert_eq!(reqs[0].method, hyper::Method::GET);
        assert!(
            reqs[0]
                .uri
                .to_string()
                .contains("/market/depths/unsubscribe/stream-456")
        );
    }

    #[tokio::test]
    async fn subscribe_trades_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let req = test_request(mock);
        subscribe_market(&req, MarketFeed::Trades, "s1", &["SYM"])
            .await
            .unwrap();

        let reqs = req.http.recorded_requests();
        assert!(
            reqs[0]
                .uri
                .to_string()
                .contains("/market/trades/subscribe/s1")
        );
    }

    #[tokio::test]
    async fn subscribe_indicator_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"indicatorId":"IND1","valueNames":["date","open"],"valueTypes":["date","number"]}"#,
        )]);
        let request = test_request(mock);

        let bar_req = SubscribeBarsRequest {
            symbol: "XCME:ES.U25".into(),
            period: 1,
            bar_type: crate::types::BarType::Minute,
            load_size: 100,
        };

        let resp = subscribe_indicator(&request, BarKind::Trade, "stream-789", &bar_req)
            .await
            .unwrap();

        assert_eq!(resp.indicator_id, "IND1");

        let reqs = request.http.recorded_requests();
        assert_eq!(reqs[0].method, hyper::Method::POST);
        assert!(
            reqs[0]
                .uri
                .to_string()
                .contains("/indicator/stream-789/tradeBars/subscribe")
        );

        let body: serde_json::Value = serde_json::from_slice(&reqs[0].body).unwrap();
        assert_eq!(body["symbol"], "XCME:ES.U25");
        assert_eq!(body["period"], 1);
        assert_eq!(body["barType"], "MINUTE");
        assert_eq!(body["loadSize"], 100);
    }

    #[tokio::test]
    async fn unsubscribe_indicator_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        let req = test_request(mock);
        unsubscribe_indicator(&req, "stream-1", "IND-ABC")
            .await
            .unwrap();

        let reqs = req.http.recorded_requests();
        assert_eq!(reqs[0].method, hyper::Method::DELETE);
        assert_eq!(
            reqs[0].uri.to_string(),
            "http://test/indicator/stream-1/unsubscribe/IND-ABC"
        );
    }

    #[tokio::test]
    async fn subscribe_market_body_status_error() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"ERROR","message":"invalid stream"}"#,
        )]);
        let req = test_request(mock);

        let result = subscribe_market(&req, MarketFeed::Quotes, "s1", &["SYM"]).await;

        let err = result.unwrap_err();
        assert!(
            matches!(err, Error::Api { status: 200, ref message } if message == "invalid stream")
        );
    }

    #[tokio::test]
    async fn unsubscribe_market_body_status_error() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"status":"ERROR","message":"not subscribed"}"#,
        )]);
        let req = test_request(mock);

        let result = unsubscribe_market(&req, MarketFeed::Quotes, "s1", &["SYM"]).await;

        let err = result.unwrap_err();
        assert!(
            matches!(err, Error::Api { status: 200, ref message } if message == "not subscribed")
        );
    }

    #[tokio::test]
    async fn subscribe_market_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            hyper::StatusCode::UNAUTHORIZED,
            r#"{"error1":"Unauthorized"}"#,
        )]);
        let req = test_request(mock);

        let result = subscribe_market(&req, MarketFeed::Quotes, "s1", &["SYM"]).await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Api { status: 401, .. }));
    }
}
