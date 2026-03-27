# Authentication

## POST /auth

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

## POST /logout

Invalidate the current token.

**Response 200** (`SuccessResponse`): `{ status: "OK", message: "OK" }`
