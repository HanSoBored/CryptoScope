# CryptoScope 🔍

> Multi-exchange crypto intelligence tool

Fetch and analyze perpetual/derivative from crypto exchanges with a clean, modular TUI interface.

---

## Features

- ✅ Fetch all perpetual and derivative symbols from Exchange
- ✅ Support for both linear (USDT) and inverse categories
- ✅ Automatic pagination handling
- ✅ Filter by symbol name (case-insensitive search)
- ✅ Multiple output formats (text, JSON, interactive TUI)
- ✅ Modular architecture - easy to add new exchanges
- ✅ Fast execution (< 3 seconds for all symbols)
- ✅ Price Screener CLI — Fetch and display symbols with price changes
- ✅ Database Caching — Daily open prices cached in SQLite (once per day)
- ✅ Two Modes — Ticker mode (fast) and K-line mode (accurate)
- ✅ Filtering — Filter by min change %, min volume, symbol search, top N
- ✅ Color Output — Green for gains, red for lossers
- ✅ Multi-View TUI — Symbol List, Screener, and Stats Dashboard views
- ✅ Contract Type Filtering — Filter by Linear/Inverse Perpetual/Futures (keys `1`-`4`)
- ✅ Screener Table — Sortable table with Change%, Volume, and Symbol columns
- ✅ Mouse Support — Click rows, scrollbar, and header tabs for navigation
- ✅ Popup Messages — Auto-dismissing status/error notifications (5s or any key)

## Installation

```bash
# Clone and build
git clone https://github.com/HanSoBored/CryptoScope
cd CryptoScope
cargo build --release

# Or install directly
cargo install --path .
```

---

## Usage

### Basic Usage

```bash
# Launch interactive TUI (default: bybit, all categories)
cryptoscope

# Specify exchange
cryptoscope -e bybit
cryptoscope --exchange bybit

# Fetch only linear (USDT perpetual) symbols
cryptoscope --category linear

# Fetch only inverse perpetual symbols
cryptoscope --category inverse

# Combine exchange + category
cryptoscope -e bybit --category linear
```

### Screener Usage

```bash
# Run screener (default: kline mode, all categories)
cryptoscope screener

# Fast mode (ticker, rolling 24h price)
cryptoscope screener --mode ticker

# Accurate mode (k-line, true 00:00 UTC open)
cryptoscope screener --mode kline

# Specify exchange and category
cryptoscope screener -e bybit --category linear

# Show top 20 gainers/losers
cryptoscope screener --top 20

# Filter by minimum 5% change
cryptoscope screener --min-change 5

# Filter by specific symbol
cryptoscope screener --symbol BTC

# Force refresh cached data (clears stale cache)
cryptoscope screener --force-refresh

# Combined filters
cryptoscope screener --mode kline --top 50 --min-change 3 --min-volume 1000000
```

### Output Formats

```bash
# Interactive terminal UI (TUI)
cryptoscope

# Human-readable text output
cryptoscope --output text
# or use the convenience flag
cryptoscope --cli

# Machine-readable JSON output
cryptoscope --output json > symbols.json
```

**Note:** `--cli` is a shorthand for `--output text`. These flags conflict with each other.

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

### How It Works
1. **First Run:** Fetches daily open prices from Bybit API and caches in SQLite
2. **Subsequent Runs:** Uses cached open prices (auto-refreshes at 00:00 UTC)
3. **Current Prices:** Always fetched fresh from API
4. **Calculation:** `(current - open) / open * 100` for % change

### Database Location
- **Linux:** `~/.local/share/cryptoscope/data.db` or `~/cryptoscope/data.db`
- **macOS:** `~/Library/Application Support/cryptoscope/data.db`
- **Windows:** `%APPDATA%\cryptoscope\data.db`

### Contract Type Filtering

Symbols can be filtered by their contract type using keyboard shortcuts:

| Key | Contract Type | Abbreviation | Description |
|-----|---------------|--------------|-------------|
| `1` | Linear Perpetual | LP | USDT-settled, no expiry |
| `2` | Linear Futures | LF | USDT-settled, fixed expiry |
| `3` | Inverse Perpetual | IP | BTC-settled, no expiry |
| `4` | Inverse Futures | IF | BTC-settled, fixed expiry |
| `0` | All Types | - | Select all contract types |

### TUI Architecture

The TUI is built with a modular architecture:
- **Multi-View System** — Symbol List, Screener, and Stats Dashboard views
- **State Extraction** — `SymbolListState`, `ScreenerState`, and `PopupState` for clean separation
- **Shared Table Utilities** — `table_common` module for consistent table rendering across views
- **Modular Key Handling** — `key_handler` with `NavResult` enum for clean event dispatch
- **Mouse Support** — Stateless `MouseState` handler with `TableContext` abstraction

---

## Example Output

### TUI Output

![TUI](docs/image/TUI.jng)

The TUI features:
- **Multi-View Navigation** — Switch between Symbol List (`l`), Screener (`s`), and Stats Dashboard (`Tab`)
- **Symbol table** — Scrollable list with selection highlighting, contract type display, and search
- **Screener table** — Sortable table showing Open, Current, Change%, and Volume 24h with color-coded changes
- **Stats dashboard** — Toggle with `Tab` or click header tabs to view statistics
- **Search** — Press `/` to filter symbols in real-time
- **Contract Type Filtering** — Press `1`-`4` to filter by LP/LF/IP/IF, `0` to select all
- **Refresh** — Press `r` to re-fetch symbols from the API
- **Cyberpunk theme** - Dark UI with neon accent colors
- **Popup messages** - Status/error notifications auto-dismiss after 5 seconds (or press any key to dismiss immediately)

**Key bindings:**

| Key | Action |
|-----|--------|
| `q` / `Esc` | Quit |
| `j` / `↓` | Next symbol |
| `k` / `↑` | Previous symbol |
| `/` | Toggle search mode |
| `Tab` | Toggle symbol list / stats view |
| `s` | Switch to Screener view |
| `l` | Switch to Symbol List view |
| `r` | Refresh data |
| `1`-`4` | Filter by contract type (LP/LF/IP/IF) |
| `0` | Select all contract types |
| `S` | Toggle screener sort direction |
| `o` / `O` | Cycle screener sort field (Change% → Volume → Symbol) |

**Mouse support:**
- Scroll wheel to navigate
- Click rows to select symbols
- Click scrollbar track to page up/down
- Click header tabs to switch views
- Click header tabs to switch views

### Screener Output

```
Symbol     | Category |         Open |   Current |   Change % |   Volume 24h
-----------+----------+--------------+-----------+------------+-------------
BTCUSDT    | linear   |     84200.00 |  86500.00 |     +2.73% |      $1.52B
ETHUSDT    | linear   |      2450.00 |   2380.00 |     -2.86% |    $890.45M
SOLUSDT    | linear   |       145.50 |    152.30 |     +4.67% |    $456.12M
DOGEUSDT   | linear   |         0.15 |      0.14 |     -6.67% |    $234.56M
```

> Colors: 🟢 Green = gains, 🔴 Red = losses (terminal-dependent)

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
    LinPerp (Linear Perpetual): 606
    LinFut (Linear Futures): 36
    InvPerp (Inverse Perpetual): 23
    InvFut (Inverse Futures): 4

📋 Sample Symbols (first 20):
  0GUSDT, 1000000BABYDOGEUSDT, 1000000CHEEMSUSDT, ...
  ... and 649 more

✅ Fetch completed in 3.1s
```


---

### Adding a New Exchange

To add support for a new exchange (e.g., Binance):

1. Create `src/exchange/binance.rs`
2. Implement the `Exchange` trait
3. Add to the factory in `src/exchange/factory.rs`

That's it! No changes to existing code required.

### Troubleshooting
**Stale cache issues?** Run with `--force-refresh` to clear old data and fetch fresh open prices.

---

## Current Status

### Supported Exchanges

- ✅ Bybit V5 (linear + inverse perpetual/futures)

### Planned

- ⏳ Binance Futures
- ⏳ OKX Derivatives
- ⏳ Symbol comparison across exchanges

---

## License

GNU General Public License v3.0 (GPL-3.0)

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.
