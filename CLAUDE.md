# Ironbeam Rust API

## API Specs

Ironbeam API documentation is in `specs/`. Start with `specs/README.md` for overview, auth pattern, streaming pattern, and file index. Load only the domain-specific file you need:

- `specs/auth.md` — authentication/logout
- `specs/account.md` — balance, positions, risk, fills
- `specs/info.md` — trader/user info, security definitions, symbol search, exchanges
- `specs/market-data.md` — REST quotes, depth, historical trades
- `specs/orders.md` — place, update, cancel, query orders
- `specs/streaming.md` — WebSocket lifecycle, subscriptions, indicators
- `specs/simulation.md` — demo-only simulated trader/account management
- `specs/types.md` — all enums, type aliases, and data structures (load alongside any endpoint file)

## Type Unification: REST vs Streaming

The API sends the same domain objects over two transports with different JSON field names:
- **REST**: camelCase full names (`accountId`, `currencyCode`, `cashBalance`)
- **Streaming (WebSocket)**: abbreviated 1-4 char names (`a`, `cc`, `cb`)

### Unified types (one struct, `#[serde(alias)]` for streaming names)
- Balance / BalanceOpt
- MarginInfo / MarginInfoOpt
- MarginDetail / MarginDetailOpt
- Position / PositionOpt
- RiskInfo / RiskInfoOpt
- Order / OrderOpt
- OrderFill / OrderFillOpt

Pattern: `#[serde(rename = "accountId", alias = "a")]`
`rename` sets the canonical (REST) wire name; `alias` accepts the streaming abbreviation on deserialization. Serialization always uses the `rename` name.

### Shared types (already identical across REST and streaming)
- QuoteFull — always uses short field names
- Depth / DepthLevel — same structure
- TradeBar, TickBar, TimeBar, VolumeBar — same structure

### Separate types (cannot unify)
- **Trade** (REST) vs **TradeOpt** (streaming) — different because:
  1. `tickDirection` field uses `TickDirection` enum (REST) vs `TickDirectionType` integer enum (streaming)
  2. TradeOpt has 5 extra fields absent from REST: `isSettlement`, `isCancelled`, `systemPricedTrade`, `investigationStatus`, `blockTrade`

### Time fields
- API `Timestamp` (i64 ms since epoch) → `time::OffsetDateTime` with custom serde deserializer
- API `DateString` ("YYYYMMDD") → `time::Date` with custom serde deserializer

## Preferences

- **Async runtime:** tokio
- **Serialization:** serde
- **Date/time:** time
- Optimize for low latency
- All public client methods must have rustdoc comments with inline example code
- `examples/` folder must have runnable examples for all API usage, suitable for copy-paste
- Aim for 80%+ test coverage
