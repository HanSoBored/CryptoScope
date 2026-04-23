# CryptoScope 🔍

**Multi-exchange crypto symbols intelligence tool**

Fetch and analyze perpetual/derivative symbols from crypto exchanges with a clean, modular CLI interface.

## Features

- ✅ Fetch all perpetual and derivative symbols from Bybit V5 API
- ✅ Support for both linear (USDT) and inverse categories
- ✅ Automatic pagination handling
- ✅ Filter by symbol name or status
- ✅ Multiple output formats (text, JSON, interactive TUI)
- ✅ Modular architecture - easy to add new exchanges
- ✅ Fast execution (< 3 seconds for all symbols)

## Installation

```bash
# Clone and build
git clone https://github.com/HanSoBored/CryptoScope
cd cryptoscope
cargo build --release

# Or install directly
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Fetch all symbols from Bybit (linear + inverse)
cryptoscope

# Fetch only linear (USDT perpetual) symbols
cryptoscope --category linear

# Fetch only inverse perpetual symbols
cryptoscope --category inverse
```

### Output Formats

```bash
# Human-readable text output (default)
cryptoscope --output text

# Machine-readable JSON output
cryptoscope --output json > symbols.json

# Interactive terminal UI (TUI)
cryptoscope --output tui
```

### Filtering

```bash
# Search for symbols containing "BTC"
cryptoscope --search BTC

# Combine filters
cryptoscope --search ETH --category linear
```

### Other Options

```bash
# Enable verbose logging
cryptoscope --verbose

# See all available options
cryptoscope --help
```

## Example Output

### Text Output

```
=== CryptoScope: BYBIT Perpetual Symbols ===

Exchange: BYBIT
Categories: linear, inverse

📊 Statistics:
  Total Symbols: 669

  By Category:
    INVERSE (Inverse Perpetual): 27
    LINEAR (USDT Perpetual): 642

  By Contract Type:
    LinearPerpetual: 606
    LinearFutures: 36
    InversePerpetual: 23
    InverseFutures: 4

📋 Sample Symbols (first 20):
  0GUSDT, 1000000BABYDOGEUSDT, 1000000CHEEMSUSDT, ...
  ... and 649 more

✅ Fetch completed in 3.1s
```

### TUI Output

Launch the interactive terminal UI:

```bash
cryptoscope --output tui
```

The TUI features:
- **Symbol table** - Scrollable list with selection highlighting
- **Stats dashboard** - Toggle with `Tab` to view statistics
- **Search** - Press `/` to filter symbols in real-time
- **Refresh** - Press `r` to re-fetch symbols from the API
- **Cyberpunk theme** - Dark UI with neon accent colors

**Key bindings:**

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `j` / `↓` | Next symbol |
| `k` / `↑` | Previous symbol |
| `/` | Toggle search mode |
| `Tab` | Toggle symbol list / stats view |
| `r` | Refresh data |

## Architecture

CryptoScope uses a trait-based architecture for easy extensibility:

```
┌─────────────────────────────────────┐
│           CLI Layer                 │
│  (main.rs + cli.rs)                 │
└─────────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────┐
│      Exchange Trait                 │
│  (exchange/exchange_trait.rs)       │
└─────────────────────────────────────┘
         ▲                  ▲
         │                  │
  ┌──────┴──────┐   ┌───────┴────────┐
  │ BybitClient │   │ BinanceClient  │ (future)
  │  (v1.0)     │   │  (v2.0)        │
  └─────────────┘   └────────────────┘
```

### Adding a New Exchange

To add support for a new exchange (e.g., Binance):

1. Create `src/exchange/binance.rs`
2. Implement the `Exchange` trait
3. Add to the factory in `src/exchange/factory.rs`

That's it! No changes to existing code required.

## Project Structure

```
cryptoscope/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Entry point
│   ├── cli.rs                  # CLI argument parsing
│   ├── error.rs                # Error types
│   ├── models/
│   │   ├── mod.rs
│   │   ├── symbol.rs           # Symbol struct
│   │   ├── response.rs         # API responses
│   │   └── statistics.rs       # Statistics aggregation
│   ├── exchange/
│   │   ├── mod.rs
│   │   ├── exchange_trait.rs   # Exchange trait
│   │   ├── bybit.rs            # Bybit implementation
│   │   └── factory.rs          # Exchange factory
│   ├── fetcher/
│   │   ├── mod.rs
│   │   └── instrument_fetcher.rs
│   ├── output/
│   │   ├── mod.rs
│   │   ├── formatter.rs        # Text output
│   │   └── json_output.rs      # JSON output
│   └── tui/
│       ├── mod.rs
│       ├── app.rs              # App state management
│       ├── runner.rs           # TUI event loop
│       ├── theme.rs            # Cyberpunk color theme
│       └── widgets/
│           ├── mod.rs
│           ├── header.rs       # Header widget
│           ├── footer.rs       # Footer widget
│           ├── popup.rs        # Popup/notification widget
│           ├── stats_panel.rs  # Stats dashboard widget
│           └── symbol_table.rs # Symbol table widget
└── tests/
```

## Tech Stack

- **tokio** - Async runtime
- **reqwest** - HTTP client
- **serde + serde_json** - JSON serialization
- **clap** - CLI framework
- **thiserror + anyhow** - Error handling
- **tracing** - Logging
- **ratatui** - Terminal UI framework
- **crossterm** - Terminal manipulation
- **unicode-width** - Unicode string width calculation

## Current Status

### Supported Exchanges

- ✅ Bybit V5 (linear + inverse perpetual/futures)

### Planned

- ⏳ Binance Futures
- ⏳ OKX Derivatives
- ⏳ Symbol comparison across exchanges

## License

GNU General Public License v3.0 (GPL-3.0)

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
