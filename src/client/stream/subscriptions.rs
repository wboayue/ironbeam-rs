use hyper::header::HeaderMap;

use crate::client::http::HttpTransport;
use crate::error::{Error, Result, parse_api_error};
use crate::types::common::SuccessResponse;
use crate::types::streaming::{IndicatorSubscribeResponse, SubscribeBarsRequest};

/// Market data feed type for subscribe/unsubscribe.
pub(crate) enum MarketFeed {
    Quotes,
    Depths,
    Trades,
}

impl MarketFeed {
    fn as_str(&self) -> &'static str {
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
    fn as_str(&self) -> &'static str {
        match self {
            BarKind::Trade => "tradeBars",
            BarKind::Tick => "tickBars",
            BarKind::Time => "timeBars",
            BarKind::Volume => "volumeBars",
        }
    }
}

/// `GET /market/{feed}/subscribe/{streamId}?symbols=SYM1,SYM2`
pub(crate) async fn subscribe_market<H: HttpTransport>(
    http: &H,
    base_url: &str,
    headers: &HeaderMap,
    feed: MarketFeed,
    stream_id: &str,
    symbols: &[&str],
) -> Result<()> {
    let symbols_param = symbols.join(",");
    let path = format!(
        "{base_url}/market/{}/subscribe/{stream_id}?symbols={symbols_param}",
        feed.as_str()
    );
    let uri = path.parse()?;
    let (status, body) = http.get(uri, headers).await?;

    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: parse_api_error(&body),
        });
    }

    let resp: SuccessResponse = serde_json::from_slice(&body)?;
    if resp.status != crate::types::ResponseStatus::Ok {
        return Err(Error::Api {
            status: status.as_u16(),
            message: resp.message.unwrap_or_default(),
        });
    }

    Ok(())
}

/// `GET /market/{feed}/unsubscribe/{streamId}?symbols=SYM1,SYM2`
pub(crate) async fn unsubscribe_market<H: HttpTransport>(
    http: &H,
    base_url: &str,
    headers: &HeaderMap,
    feed: MarketFeed,
    stream_id: &str,
    symbols: &[&str],
) -> Result<()> {
    let symbols_param = symbols.join(",");
    let path = format!(
        "{base_url}/market/{}/unsubscribe/{stream_id}?symbols={symbols_param}",
        feed.as_str()
    );
    let uri = path.parse()?;
    let (status, body) = http.get(uri, headers).await?;

    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: parse_api_error(&body),
        });
    }

    Ok(())
}

/// `POST /indicator/{streamId}/{barKind}/subscribe`
pub(crate) async fn subscribe_indicator<H: HttpTransport>(
    http: &H,
    base_url: &str,
    headers: &HeaderMap,
    kind: BarKind,
    stream_id: &str,
    req: &SubscribeBarsRequest,
) -> Result<IndicatorSubscribeResponse> {
    let path = format!(
        "{base_url}/indicator/{stream_id}/{}/subscribe",
        kind.as_str()
    );
    let uri = path.parse()?;
    let body = bytes::Bytes::from(serde_json::to_vec(req)?);
    let (status, resp_body) = http.post(uri, body, headers).await?;

    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: parse_api_error(&resp_body),
        });
    }

    Ok(serde_json::from_slice(&resp_body)?)
}

/// `DELETE /indicator/{streamId}/unsubscribe/{indicatorId}`
pub(crate) async fn unsubscribe_indicator<H: HttpTransport>(
    http: &H,
    base_url: &str,
    headers: &HeaderMap,
    stream_id: &str,
    indicator_id: &str,
) -> Result<()> {
    let path = format!("{base_url}/indicator/{stream_id}/unsubscribe/{indicator_id}");
    let uri = path.parse()?;
    let (status, body) = http.delete(uri, headers).await?;

    if !status.is_success() {
        return Err(Error::Api {
            status: status.as_u16(),
            message: parse_api_error(&body),
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use hyper::header::{AUTHORIZATION, HeaderValue};

    use crate::client::http::mock::{MockHttp, MockResponse};

    use super::*;

    fn test_headers() -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(AUTHORIZATION, HeaderValue::from_static("Bearer tok_test"));
        h
    }

    #[tokio::test]
    async fn subscribe_quotes_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        subscribe_market(
            &mock,
            "http://test",
            &test_headers(),
            MarketFeed::Quotes,
            "stream-123",
            &["XCME:ES.U25", "XCME:NQ.U25"],
        )
        .await
        .unwrap();

        let reqs = mock.recorded_requests();
        assert_eq!(reqs.len(), 1);
        assert_eq!(reqs[0].method, "GET");
        assert_eq!(
            reqs[0].uri.to_string(),
            "http://test/market/quotes/subscribe/stream-123?symbols=XCME:ES.U25,XCME:NQ.U25"
        );
        assert_eq!(
            reqs[0].headers.get(AUTHORIZATION).unwrap(),
            "Bearer tok_test"
        );
    }

    #[tokio::test]
    async fn unsubscribe_depths_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        unsubscribe_market(
            &mock,
            "http://test",
            &test_headers(),
            MarketFeed::Depths,
            "stream-456",
            &["XCME:ES.U25"],
        )
        .await
        .unwrap();

        let reqs = mock.recorded_requests();
        assert_eq!(reqs[0].method, "GET");
        assert!(reqs[0].uri.to_string().contains("/market/depths/unsubscribe/stream-456"));
    }

    #[tokio::test]
    async fn subscribe_trades_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        subscribe_market(
            &mock,
            "http://test",
            &test_headers(),
            MarketFeed::Trades,
            "s1",
            &["SYM"],
        )
        .await
        .unwrap();

        let reqs = mock.recorded_requests();
        assert!(reqs[0].uri.to_string().contains("/market/trades/subscribe/s1"));
    }

    #[tokio::test]
    async fn subscribe_indicator_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(
            r#"{"indicatorId":"IND1","valueNames":["date","open"],"valueTypes":["date","number"]}"#,
        )]);

        let req = SubscribeBarsRequest {
            symbol: "XCME:ES.U25".into(),
            period: 1,
            bar_type: crate::types::BarType::Minute,
            load_size: 100,
        };

        let resp = subscribe_indicator(
            &mock,
            "http://test",
            &test_headers(),
            BarKind::Trade,
            "stream-789",
            &req,
        )
        .await
        .unwrap();

        assert_eq!(resp.indicator_id, "IND1");

        let reqs = mock.recorded_requests();
        assert_eq!(reqs[0].method, "POST");
        assert!(reqs[0]
            .uri
            .to_string()
            .contains("/indicator/stream-789/tradeBars/subscribe"));
    }

    #[tokio::test]
    async fn unsubscribe_indicator_sends_correct_request() {
        let mock = MockHttp::new(vec![MockResponse::ok(r#"{"status":"OK"}"#)]);
        unsubscribe_indicator(
            &mock,
            "http://test",
            &test_headers(),
            "stream-1",
            "IND-ABC",
        )
        .await
        .unwrap();

        let reqs = mock.recorded_requests();
        assert_eq!(reqs[0].method, "DELETE");
        assert_eq!(
            reqs[0].uri.to_string(),
            "http://test/indicator/stream-1/unsubscribe/IND-ABC"
        );
    }

    #[tokio::test]
    async fn subscribe_market_api_error() {
        let mock = MockHttp::new(vec![MockResponse::error(
            hyper::StatusCode::UNAUTHORIZED,
            r#"{"error1":"Unauthorized"}"#,
        )]);

        let result = subscribe_market(
            &mock,
            "http://test",
            &test_headers(),
            MarketFeed::Quotes,
            "s1",
            &["SYM"],
        )
        .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, Error::Api { status: 401, .. }));
    }
}
