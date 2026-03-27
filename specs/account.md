# Account

## GET /account/{accountId}/balance

Get account balance.

**Path**: `accountId` (String)
**Query**: `balanceType` (required): `CURRENT_OPEN` | `START_OF_DAY`

**Response 200** (`AccountBalanceResponse`):
| Field | Type | Description |
|-------|------|-------------|
| balances | Vec\<Balance\> | See Balance in [types.md](types.md) |

---

## GET /account/{accountId}/positions

Get open positions.

**Path**: `accountId` (String)

**Response 200** (`PositionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| accountId | String | |
| positions | Vec\<Position\> | See Position in [types.md](types.md) |

---

## GET /account/{accountId}/risk

Get risk info.

**Path**: `accountId` (String)

**Response 200** (`AccountRiskResponse`):
| Field | Type | Description |
|-------|------|-------------|
| risks | Vec\<RiskInfo\> | See RiskInfo in [types.md](types.md) |

---

## GET /account/{accountId}/fills

Get account fills.

**Path**: `accountId` (String)

**Response 200** (`AccountFillsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| fills | Vec\<OrderFill\> | See OrderFill in [types.md](types.md) |

---

## GET /account/getAllAccounts

Get all accounts for the authenticated trader.

**Response 200** (`AllAccountsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| accounts | Vec\<AccountId\> | |

---

## GET /account/getAllBalances

Get balances for all accounts.

**Query**: `balanceType` (required): `CURRENT_OPEN` | `START_OF_DAY`

**Response 200** (`AccountBalanceResponse`)

---

## GET /account/getAllFills

Get fills for all accounts.

**Response 200** (`AccountFillsResponse`)

---

## GET /account/getAllPositions

Get positions for all accounts.

**Response 200** (`AccountsPositionsResponse`):
| Field | Type | Description |
|-------|------|-------------|
| positions | Vec\<Positions\> | Each entry has accountId + Vec\<Position\> |

---

## GET /account/getAllRiskInfo

Get risk info for all accounts.

**Response 200** (`AccountRiskResponse`)
