# Streaming

## Stream Lifecycle

1. `GET /stream/create` → `streamId` (UUID)
2. Open WebSocket: `wss://{host}/v2/stream/{streamId}?token={token}`
3. Subscribe to feeds using `streamId`
4. On WebSocket close, create a new `streamId` before reconnecting

---

## GET /stream/create

Create a new stream session.

**Response 200** (`StreamIdResponse`):
| Field | Type | Description |
|-------|------|-------------|
| streamId | String (UUID) | Use for WebSocket URL and subscribe/unsubscribe calls |

---

## WebSocket: wss://{host}/v2/stream/{streamId}?token={token}

Receives all subscribed data as JSON `StreamResponse` messages.

**StreamResponse** fields (all optional, present when data available):
| Field | Type | Description |
|-------|------|-------------|
| p | PingMessage | Keepalive ping |
| q | Vec\<QuoteFull\> | Quote updates |
| d | Vec\<Depth\> | Depth updates |
| tr | Vec\<TradeOpt\> | Trade updates |
| o | Vec\<OrderOpt\> | Order updates |
| f | Vec\<OrderFillOpt\> | Fill updates |
| ps | Vec\<PositionOpt\> | Position changes |
| psa | Vec\<AccountPositionsOpt\> | Initial position snapshot (all accounts) |
| b | BalanceOpt | Balance update |
| ba | Vec\<BalanceOpt\> | Initial balance snapshot (all accounts) |
| ri | RiskInfoOpt | Risk info change |
| ria | Vec\<RiskInfoOpt\> | Initial risk snapshot (all accounts) |
| tb | Vec\<TradeBar\> | Trade bars |
| tc | Vec\<TickBar\> | Tick bars |
| ti | Vec\<TimeBar\> | Time bars |
| vb | Vec\<VolumeBar\> | Volume bars |
| i | Vec\<IndicatorValues\> | Indicator values |
| r | Response | Account/session change notification |

---

## Market Data Subscriptions

All subscription endpoints follow the same pattern:
- **Path**: `streamId` (UUID)
- **Query**: `symbols` (comma-separated, required, max 10)
- **Response 200**: `SuccessResponse`

| Action | Endpoint |
|--------|----------|
| Subscribe quotes | `GET /market/quotes/subscribe/{streamId}` |
| Unsubscribe quotes | `GET /market/quotes/unsubscribe/{streamId}` |
| Subscribe depth | `GET /market/depths/subscribe/{streamId}` |
| Unsubscribe depth | `GET /market/depths/unsubscribe/{streamId}` |
| Subscribe trades | `GET /market/trades/subscribe/{streamId}` |
| Unsubscribe trades | `GET /market/trades/unsubscribe/{streamId}` |

---

## Indicator Subscriptions

### POST /indicator/{streamId}/tradeBars/subscribe

Subscribe to trade bar indicators.

**Path**: `streamId` (UUID)

**Body** (`SubscribeTradeBarsRequest`):
| Field | Type | Description |
|-------|------|-------------|
| symbol | Symbol | |
| period | i32 | Bar period |
| barType | BarType | DAILY, HOUR, MINUTE, TICK |
| loadSize | i32 | Initial history load size |

**Response 200** (`IndicatorSubscribeResponse`):
| Field | Type | Description |
|-------|------|-------------|
| indicatorId | String | Use to unsubscribe |
| valueNames | Vec\<String\> | e.g. ["date","open","close","high","low","volume","tradeCount","delta","value"] |
| valueTypes | Vec\<String\> | e.g. ["date","number","string",...] |

### POST /indicator/{streamId}/tickBars/subscribe

Same request/response shape as tradeBars.

### POST /indicator/{streamId}/timeBars/subscribe

Same request/response shape as tradeBars.

### POST /indicator/{streamId}/volumeBars/subscribe

Same request/response shape as tradeBars.

---

### DELETE /indicator/{streamId}/unsubscribe/{indicatorId}

Unsubscribe from an indicator.

**Path**: `streamId` (UUID), `indicatorId` (String)
