# Types

## Enums

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

## Type Aliases

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

## Core Data Types

### SecurityDefinition
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

### SecurityDefinitionLeg
| Field | Type | Description |
|-------|------|-------------|
| symbol | Symbol | |
| ratio | i32 | |
| side | Side | BID or ASK |
| securityId | String | |
| exchange | String | |
| legExchangeSymbol | Symbol | |

### SecurityMarginAndValue
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

### MarginScheduleDetail
| Field | Type | Description |
|-------|------|-------------|
| startTime | Timestamp | |
| endTime | Timestamp | |
| margin | f64 | |

### SecurityStatus
| Field | Type | Description |
|-------|------|-------------|
| exchSym | Symbol | |
| status | SecurityStatusType | |
| statusValue | i32 | |
| dateTime | Timestamp | |
| tradeDate | Timestamp | |

### SymbolInfo
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

### Balance
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

### MarginInfo
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| currencyCode | String | |
| marginO | MarginDetail | |
| marginOW | MarginDetail | With orders |
| marginOWI | MarginDetail | With orders and implied |

### MarginDetail
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

### Position
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

### RiskInfo
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

### Order
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

### OrderFill
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

### QuoteFull (streaming-optimized short field names)
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

### Depth
| Field | Type | Description |
|-------|------|-------------|
| s | Symbol | |
| b | Vec\<DepthLevel\> | Bid levels |
| a | Vec\<DepthLevel\> | Ask levels |

### DepthLevel
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

### Trade
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

### TradeBar / TickBar / TimeBar / VolumeBar (identical shape)
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

## Streaming-Optimized Types (short field names)

These types are used in WebSocket `StreamResponse` messages. They mirror the REST types with abbreviated field names for bandwidth efficiency.

### PositionOpt
`a`=accountId, `cc`=currencyCode, `s`=symbol, `pId`=positionId, `q`=quantity, `p`=price, `do`=dateOpened, `sd`=side, `upl`=unrealizedPL

### OrderOpt (required: oid, a, s, st, sd, q, ot, dr)
`oid`=orderId, `sid`=strategyId, `poid`=parentOrderId, `a`=accountId, `s`=symbol, `st`=status, `sd`=side, `q`=quantity, `lp`=limitPrice, `sp`=stopPrice, `ot`=orderType, `dr`=duration, `fq`=fillQuantity, `fp`=fillPrice, `fd`=fillDate, `cor`=childOrders, `err`=orderError

### OrderFillOpt
`oid`=orderId, `sid`=strategyId, `a`=accountId, `s`=symbol, `st`=status, `sd`=side, `q`=quantity, `p`=price, `fq`=fillQty, `ftq`=fillTotalQty, `fp`=fillPrice, `afp`=avgFillPrice, `fd`=fillDate, `t`=eventTime, `ouid`=orderUpdateId

### RiskInfoOpt
`a`=accountId, `rc`=regCode, `cc`=currencyCode, `lv`=liquidationValue, `snlv`=startNetLiqValue, `cnlv`=currentNetLiqValue, `mnlv`=maxNetLiqValue, `le`=liquidationEvents

### BalanceOpt (required: a, cc)
`a`=accountId, `cc`=currencyCode, `cb`=cashBalance, `ote`=openTradeEquity, `te`=totalEquity, `cba`=cashBalanceAvailable, `cbta`=cashAddedToday, `nl`=netLiquidity, `nla`=netLiquidityAvailable, `bt`=balanceType, `dc`=daysOnCall, `mi`=marginInfo

### MarginInfoOpt
`a`=accountId, `cc`=currencyCode, `mo`=marginDetail, `mow`=marginDetailWithOrders, `mowi`=marginDetailWithOrdersAndImplied

### MarginDetailOpt
`me`=marginError, `es`=errorSymbols, `irm`=initialRiskMargin, `mrm`=maintenanceRiskMargin, `itm`=initialTotalMargin, `mtm`=maintenanceTotalMargin, `ie`=isEstimated, `t`=asOfTime

### TradeOpt
`s`=symbol, `p`=price, `ch`=change, `sz`=size, `sq`=sequenceNumber, `st`=sendTime, `td`=tickDirection, `as`=aggressorSide, `tdt`=tradeDate, `tid`=tradeId, `is`=isSettlement, `clx`=isCancelled, `spt`=systemPricedTrade, `ist`=investigationStatus, `bt`=blockTrade

### AccountPositionsOpt
`a`=accountId, `p`=Vec\<PositionOpt\>

### IndicatorValues
`n`=name (unique indicator value name), `fi`=fromIndex, `v`=Vec\<Vec\<String\>\> (2D array of values)
