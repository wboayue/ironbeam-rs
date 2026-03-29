# Ironbeam Rust Client

Async Rust client for the Ironbeam futures trading API. Targets low-latency order execution and real-time streaming.

## API Specs

Documentation in `specs/`. Start with `specs/README.md` for auth pattern, streaming lifecycle, error shapes, and file index. Load only the file you need:

- `specs/auth.md` — authentication/logout
- `specs/account.md` — balance, positions, risk, fills
- `specs/info.md` — trader/user info, security definitions, symbol search, exchanges
- `specs/market-data.md` — REST quotes, depth, historical trades
- `specs/orders.md` — place, update, cancel, query orders
- `specs/streaming.md` — WebSocket lifecycle, subscriptions, indicators
- `specs/simulation.md` — demo-only simulated trader/account management
- `specs/types.md` — all enums, type aliases, and data structures

## API Rate Limit

- Hard limit of **10 requests per second** (sliding window). Auth and logout count toward this limit.
- Exceeding the limit returns HTTP 429. Once tripped, the cooldown is **15–30 seconds** — not just the next second.
- 429 errors themselves appear to count toward the limit, causing cascading failures.
- Examples must pace calls (≥250ms between requests) and keep total calls under 10/sec including auth/logout.
- Symbol search (`/info/symbols`) requires a minimum of 3 characters in the `text` parameter; shorter strings return 400 "Invalid text length".

## Architecture

### Design Principles

- **Single Responsibility**: each struct/module owns one concern. Separate transport (HTTP, WebSocket), serialization, and business logic.
- **Composable**: prefer small, combinable types over monolithic clients. Users should be able to use REST without pulling in WebSocket, or market data without order management.
- **Zero-cost ergonomics**: use generics and traits to avoid runtime overhead. Prefer compile-time dispatch.
- **Fail fast**: return typed errors immediately. No silent retries or swallowed errors in the core library.

### Module Layout

```
src/
  lib.rs            — crate root, re-exports
  types/            — domain model (enums, structs, serde) ✅ DONE
  error.rs          — error types (thiserror)
  client/
    config.rs       — ClientConfig (base_url, credentials, timeouts)
    http.rs         — thin ergonomic wrapper around hyper (JSON send/recv, status mapping, token injection)
    rest/           — typed REST endpoint methods, one file per domain
      auth.rs
      account.rs
      market.rs
      orders.rs
      info.rs
      simulation.rs
    stream/         — WebSocket streaming
      connection.rs — connect, reconnect, message loop
      subscriptions.rs — subscribe/unsubscribe helpers
      handler.rs    — StreamResponse dispatch to user callbacks or channels
```

### Type Unification: REST vs Streaming

The API sends the same domain objects over two transports with different JSON field names:
- **REST**: camelCase full names (`accountId`, `currencyCode`, `cashBalance`)
- **Streaming (WebSocket)**: abbreviated 1-4 char names (`a`, `cc`, `cb`)

**Unified types** (one struct, `#[serde(alias)]` for streaming names):
Balance, MarginInfo, MarginDetail, Position, RiskInfo, Order, OrderFill

Pattern: `#[serde(rename = "accountId", alias = "a")]`
`rename` sets the canonical (REST) wire name; `alias` accepts the streaming abbreviation. Serialization always uses `rename`.

**Shared types** (streaming-only short wire names):
QuoteFull, Depth/DepthLevel, TradeBar, TickBar, TimeBar, VolumeBar, TradeOpt, IndicatorValues

Pattern: `#[serde(rename = "s")] pub symbol: Symbol` — always use descriptive Rust field names, never expose short wire names as field identifiers. Map via `#[serde(rename = "wire_name")]`.

**Separate types** (cannot unify):
`Trade` (REST) vs `TradeOpt` (streaming) — different tick direction enum types + 5 extra streaming-only fields.

**Dual-format enums** (REST sends strings, streaming sends integers):
`RegCodeType`, `BalanceType`, `ResponseStatus`, `DepthSide` — custom `Deserialize` impls that accept both string (`"COMBINED"`) and integer (`1`) representations. Serialize always uses strings. Add the same pattern for any enum that appears in both REST and streaming responses with differing wire formats.

**Time fields:**
- API `Timestamp` (i64 ms epoch) → `time::OffsetDateTime` via `timestamp_ms` serde helper
- API `DateString` ("YYYYMMDD") → `time::Date` via `date_yyyymmdd` serde helper

## Stack

| Concern | Crate | Notes |
|---------|-------|-------|
| Async runtime | `tokio` | Use `tokio::select!` for concurrent stream + REST |
| HTTP | `hyper` + `hyper-util` | Thin wrapper in `client/http.rs`; `hyper-rustls` for TLS |
| WebSocket | `fastwebsockets` | Zero-copy frame parsing; use `from_slice` on payloads |
| Serialization | `serde` + `serde_json` | `serde_repr` for integer enums |
| Date/time | `time` | Never use `chrono` |
| Errors | `thiserror` | Typed error enums, no `anyhow` in library code |
| Logging | `tracing` | Structured, async-aware |

## Coding Standards

### Rust Idioms
- All public APIs are `async`. Use `async fn` on traits (Rust 2024 edition).
- Return `Result<T, Error>` everywhere. Define a crate-level `Error` enum.
- Use `impl Into<T>` / `AsRef<str>` for string params to avoid unnecessary allocations.
- Prefer `&str` over `String` in function signatures; let callers own data.
- Use builders for complex request construction (e.g., `OrderRequestBuilder`).
- Avoid `unwrap()`/`expect()` in library code. Reserve for tests only.
- Use `#[must_use]` on functions returning values that shouldn't be silently dropped.
- Never derive `Debug` on types holding secrets (credentials, tokens). Use manual impls that redact sensitive fields.
- **Field naming**: always use descriptive Rust names for struct fields, never short wire names. Map to wire format via `#[serde(rename = "wire_name")]` or `#[serde(rename = "camelCase", alias = "short")]`.

### Performance
- Minimize allocations on the hot path (streaming message deserialization).
- Use `serde_json::from_slice` over `from_str` where possible (avoids UTF-8 re-validation).
- Prefer `Bytes` / `&[u8]` for WebSocket frame handling.
- Connection pooling and keep-alive for HTTP client.
- Pre-allocate buffers for expected message sizes.
- Cache per-connection state (e.g., auth headers) at construction time, not per-request.

### Error Handling
- Define `Error` enum in `error.rs` with variants for: HTTP, WebSocket, Auth, Deserialization, Api (status + message), Timeout.
- Map API error responses (400, 401, 403, 429, 500) to typed variants with status code and body.
- Never panic. Never `unwrap()` outside tests.
- Library `Error` must not include application-only concerns (e.g., `env::VarError`). Examples use `Box<dyn Error>`.

### Logging
- Use `tracing` with structured fields, not string interpolation. Prefer `tracing::info!(stream_id = %id, "msg")` over `tracing::info!("msg for {id}")`.
- **Level guide:**
  - `error!` — transport/infrastructure failures that terminate an operation (WebSocket read error, TLS failure). Something is broken.
  - `warn!` — recoverable issues that may need investigation (parse failures, server-initiated disconnects, failed background cleanup).
  - `info!` — lifecycle events and state changes (authenticated, stream connected/closed, subscribe/unsubscribe). One line per operation, not per message.
  - `debug!` — internal details useful during development (raw payloads, shutdown signals, unexpected but harmless opcodes, config fallbacks).
- **Never log at `info!` or above on the hot path** (per-message processing). Parse/dispatch in the message loop should only log on errors.
- **Always include correlation fields** (`stream_id`, `indicator_id`) on stream-related logs so messages from concurrent streams can be distinguished.
- **Never log secrets** (tokens, passwords, API keys). Auth headers are redacted in `Debug` impls; keep it that way.
- No logging in type/serde code. Logging belongs in client/transport layers only.

### Testing
- Aim for 80%+ coverage.
- Unit tests inline (`#[cfg(test)]` modules) for serde round-trips and business logic.
- Integration tests in `tests/` using recorded API responses (no live calls in CI).
- All public methods must have rustdoc with `# Examples` showing async usage.
- Use `MockHttp` (`src/client/http.rs::mock`) for all HTTP tests — no live calls.
  - Pre-load canned responses: `MockHttp::new(vec![MockResponse::ok(body), ...])`.
  - Responses are consumed FIFO; empty queue panics (catches unexpected calls).
  - Inspect after: `mock.recorded_requests()` returns `Vec<RecordedRequest>` with method, uri, headers, body.
- Build test clients with `Client { base_url, auth_headers, http: mock, is_logged_out: AtomicBool::new(false) }`.
- Test each error branch: HTTP-level errors (non-2xx → `Error::Api`), body-level errors (status field → `Error::Auth`), malformed JSON → `Error::Json`.
- Verify auth headers are forwarded by asserting on `recorded_requests()[n].headers`.

### Documentation
- All public types and methods get rustdoc with inline examples.
- Public API methods on `Client` must include a `# Example` section with a `no_run` doc test showing async usage. Use hidden setup lines (`# `) for builder/connect boilerplate so the visible example focuses on the method call:
  ```rust
  /// # Example
  ///
  /// ```no_run
  /// # use ironbeam_rs::client::{Client, Credentials};
  /// # async fn example() -> ironbeam_rs::error::Result<()> {
  /// # let client = Client::builder()
  /// #     .credentials(Credentials { username: "u".into(), password: "p".into(), api_key: "k".into() })
  /// #     .connect().await?;
  /// let accounts = client.all_accounts().await?;
  /// # Ok(())
  /// # }
  /// ```
  ```
- `examples/` folder has runnable examples for each API domain, suitable for copy-paste.
- Examples use `#[tokio::main]` and show error handling.
- Keep doc examples in sync with actual API types. Run `cargo test --doc` to verify.

### Maintenance
- When fixing a review issue that reflects a missing or unclear convention, update this file so the same issue isn't repeated. CLAUDE.md is the source of truth for project standards.

### API Notes
- Auth requires all three fields: username, password, and apiKey (spec says optional, but API rejects without all three).
