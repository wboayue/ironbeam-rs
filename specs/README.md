# Ironbeam API Spec

Base URLs:
- Production: `https://live.ironbeamapi.com/v2`
- Demo: `https://demo.ironbeamapi.com/v2`

## Auth Pattern

1. `POST /auth` with username + apiKey → get bearer token
2. All subsequent requests: `Authorization: Bearer {token}`

## Streaming Pattern

1. `GET /stream/create` → get `streamId` (UUID)
2. Open WebSocket: `wss://{host}/v2/stream/{streamId}?token={token}`
3. Subscribe to data feeds via REST endpoints using `streamId`
4. On WebSocket close, must create a new `streamId`

## Error Responses

All endpoints may return:
| Code | Description |
|------|-------------|
| 400 | Bad request / missing params |
| 401 | Invalid/expired token |
| 403 | Forbidden |
| 406 | Not acceptable |
| 429 | Rate limited |
| 500 | Internal server error |

Error shape: `{ status: "ERROR", message: String, error: String }`

## Spec Files

| File | Description |
|------|-------------|
| [types.md](types.md) | Enums, type aliases, and all data structures |
| [auth.md](auth.md) | Authentication and logout |
| [account.md](account.md) | Balance, positions, risk, fills |
| [info.md](info.md) | Trader/user info, security definitions, symbol search, exchanges |
| [market-data.md](market-data.md) | Quotes, depth, historical trades (REST) |
| [orders.md](orders.md) | Place, update, cancel, query orders and fills |
| [streaming.md](streaming.md) | WebSocket streaming, subscriptions, indicators |
| [simulation.md](simulation.md) | Simulated trader/account management (demo only, enterprise) |
