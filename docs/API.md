# Ironbeam API Reference

Base URLs:
- Production: `https://live.ironbeamapi.com/v2`
- Demo: `https://demo.ironbeamapi.com/v2`

Auth: Bearer token via `POST /auth`. Include as `Authorization: Bearer {token}` header on all subsequent requests.

Streaming: WebSocket at `wss://{host}/v2/stream/{streamId}?token={token}`. Must call `/stream/create` first to get a `streamId`. Create a new `streamId` each time the WebSocket closes.

---

## Endpoints

### Authentication

#### POST /auth
Authenticate and obtain a bearer token.

**Body** (`AuthorizationRequest`):
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| username | String | yes | Account ID |
| password | String | no | Password (for enterprise users) |
| apiKey | String | no | API key |

**Response 200** (`AuthorizationResponse`):
| Field | Type | Description |
|-------|------|-------------|
| status | ResponseStatus | OK, ERROR, etc. |
| message | String | |
| token | String | Bearer token for subsequent requests |

**Errors**: 400 (missing credentials), 401 (invalid credentials), 403 (forbidden), 406 (not acceptable), 429 (rate limited), 500 (server error)

---

#### POST /logout
Invalidate the current token.

**Response 200** (`SuccessResponse`): `{ status: "OK", message: "OK" }`

---

### Information

#### GET /info/trader
Get trader info (accounts list, live/demo status).

**Query**: `traderId` (String, optional)

**Response 200** (`TraderInfoResponse`):
| Field | Type | Description |
|-------|------|-------------|
| accounts | Vec\<AccountId\> | List of account IDs |
| isLive | bool | Whether this is a live account |
| traderId | String | Trader identifier |

---

#### GET /info/user
Get user general info (contact info, account metadata).

**Query**: `traderId` (String, optional)

**Response 200** (`UserInfoResponse`):
| Field | Type | Description |
|-------|------|-------------|
| accountCategory | i32 | |
| accountTitle | String | |
| emailAddress1 | String | |
| emailAddress2 | String | |
| group | String | |
| isClearingAccount | bool | |
| phone1 | String | |
| phone2 | String | |
| subGroup | String | |
| accounts | Vec\<AccountId\> | |

---

#### GET /info/security/definitions
Get security definitions for given symbols.

**Query**: `symbols` (comma-separated, required, max 10). Format: `EXCHANGE:SYMBOL`, e.g. `XCME:ES.U16`

**Response 200** (`SecurityDefinitionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| securityDefinitions | Vec\<SecurityDefinition\> | See SecurityDefinition type below |

---

#### GET /info/security/margin
Get margin and value info for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`SecurityMarginAndValueResponse`):
| Field | Type | Description |
|-------|------|-------------|
| securityMarginAndValues | Vec\<SecurityMarginAndValue\> | See type below |

---

#### GET /info/security/status
Get trading status for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`SecurityStatusResponse`):
| Field | Type | Description |
|-------|------|-------------|
| securityStatuses | Vec\<SecurityStatus\> | See type below |

---

#### GET /info/symbols
Search for symbols.

**Query**:
| Param | Type | Required | Description |
|-------|------|----------|-------------|
| text | String | no | Search text |
| limit | i32 | no | Max results |
| preferActive | bool | no | Prefer active contracts |

**Response 200** (`SymbolsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbols | Vec\<SymbolInfo\> | See type below |

---

#### GET /info/exchangeSources
Get list of available exchanges.

**Response 200** (`ExchangeSourcesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| exchanges | Vec\<String\> | Exchange identifiers |

---

#### GET /info/complexes/{exchange}
Get market complexes for an exchange.

**Path**: `exchange` (String, e.g. "XCME")

**Response 200** (`ComplexesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| marketComplexes | Vec\<ComplexGroups\> | Groups with name and sub-groups |

---

#### GET /info/symbol/search/futures/{exchange}/{marketGroup}
Search for futures symbols.

**Path**: `exchange` (String), `marketGroup` (String, e.g. "ES")

**Response 200** (`SymbolFuturesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbols | Vec\<FutureInfo\> | symbol, maturityMonth, maturityYear, description |

---

#### GET /info/symbol/search/groups/{complex}
Get symbol groups by market complex.

**Path**: `complex` (String, e.g. "Currency")

**Response 200** (`ComplexGroupsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbolGroups | Vec\<ComplexGroupInfo\> | group name + display name |

---

#### GET /info/symbol/search/options/{symbol}
Get option groups for a symbol.

**Path**: `symbol` (Symbol)

**Response 200** (`SymbolOptionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| groups | Vec\<String\> | |
| optionGroups | Vec\<OptionGroupInfo\> | group, expiration, description |

---

#### GET /info/symbol/search/options/ext/{symbol}/{group}/{optionType}/{near}
Search for specific options.

**Path**: `symbol` (Symbol), `group` (String), `optionType` ("call" | "put"), `near` (bool)

**Response 200** (`SymbolSearchOptionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbolOptions | Vec\<Symbol\> | Matching option symbols |

---

#### GET /info/symbol/search/options/spreads/{symbol}
Get available option spreads.

**Path**: `symbol` (Symbol)

**Response 200** (`SymbolOptionSpreadsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbolSpreads | Vec\<Spread\> | Spread definitions |

---

#### GET /info/strategyId
Get a new strategy ID for order grouping.

**Response 200** (`StrategyIdResponse`):
| Field | Type | Description |
|-------|------|-------------|
| Id | i64 | Strategy ID |
| Minimum | i64 | Min value in range |
| Maximum | i64 | Max value in range |

---

### Account

#### GET /account/{accountId}/balance
Get account balance.

**Path**: `accountId` (String)
**Query**: `balanceType` (required): `CURRENT_OPEN` | `START_OF_DAY`

**Response 200** (`AccountBalanceResponse`):
| Field | Type | Description |
|-------|------|-------------|
| balances | Vec\<Balance\> | See Balance type below |

---

#### GET /account/{accountId}/positions
Get open positions.

**Path**: `accountId` (String)

**Response 200** (`PositionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| positions | Vec\<Position\> | See Position type below |

---

#### GET /account/{accountId}/risk
Get risk info.

**Path**: `accountId` (String)

**Response 200** (`AccountRiskResponse`):
| Field | Type | Description |
|-------|------|-------------|
| risks | Vec\<RiskInfo\> | See RiskInfo type below |

---

#### GET /account/{accountId}/fills
Get account fills.

**Path**: `accountId` (String)

**Response 200** (`AccountFillsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| fills | Vec\<OrderFill\> | See OrderFill type below |

---

#### GET /account/getAllAccounts
Get all accounts for the authenticated trader.

**Response 200** (`AllAccountsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| accounts | Vec\<AccountId\> | |

---

#### GET /account/getAllBalances
Get balances for all accounts.

**Query**: `balanceType` (required): `CURRENT_OPEN` | `START_OF_DAY`

**Response 200** (`AccountBalanceResponse`)

---

#### GET /account/getAllFills
Get fills for all accounts.

**Response 200** (`AccountFillsResponse`)

---

#### GET /account/getAllPositions
Get positions for all accounts.

**Response 200** (`AccountsPositionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| positions | Vec\<Positions\> | Each entry has accountId + Vec\<Position\> |

---

#### GET /account/getAllRiskInfo
Get risk info for all accounts.

**Response 200** (`AccountRiskResponse`)

---

### Market Data

#### GET /market/quotes
Get quotes for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`QuotesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| Quotes | Vec\<QuoteFull\> | See QuoteFull type below |

---

#### GET /market/depth
Get market depth (order book) for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`DepthResponse`):
| Field | Type | Description |
|-------|------|-------------|
| Depths | Vec\<Depth\> | See Depth type below |

---

#### GET /market/trades/{symbol}/{from}/{to}/{max}/{earlier}
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

---

### Order Management

#### POST /order/{accountId}/place
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

#### PUT /order/{accountId}/update/{orderId}
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

#### GET /order/{accountId}/{orderStatus}
Get orders by status.

**Path**: `accountId` (String), `orderStatus` (OrderStatusType, e.g. "ANY", "NEW", "FILLED", etc.)

**Response 200** (`OrdersResponse`): `{ orders: Vec<Order> }`

---

#### DELETE /order/{accountId}/cancel/{orderId}
Cancel a single order.

**Path**: `accountId` (String), `orderId` (String)

**Response 200** (`OrdersResponse`): `{ orders: Vec<Order> }`

---

#### DELETE /order/{accountId}/cancelMultiple
Cancel multiple orders.

**Path**: `accountId` (String)

**Body** (`OrderCancelMultipleRequest`):
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| orderIds | Vec\<String\> | Order IDs to cancel |

**Response 200** (`OrdersResponse`)

---

#### GET /order/{accountId}/fills
Get order fills.

**Path**: `accountId` (String)

**Response 200** (`OrdersFillsResponse`): `{ fills: Vec<OrderFill> }`

---

#### GET /order/{accountId}/toorderid/{strategyId}
Convert strategy ID to order ID.

**Path**: `accountId` (String), `strategyId` (i64)

**Response 200** (`OrderBaseResponse`): `{ orderId, strategyId }`

---

#### GET /order/{accountId}/tostrategyId/{orderId}
Convert order ID to strategy ID.

**Path**: `accountId` (String), `orderId` (String)

**Response 200** (`OrderBaseResponse`): `{ orderId, strategyId }`

---

### Streaming

#### GET /stream/create
Create a new stream session. Returns a `streamId` (UUID) used for WebSocket connection and subscriptions.

**Response 200** (`StreamIdResponse`):
| Field | Type | Description |
|-------|------|-------------|
| streamId | String (UUID) | Use for WebSocket URL and subscribe/unsubscribe calls |

---

#### WebSocket: wss://{host}/v2/stream/{streamId}?token={token}
Open after calling `/stream/create`. Receives all subscribed data as JSON `StreamResponse` messages.

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

#### GET /market/quotes/subscribe/{streamId}
Subscribe to quote updates.

**Path**: `streamId` (UUID)
**Query**: `symbols` (comma-separated, required, max 10)

---

#### GET /market/quotes/unsubscribe/{streamId}
Unsubscribe from quote updates.

**Path**: `streamId` (UUID)
**Query**: `symbols` (comma-separated, required, max 10)

---

#### GET /market/depths/subscribe/{streamId}
Subscribe to depth (order book) updates.

**Path**: `streamId` (UUID)
**Query**: `symbols` (comma-separated, required, max 10)

---

#### GET /market/depths/unsubscribe/{streamId}
Unsubscribe from depth updates.

**Path**: `streamId` (UUID)
**Query**: `symbols` (comma-separated, required, max 10)

---

#### GET /market/trades/subscribe/{streamId}
Subscribe to trade updates.

**Path**: `streamId` (UUID)
**Query**: `symbols` (comma-separated, required, max 10)

---

#### GET /market/trades/unsubscribe/{streamId}
Unsubscribe from trade updates.

**Path**: `streamId` (UUID)
**Query**: `symbols` (comma-separated, required, max 10)

---

### Indicators

#### POST /indicator/{streamId}/tradeBars/subscribe
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

---

#### POST /indicator/{streamId}/tickBars/subscribe
Same request/response shape as tradeBars.

#### POST /indicator/{streamId}/timeBars/subscribe
Same request/response shape as tradeBars.

#### POST /indicator/{streamId}/volumeBars/subscribe
Same request/response shape as tradeBars.

---

#### DELETE /indicator/{streamId}/unsubscribe/{indicatorId}
Unsubscribe from an indicator.

**Path**: `streamId` (UUID), `indicatorId` (String)

---

### Simulated Trader/Account (Demo Only, Enterprise)

#### POST /simulatedTraderCreate
Create a simulated trader.

**Body**: `{ FirstName, LastName, Address1, Address2?, City, State, Country, ZipCode, Phone, Email, Password, TemplateId }` (all String)

TemplateId values: `XAP50` ($50k), `XAP100` ($100k), `XAP150` ($150k)

**Response 200**: `{ TraderId: String }`

---

#### POST /simulatedAccountAdd
Add account to existing trader. Body: `{ TraderId, Password, TemplateId }`

**Response 200**: `{ AccountId: String }`

---

#### PUT /simulatedAccountReset
Reset account to initial state. Body: `{ AccountId, TemplateId }`

---

#### DELETE /simulatedAccountExpire
Expire an account. Body: `{ AccountId }`

---

#### POST /simulatedAccount/addCash
Add cash to account. Body: `{ AccountId: String, Amount: f32 }`

---

#### GET /simulatedAccount/getCashReport/{accountId}
**Path**: `accountId`
**Query**: `startDate` (i64, YYYYMMDD), `endDate` (i64, YYYYMMDD)

**Response 200**: `{ AccountId, CashReport: Vec<{ amount: f64, entryDate: i64, availableDate: i64 }> }`

---

#### POST /simulatedAccount/liquidate
Liquidate accounts. Body: `{ Accounts?, Groups?, ExceptAccounts?, ForceManualLiquidation?, UseManualLiquidationForIlliquidMarkets?, SendAccountEmail?, SendOfficeEmail? }`

---

#### POST /simulatedAccount/setRisk
Set risk parameters. Body includes `AccountId` (required) plus optional nullable thresholds: `LiquidationAccountValue`, `LiquidationLossFromStartOfDay`, `LiquidationLossFromHighOfDay`, `LiquidationLossFromHighOfMultiday`, `LiquidationPctLossFromStartOfDay` (0-100), `LiquidationPctLossFromHighOfDay` (0-100), `LiquidationPctLossFromHighOfMultiday` (0-100), `LiquidationPctMarginDeficiency` (0-100), `LiquidationMaxValueOverride`, `ReducePositionsOnly`, `RestoreTrading`, `MarginScheduleName`, `TemplateId`.

---

## Types

### Enums

```
ResponseStatus: OK | ERROR | WARNING | INFO | FATAL | UNKNOWN

BalanceType: CURRENT_OPEN | START_OF_DAY

OrderSide: BUY | SELL | INVALID

OrderType: "" (INVALID) | "1" (MARKET) | "2" (LIMIT) | "3" (STOP) | "4" (STOP_LIMIT)

DurationType: "" (INVALID) | "0" (DAY) | "1" (GOOD_TILL_CANCEL)

OrderStatusType: ANY | INVALID | SUBMITTED | NEW | PARTIALLY_FILLED | FILLED
  | DONE_FOR_DAY | CANCELLED | REPLACED | PENDING_CANCEL | STOPPED
  | REJECTED | SUSPENDED | PENDING_NEW | CALCULATED | EXPIRED
  | ACCEPTED_FOR_BIDDING | PENDING_REPLACE | CANCEL_REJECTED
  | ORDER_NOT_FOUND | QUEUED_NEW | QUEUED_CANCEL | COMPLETE

PositionSide: LONG | SHORT

SecurityType: INVALID | FUT | OPT | MIXED

OptionType: INVALID | PUT | CALL

OptionExpirationType: INVALID | AMERICAN | EUROPEAN

Side: BID | ASK

SideShort: B | A

RegCodeType: INVALID | COMBINED | REGULATED | NON_SECURED | SECURED

SecurityStatusType (integer):
  2=TRADING_HALT | 4=CLOSED | 15=PRICE_INDICATION | 17=OPEN | 18=CLOSE
  | 20=UNKNOWN | 21=PRE_OPEN | 22=OPENING_ROTATION | 24=PRE_CROSS
  | 25=CROSS | 26=NO_CANCEL | 30=EXPIRED | 31=PRE_CLOSE
  | 103=NO_CHANGE | 126=POST_CLOSE

AggressorSideType (integer): 0=INVALID | 1=BUY | 2=SELL

TickDirectionType (integer): 0=PLUS | 1=SAME | 2=MINUS | 255=INVALID

TickDirection: INVALID | PLUS | MINUS | SAME

BarType: DAILY | HOUR | MINUTE | TICK

ExchangeStrategyType: NONE | SP | FX | RT | EQ | BF | CF | FS | IS | PK
  | MP | PB | DF | PS | C1 | FB | BS | SA | SB | WS | XS | DI | IV
  | EC | SI | SD | MS | _3W | _3C | _3P | BX | BO | XT | CC | CO
  | DB | HO | DG | HS | IC | _12 | _13 | _23 | RR | SS | ST | SG
  | SR | VT | JR | IB | GT | GN | DN

SystemPricedTrade: INVALID | SYSTEM | CRACK

InvestigationStatus: INVALID | INVESTIGATING | COMPLETED

BlockTrade: INVALID | NORMAL | EFP | EFS | OFF_EXCHANGE | NG | CCX | EFR
```

### Primitive Type Aliases

```
AccountId: String (pattern: ^\S+$, maxLength: 10)
Symbol: String (pattern: ^([A-Z]{1,10}:)?[A-Z0-9 _\.]{1,120}$, maxLength: 130)
  Format: "EXCHANGE:SYMBOL" e.g. "XCME:ES.U16"
Timestamp: i64 (milliseconds since epoch)
Price: f64
Quantity: f64
Volume: f64
NumberString: String (pattern: ^[0-9]{1,10}\.[0-9]{1,10}$)
CurrencyCode: String (pattern: ^[A-Z]{3}$, e.g. "USD")
DateString: String (pattern: ^[0-9]{8}$, YYYYMMDD)
OrderId: String (maxLength: 50)
StrategyId: i64
OrderUpdateId: String (pattern: ^[0-9\-]{1,32}$)
PositionId: String (maxLength: 50)
StreamSessionId: String (UUID)
IndicatorId: String (maxLength: 100)
SubAccountId: String (pattern: ^\S+$, maxLength: 10)
Spread: String (pattern: ^([+-]\d+:[A-Za-z0-9._]+)(:[+-]\d+:[A-Za-z0-9._]+)*$)
MarketComplexType: String (maxLength: 50)
```

### Core Data Types

#### SecurityDefinition
| Field | Type | Description |
|-------|------|-------------|
| exchSym | Symbol | Exchange:Symbol |
| exchangeSource | String | |
| activationTime | Timestamp | |
| expirationTime | Timestamp | |
| marketComplex | String | |
| marketGroup | String | |
| marketSymbol | String | |
| cfiCode | String | |
| allowOpenOrders | bool | |
| maturityMonth | i32 | 1-12 |
| maturityYear | i32 | 2000-2100 |
| productDescription | String | |
| userDefinded | bool | |
| intradayDefinded | bool | |
| optionType | OptionType | |
| optionExpirationType | OptionExpirationType | |
| strikePrice | f64 | |
| underlyingSymbol | Symbol | |
| variableTickTableCode | i32 | |
| exchangeStrategyType | ExchangeStrategyType | |
| securityType | SecurityType | |
| securityId | String | |
| legs | Vec\<SecurityDefinitionLeg\> | |
| depthLevels | i32 | |
| mainFraction | f64 | |
| subFraction | f64 | |
| scale | i32 | |
| minPriceIncrement | f64 | |
| minPriceIncrementValue | f64 | |
| regCode | RegCodeType | |
| currencyCode | String | |
| displayFactor | f64 | |
| allowTrading | bool | |
| scalingFactorScreen | f64 | |
| exchangeSymbol | Symbol | |
| creationDate | Timestamp | |

#### SecurityDefinitionLeg
| Field | Type | Description |
|-------|------|-------------|
| symbol | Symbol | |
| ratio | i32 | |
| side | Side | BID or ASK |
| securityId | String | |
| exchange | String | |
| legExchangeSymbol | Symbol | |

#### SecurityMarginAndValue
| Field | Type | Description |
|-------|------|-------------|
| exchSym | Symbol | |
| currentPrice | f64 | |
| currentTime | Timestamp | |
| currentValue | f64 | |
| initialMarginLong | f64 | |
| initialMaginShort | f64 | Note: typo in API ("Magin") |
| maintMarginLong | f64 | |
| maintMarginShort | f64 | |
| spanSettlePrice | f64 | |
| spanSettleValue | f64 | |
| marginScheduleDetails | Vec\<MarginScheduleDetail\> | |

#### MarginScheduleDetail
| Field | Type | Description |
|-------|------|-------------|
| startTime | Timestamp | |
| endTime | Timestamp | |
| margin | f64 | |

#### SecurityStatus
| Field | Type | Description |
|-------|------|-------------|
| exchSym | Symbol | |
| status | SecurityStatusType | |
| statusValue | i32 | |
| dateTime | Timestamp | |
| tradeDate | Timestamp | |

#### SymbolInfo
| Field | Type | Description |
|-------|------|-------------|
| symbol | Symbol | |
| currency | String | |
| description | String | |
| hasQuotes | bool | |
| pipValue | f64 | |
| pipSize | f64 | |
| minTick | f64 | |
| qtyStep | f64 | |
| symbolType | String | Forex, Future, CFD, etc. |

#### Balance
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| accountId | String | yes | |
| currencyCode | String | yes | |
| cashBalance | f64 | | |
| cashBalanceAvailable | f64 | | |
| openTradeEquity | f64 | | |
| totalEquity | f64 | | |
| cashAddedToday | f64 | | |
| netLiquidity | f64 | | |
| netLiquidityAvailable | f64 | | |
| daysOnCall | i64 | | |
| balanceType | BalanceType | | |
| marginInfo | MarginInfo | | |

#### MarginInfo
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| currencyCode | String | |
| marginO | MarginDetail | |
| marginOW | MarginDetail | With orders |
| marginOWI | MarginDetail | With orders and implied |

#### MarginDetail
| Field | Type | Description |
|-------|------|-------------|
| marginError | String | |
| errorSymbols | String | |
| initialRiskMargin | f64 | |
| maintenanceRiskMargin | f64 | |
| initialTotalMargin | f64 | |
| maintenanceTotalMargin | f64 | |
| isEstimated | bool | |
| asOfTime | i64 | |

#### Position
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| currencyCode | String | |
| exchSym | Symbol | |
| positionId | String | |
| quantity | f64 | |
| price | f64 | |
| dateOpened | String | YYYYMMDD |
| side | PositionSide | LONG or SHORT |
| unrealizedPL | f64 | |

#### RiskInfo
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| regCode | RegCodeType | |
| currencyCode | String | |
| liquidationValue | f64 | Triggers liquidation if value falls to/below |
| startNetLiquidationValue | f64 | |
| currentNetLiquidationValue | f64 | |
| maxNetLiquidationValue | f64 | |
| maxNetLiquidationValueMultiDay | f64 | |
| liquidationEvents | Vec\<i32\> | Event codes (see below) |

Liquidation event codes: 1=AUTOLIQ_STARTED, 2=AUTOLIQ_SUCCESSFUL, 3=AUTOLIQ_ORDERS_FAILED, 4=AUTOLIQ_FAIL, 12=RESET_LIQUIDATION, 13=MANUALLIQ_STARTED, 14=MANUALLIQ_SUCCESSFUL

#### Order
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| orderId | String | yes | |
| strategyId | i64 | | |
| parentOrderId | String | | |
| accountId | String | yes | |
| exchSym | Symbol | yes | |
| status | OrderStatusType | yes | |
| side | OrderSide | yes | |
| quantity | f64 | yes | |
| limitPrice | f64 | | |
| stopPrice | f64 | | |
| orderType | OrderType | yes | |
| duration | DurationType | yes | |
| fillQuantity | f64 | | |
| fillPrice | f64 | | Average fill price |
| fillDate | String | | ISO 8601 datetime |
| childOrders | Vec\<String\> | | Stop loss / take profit order IDs |
| orderError | OrderError | | `{ errorCode: i32, errorText: String }` |

#### OrderFill
| Field | Type | Description |
|-------|------|-------------|
| orderId | String | |
| strategyId | i64 | |
| accountId | String | |
| exchSym | Symbol | |
| status | OrderStatusType | |
| side | OrderSide | |
| quantity | f64 | |
| price | f64 | |
| fillQuantity | f64 | |
| fillTotalQuantity | f64 | |
| fillPrice | f64 | |
| avgFillPrice | f64 | |
| fillDate | String | ISO 8601 datetime |
| timeOrderEvent | i64 | ms since epoch |
| orderUpdateId | String | |

#### QuoteFull (streaming-optimized short field names)
| Field | Type | Description |
|-------|------|-------------|
| s | Symbol | Exchange symbol |
| l | f64 | Last trade price |
| sz | i32 | Last trade size |
| ch | f64 | Change from previous settle |
| op | f64 | Open price |
| hi | f64 | High price |
| lo | f64 | Low price |
| ags | i32 | Aggressor side (0=INVALID,1=BUY,2=SELL) |
| td | i32 | Tick direction (0=PLUS,1=SAME,2=MINUS,255=INVALID) |
| stt | f64 | Settlement price |
| stts | String | Settlement trade date (YYYYMMDD) |
| sttst | i64 | Settlement send time (ms) |
| pstt | f64 | Previous settlement price |
| pstts | String | Previous settlement trade date |
| sttch | f64 | Settlement change |
| hb | f64 | High bid |
| la | f64 | Low ask |
| b | f64 | Bid price |
| bt | i64 | Bid time (ms) |
| bs | i64 | Bid size |
| ibc | i64 | Implied bid order count |
| ibs | i32 | Implied bid size |
| a | f64 | Ask price |
| at | i64 | Ask time (ms) |
| as | i64 | Ask size |
| ias | i64 | Implied ask size |
| iac | i64 | Implied ask order count |
| tt | i64 | Trade time (ms) |
| tdt | String | Trade date (YYYYMMDD) |
| secs | i32 | Security status (SecurityStatusType) |
| sdt | String | Session date (YYYYMMDD) |
| oi | i32 | Open interest |
| tv | i32 | Total volume |
| bv | i32 | Block volume |
| swv | i32 | Swaps volume |
| pv | i32 | Physical volume |

#### Depth
| Field | Type | Description |
|-------|------|-------------|
| s | Symbol | |
| b | Vec\<DepthLevel\> | Bid levels |
| a | Vec\<DepthLevel\> | Ask levels |

#### DepthLevel
| Field | Type | Description |
|-------|------|-------------|
| l | i32 | Level number (0-based) |
| t | i64 | Time (ms) |
| s | String | Side ("B" or "A") |
| p | f64 | Price |
| o | i32 | Order count |
| sz | f64 | Total size |
| ioc | i32 | Implied order count |
| is | f64 | Implied size |

#### Trade
| Field | Type | Description |
|-------|------|-------------|
| symbol | String | |
| price | f64 | |
| change | f64 | |
| size | f64 | |
| sequenceNumber | i64 | |
| sendTime | i64 | ms since epoch |
| tickDirection | TickDirection | |
| aggressorSide | AggressorSideType | |
| tradeDate | String | |
| tradeId | i64 | |
| totalVolume | f64 | |

#### TradeBar / TickBar / TimeBar / VolumeBar (identical shape)
| Field | Type | Description |
|-------|------|-------------|
| t | i64 | Time (ms) |
| o | f64 | Open |
| h | f64 | High |
| l | f64 | Low |
| c | f64 | Close |
| v | f64 | Volume |
| tc | i64 | Trade count |
| d | f64 | Delta |
| i | String | Indicator name |

### Streaming-Optimized Types (short field names)

These types are used in WebSocket `StreamResponse` messages. They mirror the REST types with abbreviated field names for bandwidth efficiency.

#### PositionOpt
`a`=accountId, `cc`=currencyCode, `s`=symbol, `pId`=positionId, `q`=quantity, `p`=price, `do`=dateOpened, `sd`=side, `upl`=unrealizedPL

#### OrderOpt (required: oid, a, s, st, sd, q, ot, dr)
`oid`=orderId, `sid`=strategyId, `poid`=parentOrderId, `a`=accountId, `s`=symbol, `st`=status, `sd`=side, `q`=quantity, `lp`=limitPrice, `sp`=stopPrice, `ot`=orderType, `dr`=duration, `fq`=fillQuantity, `fp`=fillPrice, `fd`=fillDate, `cor`=childOrders, `err`=orderError

#### OrderFillOpt
`oid`=orderId, `sid`=strategyId, `a`=accountId, `s`=symbol, `st`=status, `sd`=side, `q`=quantity, `p`=price, `fq`=fillQty, `ftq`=fillTotalQty, `fp`=fillPrice, `afp`=avgFillPrice, `fd`=fillDate, `t`=eventTime, `ouid`=orderUpdateId

#### RiskInfoOpt
`a`=accountId, `rc`=regCode, `cc`=currencyCode, `lv`=liquidationValue, `snlv`=startNetLiqValue, `cnlv`=currentNetLiqValue, `mnlv`=maxNetLiqValue, `le`=liquidationEvents

#### BalanceOpt (required: a, cc)
`a`=accountId, `cc`=currencyCode, `cb`=cashBalance, `ote`=openTradeEquity, `te`=totalEquity, `cba`=cashBalanceAvailable, `cbta`=cashAddedToday, `nl`=netLiquidity, `nla`=netLiquidityAvailable, `bt`=balanceType, `dc`=daysOnCall, `mi`=marginInfo

#### MarginInfoOpt
`a`=accountId, `cc`=currencyCode, `mo`=marginDetail, `mow`=marginDetailWithOrders, `mowi`=marginDetailWithOrdersAndImplied

#### MarginDetailOpt
`me`=marginError, `es`=errorSymbols, `irm`=initialRiskMargin, `mrm`=maintenanceRiskMargin, `itm`=initialTotalMargin, `mtm`=maintenanceTotalMargin, `ie`=isEstimated, `t`=asOfTime

#### TradeOpt
`s`=symbol, `p`=price, `ch`=change, `sz`=size, `sq`=sequenceNumber, `st`=sendTime, `td`=tickDirection, `as`=aggressorSide, `tdt`=tradeDate, `tid`=tradeId, `is`=isSettlement, `clx`=isCancelled, `spt`=systemPricedTrade, `ist`=investigationStatus, `bt`=blockTrade

#### AccountPositionsOpt
`a`=accountId, `p`=Vec\<PositionOpt\>

#### IndicatorValues
`n`=name (unique indicator value name), `fi`=fromIndex, `v`=Vec\<Vec\<String\>\> (2D array of values)

---

## Error Responses

All endpoints may return:
| Code | Schema | Description |
|------|--------|-------------|
| 400 | Error | Bad request / missing params |
| 401 | Error | Invalid/expired token |
| 403 | ErrorResponse403 | Forbidden |
| 406 | ErrorResponse406 | Not acceptable |
| 429 | ErrorResponse429 | Rate limited |
| 500 | - | Internal server error |

Error shape: `{ status: "ERROR", message: String, error: String }`
