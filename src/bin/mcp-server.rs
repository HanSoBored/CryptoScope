//! CryptoScope MCP Server
//!
//! Model Context Protocol (MCP) server that exposes CryptoScope API
//! capabilities to AI agents via JSON-RPC 2.0.
//!
//! Usage:
//! ```bash
//! cargo run --bin mcp-server
//! ```
//!
//! MCP Configuration (claude_desktop_config.json):
//! ```json
//! {
//!   "mcpServers": {
//!     "cryptoscope": {
//!       "command": "cargo",
//!       "args": ["run", "--bin", "mcp-server", "--"],
//!       "env": {
//!         "CRYPTOSCOPE_API_URL": "http://localhost:3000"
//!       }
//!     }
//!   }
//! }
//! ```

use cryptoscope::mcp;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    mcp::run().await
}
