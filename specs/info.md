# Information

## GET /info/trader

Get trader info (accounts list, live/demo status).

**Query**: `traderId` (String, optional)

**Response 200** (`TraderInfoResponse`):
| Field | Type | Description |
|-------|------|-------------|
| accounts | Vec\<AccountId\> | List of account IDs |
| isLive | bool | Whether this is a live account |
| traderId | String | Trader identifier |

---

## GET /info/user

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

## GET /info/security/definitions

Get security definitions for given symbols.

**Query**: `symbols` (comma-separated, required, max 10). Format: `EXCHANGE:SYMBOL`, e.g. `XCME:ES.U16`

**Response 200** (`SecurityDefinitionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| securityDefinitions | Vec\<SecurityDefinition\> | See [types.md](types.md) |

---

## GET /info/security/margin

Get margin and value info for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`SecurityMarginAndValueResponse`):
| Field | Type | Description |
|-------|------|-------------|
| securityMarginAndValues | Vec\<SecurityMarginAndValue\> | See [types.md](types.md) |

---

## GET /info/security/status

Get trading status for symbols.

**Query**: `symbols` (comma-separated, required, max 10)

**Response 200** (`SecurityStatusResponse`):
| Field | Type | Description |
|-------|------|-------------|
| securityStatuses | Vec\<SecurityStatus\> | See [types.md](types.md) |

---

## GET /info/symbols

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
| symbols | Vec\<SymbolInfo\> | See [types.md](types.md) |

---

## GET /info/exchangeSources

Get list of available exchanges.

**Response 200** (`ExchangeSourcesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| exchanges | Vec\<String\> | Exchange identifiers |

---

## GET /info/complexes/{exchange}

Get market complexes for an exchange.

**Path**: `exchange` (String, e.g. "XCME")

**Response 200** (`ComplexesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| marketComplexes | Vec\<ComplexGroups\> | Groups with name and sub-groups |

---

## GET /info/symbol/search/futures/{exchange}/{marketGroup}

Search for futures symbols.

**Path**: `exchange` (String), `marketGroup` (String, e.g. "ES")

**Response 200** (`SymbolFuturesResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbols | Vec\<FutureInfo\> | symbol, maturityMonth, maturityYear, description |

---

## GET /info/symbol/search/groups/{complex}

Get symbol groups by market complex.

**Path**: `complex` (String, e.g. "Currency")

**Response 200** (`ComplexGroupsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbolGroups | Vec\<ComplexGroupInfo\> | group name + display name |

---

## GET /info/symbol/search/options/{symbol}

Get option groups for a symbol.

**Path**: `symbol` (Symbol)

**Response 200** (`SymbolOptionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| groups | Vec\<String\> | |
| optionGroups | Vec\<OptionGroupInfo\> | group, expiration, description |

---

## GET /info/symbol/search/options/ext/{symbol}/{group}/{optionType}/{near}

Search for specific options.

**Path**: `symbol` (Symbol), `group` (String), `optionType` ("call" | "put"), `near` (bool)

**Response 200** (`SymbolSearchOptionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbolOptions | Vec\<Symbol\> | Matching option symbols |

---

## GET /info/symbol/search/options/spreads/{symbol}

Get available option spreads.

**Path**: `symbol` (Symbol)

**Response 200** (`SymbolOptionSpreadsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| symbolSpreads | Vec\<Spread\> | Spread definitions |

---

## GET /info/strategyId

Get a new strategy ID for order grouping.

**Response 200** (`StrategyIdResponse`):
| Field | Type | Description |
|-------|------|-------------|
| Id | i64 | Strategy ID |
| Minimum | i64 | Min value in range |
| Maximum | i64 | Max value in range |
