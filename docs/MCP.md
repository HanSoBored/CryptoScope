# CryptoScope MCP Server

AI agent integration for CryptoScope via Model Context Protocol.

## Quick Start

```bash
# Run MCP server
cargo run --bin mcp-server
```

**Configure in AI Client:**

**Claude Desktop** (`claude_desktop_config.json`):
```json
{
  "mcpServers": {
    "cryptoscope": {
      "command": "cargo",
      "args": ["run", "--bin", "mcp-server"],
      "cwd": "/path/to/CryptoScope"
    }
  }
}
```

**Cursor** (`.cursor/mcp.json`):
```json
{
  "mcpServers": [{
    "name": "CryptoScope",
    "command": "cargo",
    "args": ["run", "--bin", "mcp-server"],
    "cwd": "/path/to/CryptoScope"
  }]
}
```

---

## Available Tools

| Tool | Purpose | Auth? |
|------|---------|-------|
| `get_exchanges` | List supported exchanges | No |
| `get_symbols` | Fetch trading symbols | No |
| `run_screener` | Price screening | No |
| `get_stats` | Cache statistics | No |
| `login` | JWT authentication | No |
| `refresh_cache` | Refresh cache | Yes |

### Tool Parameters

**`get_symbols`**
- `exchange` (required): `"bybit"`
- `category` (optional): `"linear"`, `"inverse"`, `"spot"`
- `search` (optional): Filter by name

**`run_screener`**
- `exchange` (optional, default: `"bybit"`)
- `min_change` (optional): Min price change %
- `min_volume` (optional): Min 24h volume
- `top` (optional): Limit results (max 1000)
- `search` (optional): Filter by name
- `contract_type` (optional): `"linear"` or `"inverse"`

**`login`**
- `username` (required)
- `password` (required)

> **See Also:** [API Reference](../API.md) for detailed endpoint documentation

---

## Configuration

MCP inherits config from CryptoScope API. Only MCP-specific settings:

| Variable | Default | Description |
|----------|---------|-------------|
| `CRYPTOSCOPE_API_URL` | `http://localhost:3000` | API endpoint |

> **See Also:** [Configuration Guide](../CONFIGURATION.md)

---

## Testing

```bash
# Test MCP module
cargo test mcp
```

---

## Security

- **Parameter Validation:** All inputs validated (exchange, category, search, contract_type)
- **JWT:** HS256 with `exp`, `iat`, `nbf` claims (24h expiration)
- **Token Storage:** In-memory, never exposed in responses

> **See Also:** [API Reference - Security](../API.md#authentication)

---

## References

- [MCP Specification](https://modelcontextprotocol.io/)
- [rmcp crate](https://docs.rs/rmcp)
- [API Reference](../API.md)
- [Configuration Guide](../CONFIGURATION.md)
