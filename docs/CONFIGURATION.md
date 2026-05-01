# Configuration Guide ⚙️

Complete guide for configuring CryptoScope.

## Table of Contents

- [Environment Variables](#environment-variables)
- [Required Variables](#required-variables)
- [Optional Variables](#optional-variables)
- [Database Configuration](#database-configuration)
- [Security Configuration](#security-configuration)
- [Configuration Examples](#configuration-examples)

---

## Environment Variables

### Quick Reference

| Category | Variables |
|----------|-----------|
| **Required** | `JWT_SECRET`, `ADMIN_USER`, `ADMIN_PASS_HASH`, `RUST_ENV`, `CORS_ORIGINS` |
| **Logging** | `RUST_LOG` |
| **Database** | `DATABASE_PATH` |
| **Server** | `HOST`, `PORT` |
| **CORS** | `CORS_PERMISSIVE`, `CORS_ORIGINS`, `RUST_ENV` |
| **Frontend** | `NEXT_PUBLIC_API_URL` |
| **Rate Limiting** | `RATE_LIMIT_PER_SECOND`, `RATE_LIMIT_BURST_SIZE` |

---

## Required Variables

These variables must be set for the application to run properly.

### JWT_SECRET

| Property | Value |
|----------|-------|
| Required | **Yes** |
| Default | None |
| Min Length | 32 characters |

JWT signing key for authentication tokens.

**Generate:**
```bash
openssl rand -base64 32
```

**Example:**
```bash
JWT_SECRET="your-super-secret-key-at-least-32-chars-long"
```

---

### ADMIN_USER

| Property | Value |
|----------|-------|
| Required | **Yes** |
| Default | `admin` |

Admin username for API authentication.

**Example:**
```bash
ADMIN_USER="admin"
```

---

### ADMIN_PASS_HASH

| Property | Value |
|----------|-------|
| Required | **Yes** |
| Default | None |
| Format | Argon2id (PHC string format) |

Argon2id password hash for admin authentication. **Never store plaintext passwords.**

**Generate:**
```bash
# Using the binary
cargo run --bin generate-hash

# In Docker
docker compose run backend cargo run --bin generate-hash
```

**Example:**
```bash
ADMIN_PASS_HASH="$argon2id$v=19$m=19456,t=2,p=1$..."
```

---

### RUST_ENV

| Property | Value |
|----------|-------|
| Required | **Yes (production)** |
| Values | `production`, `prod`, `development`, `dev` |

Environment mode. **Critical for production security.**

**Example:**
```bash
# Production (enforces strict CORS)
RUST_ENV=production

# Development (relaxed settings)
RUST_ENV=development
```

---

### CORS_ORIGINS

| Property | Value |
|----------|-------|
| Required | **Yes (production)** |
| Format | Comma-separated URLs |

Allowed origins for CORS. Required in production.

**Example:**
```bash
# Single origin
CORS_ORIGINS="https://yourdomain.com"

# Multiple origins
CORS_ORIGINS="https://yourdomain.com,https://app.yourdomain.com"

# Local development
CORS_ORIGINS="http://localhost:3001,http://localhost:3000"
```

---

## Optional Variables

### Logging

#### RUST_LOG

| Property | Value |
|----------|-------|
| Required | No |
| Default | `info` |
| Values | `error`, `warn`, `info`, `debug`, `trace` |

Backend logging level.

**Example:**
```bash
# Production (minimal logs)
RUST_LOG=warn

# Development (verbose)
RUST_LOG=debug

# Specific modules
RUST_LOG=cryptoscope=debug,hyper=info
```

---

### Database

#### DATABASE_PATH

| Property | Value |
|----------|-------|
| Required | No |
| Default | `./cryptoscope_data` |

SQLite database directory path.

**Example:**
```bash
# Default (relative to project root)
DATABASE_PATH="./cryptoscope_data"

# Absolute path
DATABASE_PATH="/var/lib/cryptoscope/data"

# Docker volume
DATABASE_PATH="/app/data"
```

---

### Server

#### HOST

| Property | Value |
|----------|-------|
| Required | No |
| Default | `0.0.0.0` |

Server bind host.

**Example:**
```bash
# All interfaces (Docker default)
HOST="0.0.0.0"

# Localhost only
HOST="127.0.0.1"
```

#### PORT

| Property | Value |
|----------|-------|
| Required | No |
| Default | `3000` |

Server port.

**Example:**
```bash
PORT="3000"
```

---

### CORS

#### CORS_PERMISSIVE

| Property | Value |
|----------|-------|
| Required | No |
| Default | `false` |
| Values | `true`, `false` |

Allow all origins. **Blocked in production** when `RUST_ENV=production`.

**Example:**
```bash
# Development only - allows any origin
CORS_PERMISSIVE=true

# Production - must use CORS_ORIGINS
CORS_PERMISSIVE=false
```

---

### Frontend

#### NEXT_PUBLIC_API_URL

| Property | Value |
|----------|-------|
| Required | No |
| Default | `http://localhost:3000` |

Frontend API URL. Frontend-only environment variable.

**Example:**
```bash
# Local development
NEXT_PUBLIC_API_URL="http://localhost:3000"

# Production
NEXT_PUBLIC_API_URL="https://api.yourdomain.com"
```

---

### Rate Limiting

#### RATE_LIMIT_PER_SECOND

| Property | Value |
|----------|-------|
| Required | No |
| Default | `50` |

Max requests per second for general API endpoints.

**Example:**
```bash
RATE_LIMIT_PER_SECOND="50"
```

#### RATE_LIMIT_BURST_SIZE

| Property | Value |
|----------|-------|
| Required | No |
| Default | `100` |

Burst size for rate limiter.

**Example:**
```bash
RATE_LIMIT_BURST_SIZE="100"
```

---

## Database Configuration

### Default Locations

| Platform | Default Path |
|----------|-------------|
| **Docker** | `/app/data/cryptoscope.db` (volume: `./data`) |
| **Linux/macOS/Windows** | `data/cryptoscope.db` (relative to project root) |

### Database Files

The following files are created in the database directory:

```
cryptoscope_data/
├── cryptoscope.db      # Main SQLite database
├── cryptoscope.db-shm  # SQLite shared memory
└── cryptoscope.db-wal  # SQLite write-ahead log
```

### Backup

```bash
# Backup database
cp data/cryptoscope.db data/cryptoscope.db.backup

# Or with timestamp
cp data/cryptoscope.db "data/cryptoscope.db.$(date +%Y%m%d)"
```

---

## Security Configuration

### Production Checklist

Before deploying to production:

- [ ] `RUST_ENV=production` is set
- [ ] `CORS_ORIGINS` is configured with your domain(s)
- [ ] `CORS_PERMISSIVE=false` (default)
- [ ] `JWT_SECRET` is a strong random value (32+ characters)
- [ ] `ADMIN_PASS_HASH` uses Argon2id hash (not plaintext)
- [ ] Database directory has proper permissions
- [ ] `RUST_LOG` is set to `warn` or `error` (not `debug`)

### Security Notes

| Setting | Risk if Misconfigured |
|---------|----------------------|
| `CORS_PERMISSIVE=true` in production | Allows any website to make API requests |
| Weak `JWT_SECRET` | Tokens can be forged |
| Plaintext password | Credentials exposed if config is leaked |
| `RUST_ENV` not set | CORS protections may be bypassed |

### Generate Secure Values

```bash
# JWT Secret (32+ random characters)
openssl rand -base64 32

# Password Hash (Argon2id)
cargo run --bin generate-hash
```

---

## Configuration Examples

### Development (.env)

```bash
# Environment
RUST_ENV=development
RUST_LOG=debug

# Authentication
JWT_SECRET="dev-secret-key-change-in-production-12345678"
ADMIN_USER=admin
ADMIN_PASS_HASH="$argon2id$v=19$m=19456,t=2,p=1$..."

# CORS (permissive for local dev)
CORS_PERMISSIVE=true

# Server
HOST=0.0.0.0
PORT=3000

# Database
DATABASE_PATH=./cryptoscope_data

# Frontend
NEXT_PUBLIC_API_URL=http://localhost:3000

# Rate limiting (relaxed for dev)
RATE_LIMIT_PER_SECOND=100
RATE_LIMIT_BURST_SIZE=200
```

### Production (.env)

```bash
# Environment
RUST_ENV=production
RUST_LOG=warn

# Authentication
JWT_SECRET="<generated-with-openssl-rand-base64-32>"
ADMIN_USER=admin
ADMIN_PASS_HASH="<generated-with-cargo-run-bin-generate-hash>"

# CORS (restrictive)
CORS_PERMISSIVE=false
CORS_ORIGINS=https://yourdomain.com,https://app.yourdomain.com

# Server
HOST=0.0.0.0
PORT=3000

# Database
DATABASE_PATH=/var/lib/cryptoscope/data

# Frontend
NEXT_PUBLIC_API_URL=https://api.yourdomain.com

# Rate limiting
RATE_LIMIT_PER_SECOND=50
RATE_LIMIT_BURST_SIZE=100
```

### Docker Compose Environment

```yaml
# docker-compose.yml
services:
  backend:
    environment:
      - RUST_ENV=production
      - JWT_SECRET=${JWT_SECRET}
      - ADMIN_USER=${ADMIN_USER}
      - ADMIN_PASS_HASH=${ADMIN_PASS_HASH}
      - CORS_ORIGINS=${CORS_ORIGINS}
      - DATABASE_PATH=/app/data
    volumes:
      - ./data:/app/data
```

---

## Troubleshooting

### JWT_SECRET Too Short

**Error:** `JWT_SECRET must be at least 32 characters`

**Solution:** Generate a secure secret:
```bash
openssl rand -base64 32
```

### CORS Request Blocked

**Error:** `Blocked by CORS policy` in browser console

**Solution:**
1. Set `CORS_ORIGINS` to your frontend URL (e.g., `http://localhost:3001`)
2. For development only: set `CORS_PERMISSIVE=true` (⚠️ Never in production)
3. Ensure `RUST_ENV=production` in production to enforce strict CORS

### Invalid ADMIN_PASS_HASH Format

**Error:** `Invalid password hash format` on login

**Solution:**
1. Generate a new hash using the provided tool:
   ```bash
   cargo run --bin generate-hash
   ```
2. Enter your password when prompted
3. Copy the full hash (starts with `$argon2id$`)
4. Paste into `.env` as `ADMIN_PASS_HASH`

### Database Path Issues

**Error:** `Failed to open database at [path]`

**Solution:**
1. Ensure the directory exists: `mkdir -p ./cryptoscope_data`
2. Check file permissions: `chmod 644 ./cryptoscope_data/cryptoscope.db`
3. Verify `DATABASE_PATH` in `.env` points to correct location

---

## Related Documentation

- [Deployment Guide](DEPLOYMENT.md) - Docker deployment
- [Development Guide](DEVELOPMENT.md) - Local development
- [API Reference](API.md) - API endpoints
