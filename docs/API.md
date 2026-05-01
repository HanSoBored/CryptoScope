# API Reference 📡

Complete API documentation for CryptoScope.

## Table of Contents

- [Overview](#overview)
- [Authentication](#authentication)
- [Public Endpoints](#public-endpoints)
- [Protected Endpoints](#protected-endpoints)
- [Rate Limiting](#rate-limiting)
- [Error Handling](#error-handling)
- [Examples](#examples)

---

## Overview

| Property | Value |
|----------|-------|
| Base URL | `http://localhost:3000` |
| API Version | `/api/v1` |
| Format | JSON |
| Documentation | `/api-docs/swagger-ui` |
| OpenAPI Spec | `/api-docs/openapi.json` |

### Interactive Documentation

When the server is running, access:
- **Swagger UI:** http://localhost:3000/api-docs/swagger-ui
- **OpenAPI JSON:** http://localhost:3000/api-docs/openapi.json

---

## Authentication

### Overview

| Property | Value |
|----------|-------|
| Type | JWT (JSON Web Token) |
| Password Hashing | Argon2id |
| Token Location | `Authorization` header |
| Format | `Bearer <token>` |

### Login

**Endpoint:** `POST /api/v1/auth/login`

**Request:**
```json
{
  "username": "admin",
  "password": "your-password"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2024-01-01T12:00:00Z"
}
```

### Using JWT Token

Include the token in all protected endpoint requests:

```bash
curl -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
     http://localhost:3000/api/v1/refresh
```

### Generate Password Hash

Before first login, generate a password hash:

```bash
# Using the binary
cargo run --bin generate-hash

# Or in Docker
docker compose run backend cargo run --bin generate-hash
```

---

## Public Endpoints

No authentication required.

### Health Check

**GET** `/health`

Check if the service is running.

**Response:**
```json
{
  "status": "healthy",
  "timestamp": "2024-01-01T12:00:00Z"
}
```

---

### List Exchanges

**GET** `/api/v1/exchanges`

Get list of available exchanges.

**Response:**
```json
[
  {
    "id": "bybit",
    "name": "Bybit V5",
    "supported": true
  }
]
```

---

### Get Symbols

**GET** `/api/v1/symbols`

Fetch symbols for an exchange.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `exchange` | string | Yes | Exchange ID (e.g., `bybit`) |
| `category` | string | No | `linear`, `inverse`, `spot` |
| `search` | string | No | Filter by symbol name |

**Example:**
```bash
curl "http://localhost:3000/api/v1/symbols?exchange=bybit&category=linear"
```

**Response:**
```json
[
  {
    "symbol": "BTCUSDT",
    "base": "BTC",
    "quote": "USDT",
    "contract_type": "linear_perpetual",
    "status": "trading"
  }
]
```

---

### Get Statistics

**GET** `/api/v1/stats`

Get cache statistics.

**Response:**
```json
{
  "total_symbols": 1250,
  "last_updated": "2024-01-01T12:00:00Z",
  "cache_hit_rate": 0.95
}
```

---

### Price Screener

**GET** `/api/v1/screener`

Run market screener with filters.

**Query Parameters:**

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `exchange` | string | `bybit` | Exchange ID |
| `min_change` | number | `0` | Minimum price change % |
| `min_volume` | number | `0` | Minimum 24h volume |
| `top` | integer | `0` | Limit to top N results |
| `search` | string | - | Filter by symbol name |
| `contract_type` | string | - | `linear`, `inverse`, etc. |

**Example:**
```bash
curl "http://localhost:3000/api/v1/screener?exchange=bybit&min_change=5&top=10"
```

**Response:**
```json
[
  {
    "symbol": "BTCUSDT",
    "price_change_24h": 5.23,
    "volume_24h": 1500000000,
    "current_price": 45000.00
  }
]
```

---

### Login (Public)

**POST** `/api/v1/auth/login`

Authenticate and get JWT token.

**Request Body:**
```json
{
  "username": "admin",
  "password": "your-password"
}
```

**Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2024-01-01T12:00:00Z"
}
```

---

## Protected Endpoints

Requires JWT authentication.

### Refresh Cache

**POST** `/api/v1/refresh`

Manually refresh the price cache (admin only).

**Headers:**
```
Authorization: Bearer <token>
```

**Response:**
```json
{
  "status": "refreshed",
  "symbols_updated": 1250,
  "timestamp": "2024-01-01T12:00:00Z"
}
```

**Example:**
```bash
curl -X POST \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  http://localhost:3000/api/v1/refresh
```

---

## Rate Limiting

### Limits

| Endpoint Type | Rate Limit | Burst Size |
|---------------|------------|------------|
| General API | 50 req/s | 100 |
| Authentication | 10 req/min | 5 |
| Cache Refresh | 5 req/min | 2 |

### Configuration

Rate limits are configurable via environment variables:

```bash
RATE_LIMIT_PER_SECOND=50
RATE_LIMIT_BURST_SIZE=100
```

### Rate Limit Headers

Responses include rate limit information:

```
X-RateLimit-Limit: 50
X-RateLimit-Remaining: 45
X-RateLimit-Reset: 1704110400
```

### Rate Limit Exceeded

**Response (429):**
```json
{
  "error": "Rate limit exceeded",
  "retry_after": 60
}
```

---

## Error Handling

### Error Response Format

```json
{
  "error": "Error message",
  "code": "ERROR_CODE",
  "details": {}
}
```

### Common Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| `UNAUTHORIZED` | 401 | Missing or invalid JWT token |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `BAD_REQUEST` | 400 | Invalid request parameters |
| `RATE_LIMITED` | 429 | Too many requests |
| `INTERNAL_ERROR` | 500 | Server error |

### Example Error

**Request:** Invalid token
```bash
curl -H "Authorization: Bearer invalid-token" \
     http://localhost:3000/api/v1/refresh
```

**Response (401):**
```json
{
  "error": "Invalid token",
  "code": "UNAUTHORIZED"
}
```

---

## Examples

### Complete Workflow

```bash
# 1. Login and get token
TOKEN=$(curl -s -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"your-password"}' \
  | jq -r '.token')

# 2. Get symbols
curl "http://localhost:3000/api/v1/symbols?exchange=bybit"

# 3. Run screener
curl "http://localhost:3000/api/v1/screener?min_change=5&top=10"

# 4. Refresh cache (protected)
curl -X POST http://localhost:3000/api/v1/refresh \
  -H "Authorization: Bearer $TOKEN"

# 5. Get stats
curl http://localhost:3000/api/v1/stats
```

### Using with JavaScript

```javascript
const API_BASE = 'http://localhost:3000/api/v1';

// Login
async function login(username, password) {
  const res = await fetch(`${API_BASE}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ username, password })
  });
  const { token } = await res.json();
  return token;
}

// Fetch symbols
async function getSymbols(exchange, token) {
  const res = await fetch(`${API_BASE}/symbols?exchange=${exchange}`, {
    headers: { 'Authorization': `Bearer ${token}` }
  });
  return res.json();
}
```

### Using with Python

```python
import requests

API_BASE = 'http://localhost:3000/api/v1'

# Login
def login(username, password):
    res = requests.post(f'{API_BASE}/auth/login',
                       json={'username': username, 'password': password})
    return res.json()['token']

# Fetch symbols
def get_symbols(exchange, token):
    headers = {'Authorization': f'Bearer {token}'}
    res = requests.get(f'{API_BASE}/symbols?exchange={exchange}',
                      headers=headers)
    return res.json()
```

---

## Troubleshooting

### Token Expired

**Error:** `401 Unauthorized` with message "Token has expired"

**Solution:**
1. Tokens expire after 24 hours by default
2. Re-authenticate: `POST /api/v1/auth/login` with your credentials
3. Store the new token and update your Authorization header

### Rate Limit Exceeded

**Error:** `429 Too Many Requests`

**Solution:**
1. Check the `Retry-After` header for how long to wait
2. Default limits:
   - Public endpoints: 50 req/s (burst: 100)
   - Authenticated endpoints: 10 req/s (burst: 20)
   - Cache refresh: 2 req/s (burst: 5)
3. Implement exponential backoff in your client
4. For higher limits: adjust `RATE_LIMIT_PER_SECOND` in config

### Swagger UI Not Loading

**Error:** Blank page at `/api-docs/swagger-ui`

**Solution:**
1. Ensure backend is running on port 3000
2. Check browser console for CORS errors
3. Verify `CORS_ORIGINS` includes your browser's origin
4. Try the OpenAPI JSON directly: `/api-docs/openapi.json`

### 404 on API Endpoints

**Error:** `404 Not Found` on valid endpoints

**Solution:**
1. Ensure you're using the correct base path: `/api/v1/...`
2. Check that the endpoint exists (see [Public Endpoints](#public-endpoints))
3. Verify backend logs for routing errors

---

## Related Documentation

- [Configuration Guide](CONFIGURATION.md) - Environment variables
- [Deployment Guide](DEPLOYMENT.md) - Docker setup
- [Development Guide](DEVELOPMENT.md) - Local development
