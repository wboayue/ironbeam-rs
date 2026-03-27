# Order Management

## POST /order/{accountId}/place

Place a new order.

**Path**: `accountId` (String)

**Body** (`OrderRequest`):
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| exchSym | Symbol | yes | e.g. "XCME:ES.U16" |
| side | OrderSide | yes | BUY, SELL |
| quantity | f64 | yes | |
| orderType | OrderType | yes | "1"=MARKET, "2"=LIMIT, "3"=STOP, "4"=STOP_LIMIT |
| duration | DurationType | yes | "0"=DAY, "1"=GTC |
| limitPrice | f64 | no | Required for LIMIT, STOP_LIMIT |
| stopPrice | f64 | no | Required for STOP, STOP_LIMIT |
| stopLoss | f64 | no | Bracket order stop loss price |
| takeProfit | f64 | no | Bracket order take profit price |
| stopLossOffset | f32 | no | Stop loss offset in pips |
| takeProfitOffset | f32 | no | Take profit offset in pips |
| trailingStop | f32 | no | Not yet supported |
| waitForOrderId | bool | no | Wait for exchange orderId (default: true) |

**Response 200** (`OrderBaseResponse`):
| Field | Type | Description |
|-------|------|-------------|
| orderId | String | Exchange order ID |
| strategyId | i64 | Strategy ID |

---

## PUT /order/{accountId}/update/{orderId}

Update an existing order.

**Path**: `accountId` (String), `orderId` (String)

**Body** (`OrderUpdateRequest`):
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| orderId | String | yes | |
| quantity | i32 | yes | |
| limitPrice | f64 | no | |
| stopPrice | f64 | no | |
| stopLoss | f64 | no | |
| takeProfit | f64 | no | |
| stopLossOffset | f32 | no | |
| takeProfitOffset | f32 | no | |

**Response 200** (`OrdersResponse`): `{ orders: Vec<Order> }`

---

## GET /order/{accountId}/{orderStatus}

Get orders by status.

**Path**: `accountId` (String), `orderStatus` (OrderStatusType, e.g. "ANY", "NEW", "FILLED", etc.)

**Response 200** (`OrdersResponse`): `{ orders: Vec<Order> }`

---

## DELETE /order/{accountId}/cancel/{orderId}

Cancel a single order.

**Path**: `accountId` (String), `orderId` (String)

**Response 200** (`OrdersResponse`): `{ orders: Vec<Order> }`

---

## DELETE /order/{accountId}/cancelMultiple

Cancel multiple orders.

**Path**: `accountId` (String)

**Body** (`OrderCancelMultipleRequest`):
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| orderIds | Vec\<String\> | Order IDs to cancel |

**Response 200** (`OrdersResponse`)

---

## GET /order/{accountId}/fills

Get order fills.

**Path**: `accountId` (String)

**Response 200** (`OrdersFillsResponse`): `{ fills: Vec<OrderFill> }`

---

## GET /order/{accountId}/toorderid/{strategyId}

Convert strategy ID to order ID.

**Path**: `accountId` (String), `strategyId` (i64)

**Response 200** (`OrderBaseResponse`): `{ orderId, strategyId }`

---

## GET /order/{accountId}/tostrategyId/{orderId}

Convert order ID to strategy ID.

**Path**: `accountId` (String), `orderId` (String)

**Response 200** (`OrderBaseResponse`): `{ orderId, strategyId }`
