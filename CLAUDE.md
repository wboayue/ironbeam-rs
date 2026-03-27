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
