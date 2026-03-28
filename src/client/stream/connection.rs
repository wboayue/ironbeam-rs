use bytes::Bytes;
use fastwebsockets::handshake;
use fastwebsockets::{FragmentCollector, Frame, OpCode};
use http_body_util::Empty;
use hyper::header::{CONNECTION, UPGRADE};
use hyper::upgrade::Upgraded;
use hyper::{Request, Uri};
use hyper_util::rt::TokioIo;
use tokio::sync::{mpsc, watch};

use crate::error::{Error, Result};
use crate::types::streaming::StreamResponse;

use super::handler::StreamEvent;

/// Abstraction over WebSocket frames for testability.
pub(crate) trait WsTransport: Send + 'static {
    fn read_frame(
        &mut self,
    ) -> impl std::future::Future<Output = Result<WsMessage>> + Send;

    fn write_close(
        &mut self,
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Parsed WebSocket message.
pub(crate) enum WsMessage {
    Text(Bytes),
    Binary(Bytes),
    Close(Option<String>),
}

/// Production WebSocket transport using fastwebsockets.
pub(crate) struct FastWs {
    inner: FragmentCollector<TokioIo<Upgraded>>,
}

impl WsTransport for FastWs {
    async fn read_frame(&mut self) -> Result<WsMessage> {
        loop {
            let frame = self
                .inner
                .read_frame()
                .await
                .map_err(|e| Error::WebSocket(e.to_string()))?;

            match frame.opcode {
                OpCode::Text => return Ok(WsMessage::Text(Bytes::from(Vec::from(frame.payload)))),
                OpCode::Binary => {
                    return Ok(WsMessage::Binary(Bytes::from(Vec::from(frame.payload))))
                }
                OpCode::Close => {
                    let reason = if frame.payload.len() > 2 {
                        String::from_utf8(frame.payload[2..].to_vec()).ok()
                    } else {
                        None
                    };
                    return Ok(WsMessage::Close(reason));
                }
                other => {
                    tracing::debug!(?other, "ignoring unexpected WebSocket opcode");
                }
            }
        }
    }

    async fn write_close(&mut self) -> Result<()> {
        self.inner
            .write_frame(Frame::close(1000, b""))
            .await
            .map_err(|e| Error::WebSocket(e.to_string()))
    }
}

/// hyper upgrade executor for fastwebsockets handshake.
struct SpawnExecutor;

impl<Fut> hyper::rt::Executor<Fut> for SpawnExecutor
where
    Fut: std::future::Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    fn execute(&self, fut: Fut) {
        tokio::task::spawn(fut);
    }
}

/// Open a WebSocket connection to the streaming endpoint.
pub(crate) async fn connect(base_url: &str, stream_id: &str, token: &str) -> Result<FastWs> {
    let ws_url = build_ws_url(base_url, stream_id, token)?;
    let uri: Uri = ws_url.parse()?;

    let host = uri.host().ok_or_else(|| Error::WebSocket("missing host".into()))?;
    let port = uri.port_u16().unwrap_or(443);
    let addr = format!("{host}:{port}");

    let tcp = tokio::net::TcpStream::connect(&addr)
        .await
        .map_err(|e| Error::WebSocket(e.to_string()))?;

    let tls = connect_tls(host, tcp).await?;

    let req = Request::builder()
        .method("GET")
        .uri(uri.path_and_query().map(|pq| pq.as_str()).unwrap_or("/"))
        .header("Host", host)
        .header(UPGRADE, "websocket")
        .header(CONNECTION, "Upgrade")
        .header("Sec-WebSocket-Key", handshake::generate_key())
        .header("Sec-WebSocket-Version", "13")
        .body(Empty::<Bytes>::new())
        .map_err(|e| Error::WebSocket(e.to_string()))?;

    let (ws, _) = handshake::client(&SpawnExecutor, req, tls)
        .await
        .map_err(|e| Error::WebSocket(e.to_string()))?;

    Ok(FastWs {
        inner: FragmentCollector::new(ws),
    })
}

/// Run the WebSocket message loop, dispatching events through the channel.
pub(crate) async fn message_loop<W: WsTransport>(
    mut ws: W,
    tx: mpsc::Sender<Result<StreamEvent>>,
    mut shutdown_rx: watch::Receiver<bool>,
) {
    loop {
        tokio::select! {
            biased;

            _ = shutdown_rx.changed() => {
                let _ = ws.write_close().await;
                return;
            }

            frame = ws.read_frame() => {
                match frame {
                    Ok(WsMessage::Text(payload)) | Ok(WsMessage::Binary(payload)) => {
                        match serde_json::from_slice::<StreamResponse>(&payload) {
                            Ok(resp) => {
                                for event in resp.into_events() {
                                    if tx.send(Ok(event)).await.is_err() {
                                        return; // receiver dropped
                                    }
                                }
                            }
                            Err(e) => {
                                let _ = tx.send(Err(Error::Json(e))).await;
                            }
                        }
                    }
                    Ok(WsMessage::Close(reason)) => {
                        let msg = reason.unwrap_or_else(|| "connection closed by server".into());
                        let _ = tx.send(Err(Error::WebSocket(msg))).await;
                        return;
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        return;
                    }
                }
            }
        }
    }
}

/// Convert `https://host/v2` → `wss://host/v2/stream/{id}?token={token}`.
fn build_ws_url(base_url: &str, stream_id: &str, token: &str) -> Result<String> {
    let ws_base = base_url
        .replacen("https://", "wss://", 1)
        .replacen("http://", "ws://", 1);
    let encoded_token = urlencoding::encode(token);
    Ok(format!("{ws_base}/stream/{stream_id}?token={encoded_token}"))
}

async fn connect_tls(
    host: &str,
    tcp: tokio::net::TcpStream,
) -> Result<tokio_rustls::client::TlsStream<tokio::net::TcpStream>> {
    use std::sync::Arc;
    use tokio_rustls::TlsConnector;

    let mut root_store = rustls::RootCertStore::empty();
    root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());

    let config = rustls::ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(config));
    let domain = rustls::pki_types::ServerName::try_from(host.to_owned())
        .map_err(|e| Error::WebSocket(e.to_string()))?;

    connector
        .connect(domain, tcp)
        .await
        .map_err(|e| Error::WebSocket(e.to_string()))
}

#[cfg(test)]
pub(crate) mod mock {
    use std::collections::VecDeque;

    use bytes::Bytes;

    use crate::error::Result;

    use super::{WsMessage, WsTransport};

    /// Test double for WebSocket transport.
    pub struct MockWsTransport {
        frames: VecDeque<Result<WsMessage>>,
    }

    impl MockWsTransport {
        pub fn new(frames: Vec<Result<WsMessage>>) -> Self {
            Self {
                frames: frames.into(),
            }
        }

        /// Create a mock that yields JSON text frames then closes.
        pub fn from_json(messages: &[&str]) -> Self {
            let mut frames: Vec<Result<WsMessage>> = messages
                .iter()
                .map(|m| Ok(WsMessage::Text(Bytes::from(m.to_string()))))
                .collect();
            frames.push(Ok(WsMessage::Close(None)));
            Self::new(frames)
        }
    }

    impl WsTransport for MockWsTransport {
        async fn read_frame(&mut self) -> Result<WsMessage> {
            match self.frames.pop_front() {
                Some(frame) => frame,
                None => Ok(WsMessage::Close(None)),
            }
        }

        async fn write_close(&mut self) -> Result<()> {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mock::MockWsTransport;

    #[test]
    fn build_ws_url_from_https() {
        let url = build_ws_url("https://demo.ironbeamapi.com/v2", "abc-123", "tok").unwrap();
        assert_eq!(url, "wss://demo.ironbeamapi.com/v2/stream/abc-123?token=tok");
    }

    #[test]
    fn build_ws_url_from_http() {
        let url = build_ws_url("http://localhost:8080/v2", "id", "t").unwrap();
        assert_eq!(url, "ws://localhost:8080/v2/stream/id?token=t");
    }

    #[tokio::test]
    async fn message_loop_dispatches_events() {
        let ws = MockWsTransport::from_json(&[
            r#"{"q":[{"s":"XCME:ES.U25"}]}"#,
            r#"{"p":{"ping":"keepalive"}}"#,
        ]);
        let (tx, mut rx) = mpsc::channel(16);
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);

        tokio::spawn(message_loop(ws, tx, shutdown_rx));

        let event = rx.recv().await.unwrap().unwrap();
        assert!(matches!(event, StreamEvent::Quotes(..)));

        let event = rx.recv().await.unwrap().unwrap();
        assert!(matches!(event, StreamEvent::Ping(..)));

        // Close event
        let event = rx.recv().await.unwrap();
        assert!(matches!(event, Err(Error::WebSocket(..))));
    }

    #[tokio::test]
    async fn message_loop_handles_bad_json() {
        let ws = MockWsTransport::from_json(&[
            "not valid json",
            r#"{"p":{"ping":"ok"}}"#,
        ]);
        let (tx, mut rx) = mpsc::channel(16);
        let (_shutdown_tx, shutdown_rx) = watch::channel(false);

        tokio::spawn(message_loop(ws, tx, shutdown_rx));

        // First event: JSON error
        let event = rx.recv().await.unwrap();
        assert!(matches!(event, Err(Error::Json(..))));

        // Loop continues — next valid frame arrives
        let event = rx.recv().await.unwrap().unwrap();
        assert!(matches!(event, StreamEvent::Ping(..)));

        // Then close
        let event = rx.recv().await.unwrap();
        assert!(matches!(event, Err(Error::WebSocket(..))));
    }

    #[tokio::test]
    async fn message_loop_shutdown() {
        let ws = MockWsTransport::new(vec![]);
        let (tx, mut rx) = mpsc::channel(16);
        let (shutdown_tx, shutdown_rx) = watch::channel(false);

        let handle = tokio::spawn(message_loop(ws, tx, shutdown_rx));

        // Signal shutdown before any frames
        let _ = shutdown_tx.send(true);
        handle.await.unwrap();

        // Channel closed, no events
        assert!(rx.recv().await.is_none());
    }
}
