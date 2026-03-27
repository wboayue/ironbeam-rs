# Simulated Trader/Account (Demo Only, Enterprise)

## POST /simulatedTraderCreate

Create a simulated trader.

**Body**: `{ FirstName, LastName, Address1, Address2?, City, State, Country, ZipCode, Phone, Email, Password, TemplateId }` (all String)

TemplateId values: `XAP50` ($50k), `XAP100` ($100k), `XAP150` ($150k)

**Response 200**: `{ TraderId: String }`

---

## POST /simulatedAccountAdd

Add account to existing trader. Body: `{ TraderId, Password, TemplateId }`

**Response 200**: `{ AccountId: String }`

---

## PUT /simulatedAccountReset

Reset account to initial state. Body: `{ AccountId, TemplateId }`

---

## DELETE /simulatedAccountExpire

Expire an account. Body: `{ AccountId }`

---

## POST /simulatedAccount/addCash

Add cash to account. Body: `{ AccountId: String, Amount: f32 }`

---

## GET /simulatedAccount/getCashReport/{accountId}

**Path**: `accountId`
**Query**: `startDate` (i64, YYYYMMDD), `endDate` (i64, YYYYMMDD)

**Response 200**: `{ AccountId, CashReport: Vec<{ amount: f64, entryDate: i64, availableDate: i64 }> }`

---

## POST /simulatedAccount/liquidate

Liquidate accounts. Body: `{ Accounts?, Groups?, ExceptAccounts?, ForceManualLiquidation?, UseManualLiquidationForIlliquidMarkets?, SendAccountEmail?, SendOfficeEmail? }`

---

## POST /simulatedAccount/setRisk

Set risk parameters.

**Body**:
| Field | Type | Required | Description |
|-------|------|----------|-------------|
| AccountId | String | yes | |
| LiquidationAccountValue | f64? | no | |
| LiquidationLossFromStartOfDay | f64? | no | |
| LiquidationLossFromHighOfDay | f64? | no | |
| LiquidationLossFromHighOfMultiday | f64? | no | |
| LiquidationPctLossFromStartOfDay | f64? | no | 0-100 |
| LiquidationPctLossFromHighOfDay | f64? | no | 0-100 |
| LiquidationPctLossFromHighOfMultiday | f64? | no | 0-100 |
| LiquidationPctMarginDeficiency | f64? | no | 0-100 |
| LiquidationMaxValueOverride | f64? | no | |
| ReducePositionsOnly | bool? | no | |
| RestoreTrading | bool? | no | |
| MarginScheduleName | String? | no | |
| TemplateId | String? | no | |
