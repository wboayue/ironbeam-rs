# Market Data (REST)

## GET /market/quotes

Get quotes for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`QuotesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| Quotes | Vec\<QuoteFull\> | See QuoteFull in [types.md](types.md) |

---

## GET /market/depth

Get market depth (order book) for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`DepthResponse`):
| Field | Type | Description |
|-------|------|-------------|
| Depths | Vec\<Depth\> | See Depth in [types.md](types.md) |

---

## GET /market/trades/{symbol}/{from}/{to}/{max}/{earlier}

Get historical trades.

**Path**:
| Param | Type | Description |
|-------|------|-------------|
| symbol | Symbol | e.g. XCME:ES.U16 |
| from | i64 | Start time (ms since epoch) |
| to | i64 | End time (ms since epoch) |
| max | i32 | Max records (1-100) |
| earlier | bool | Search direction |

**Response 200** (`TradesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| traders | Vec\<Trade\> | Note: field is named "traders" not "trades" |
