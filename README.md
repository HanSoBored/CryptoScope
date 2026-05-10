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
- ✅ **MCP Server** — AI agent integration via Model Context Protocol
- ✅ **Security** — Parameter validation, JWT nbf claims, path traversal prevention
- ✅ **Docker Support** — Production and development Docker Compose setups
- ✅ **Fast Execution** — Sub-3-second fetch for all symbols

## Installation

```bash
# Build from source
git clone https://github.com/HanSoBored/CryptoScope
cd CryptoScope
cargo build --release
```

**Prerequisites:** Rust 1.88+, Node.js 20+, Linux: `sudo apt-get install -y pkg-config libssl-dev`

## Documentation

| Guide | Description |
|-------|-------------|
| 📖 [Deployment Guide](docs/DEPLOYMENT.md) | Docker setup, production deployment, troubleshooting |
| 💻 [Development Guide](docs/DEVELOPMENT.md) | Local dev setup, hot reload, code quality |
| 📡 [API Reference](docs/API.md) | REST API endpoints, authentication, examples |
| ⚙️ [Configuration](docs/CONFIGURATION.md) | Environment variables, security settings |
| 🤖 [MCP Server](docs/mcp/README.md) | AI agent integration (Claude, Cursor, etc.) |

## Current Status

**Supported:** Bybit V5 (linear + inverse perpetual/futures)

**Planned:** Binance Futures, OKX Derivatives, cross-exchange symbol comparison

## Project Structure

```
CryptoScope/
├── src/
│   ├── api/              # REST API endpoints (Axum)
│   ├── core/
│   │   ├── exchange/     # Exchange integrations (Bybit, etc.)
│   │   ├── screener/     # Price screening logic
│   │   ├── db/           # Database repository
│   │   └── security.rs   # Parameter validation, path safety
│   ├── mcp/
│   │   ├── mod.rs        # MCP server implementation
│   │   ├── client.rs     # HTTP client for API
│   │   └── types.rs      # MCP parameter types
│   └── bin/
│       └── mcp-server.rs # MCP server binary
├── frontend/             # Next.js web application
├── docs/                 # Documentation
├── data/                 # SQLite database (gitignored)
└── docker-compose.yml
```

## Configuration

**Required:** `JWT_SECRET`, `ADMIN_USER`, `ADMIN_PASS_HASH`, `CORS_ORIGINS`, `RUST_ENV`

See [Configuration Guide](docs/CONFIGURATION.md) for all environment variables and options.

## MCP Server (AI Integration)

CryptoScope provides an MCP server for AI agents to interact with market data:

```bash
# Run MCP server
cargo run --bin mcp-server

# Configure in Claude Desktop or Cursor
# See docs/mcp/README.md for setup instructions
```

**Available Tools:**
- `get_exchanges` — List supported exchanges
- `get_symbols` — Fetch trading symbols
- `run_screener` — Price screening with filters
- `get_stats` — Cache statistics
- `login` — JWT authentication
- `refresh_cache` — Manual cache refresh (protected)

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.

## License

GNU General Public License v3.0 (GPL-3.0)
