# CryptoScope 🔍

> Multi-exchange crypto intelligence platform with real-time streaming, screening, and analytics.

## Quick Start

```bash
git clone https://github.com/HanSoBored/CryptoScope
cd CryptoScope
cp .env.example .env
docker compose up -d
```

**Access:**
- Frontend: http://localhost:3001
- Backend: http://localhost:3000
- API Docs: http://localhost:3000/api-docs/swagger-ui

> ⚠️ **Notice:** CLI/TUI interface has been deprecated. Use the web API and frontend instead.

## Features

- ✅ **Price Screener** — Filter symbols by price change %, volume, and contract type
- ✅ **Database Caching** — Daily open prices cached in SQLite for fast queries
- ✅ **K-line Mode** — Accurate 00:00 UTC daily open for price calculations
- ✅ **Multi-Exchange** — Modular architecture for easy exchange integration
- ✅ **Web UI** — Modern Next.js frontend with real-time updates
- ✅ **Authentication** — JWT-based auth with Argon2id password hashing
- ✅ **Docker Support** — Production and development Docker Compose setups
- ✅ **Fast Execution** — Sub-3-second fetch for all symbols

## Installation

```bash
# Build from source
git clone https://github.com/HanSoBored/CryptoScope
cd CryptoScope
cargo build --release
```

**Prerequisites:** Rust 1.80+, Node.js 20+, Linux: `sudo apt-get install -y pkg-config libssl-dev`

## Documentation

| Guide | Description |
|-------|-------------|
| 📖 [Deployment Guide](docs/DEPLOYMENT.md) | Docker setup, production deployment, troubleshooting |
| 💻 [Development Guide](docs/DEVELOPMENT.md) | Local dev setup, hot reload, code quality |
| 📡 [API Reference](docs/API.md) | Endpoints, authentication, examples |
| ⚙️ [Configuration](docs/CONFIGURATION.md) | Environment variables, security settings |

## Current Status

**Supported:** Bybit V5 (linear + inverse perpetual/futures)

**Planned:** Binance Futures, OKX Derivatives, cross-exchange symbol comparison

## Project Structure

```
CryptoScope/
├── src/              # Rust backend (Axum API, exchange integrations)
├── frontend/         # Next.js web application
├── docs/             # Documentation (deployment, development, API, config)
├── data/             # SQLite database (gitignored)
└── docker-compose.yml
```

## Configuration

**Required:** `JWT_SECRET`, `ADMIN_USER`, `ADMIN_PASS_HASH`, `CORS_ORIGINS`, `RUST_ENV`

See [Configuration Guide](docs/CONFIGURATION.md) for all environment variables and options.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## License

GNU General Public License v3.0 (GPL-3.0)
