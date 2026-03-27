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
    http.rs         — low-level async HTTP (reqwest), token management
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

**Shared types** (identical across REST and streaming):
QuoteFull, Depth/DepthLevel, TradeBar, TickBar, TimeBar, VolumeBar

**Separate types** (cannot unify):
`Trade` (REST) vs `TradeOpt` (streaming) — different tick direction enum types + 5 extra streaming-only fields.

**Time fields:**
- API `Timestamp` (i64 ms epoch) → `time::OffsetDateTime` via `timestamp_ms` serde helper
- API `DateString` ("YYYYMMDD") → `time::Date` via `date_yyyymmdd` serde helper

## Stack

| Concern | Crate | Notes |
|---------|-------|-------|
| Async runtime | `tokio` | Use `tokio::select!` for concurrent stream + REST |
| HTTP | `reqwest` | Reuse single `Client` with connection pooling |
| WebSocket | `tokio-tungstenite` | Async WebSocket with auto-reconnect |
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

### Performance
- Minimize allocations on the hot path (streaming message deserialization).
- Use `serde_json::from_slice` over `from_str` where possible (avoids UTF-8 re-validation).
- Prefer `Bytes` / `&[u8]` for WebSocket frame handling.
- Connection pooling and keep-alive for HTTP client.
- Pre-allocate buffers for expected message sizes.

### Error Handling
- Define `Error` enum in `error.rs` with variants for: HTTP, WebSocket, Auth, Deserialization, Api (status + message), Timeout.
- Map API error responses (400, 401, 403, 429, 500) to typed variants with status code and body.
- Never panic. Never `unwrap()` outside tests.

### Testing
- Aim for 80%+ coverage.
- Unit tests inline (`#[cfg(test)]` modules) for serde round-trips and business logic.
- Integration tests in `tests/` using recorded API responses (no live calls in CI).
- All public methods must have rustdoc with `# Examples` showing async usage.

### Documentation
- All public types and methods get rustdoc with inline examples.
- `examples/` folder has runnable examples for each API domain, suitable for copy-paste.
- Examples use `#[tokio::main]` and show error handling.
