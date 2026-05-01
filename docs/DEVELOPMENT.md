# Development Guide 💻

Complete guide for setting up and developing CryptoScope locally.

## Table of Contents

- [Prerequisites](#prerequisites)
- [Backend Development](#backend-development)
- [Frontend Development](#frontend-development)
- [Full Stack Development](#full-stack-development)
- [Code Quality](#code-quality)
- [Testing](#testing)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

### Required Software

| Tool | Version | Install Link |
|------|---------|--------------|
| Rust | 1.80+ | [rustup.rs](https://rustup.rs/) |
| Node.js | 20+ | [nodejs.org](https://nodejs.org/) |
| Git | Latest | [git-scm.com](https://git-scm.com/) |

### Build Dependencies

**Linux:**
```bash
sudo apt-get install -y pkg-config libssl-dev
```

**macOS:**
```bash
xcode-select --install
brew install openssl
```

**Windows:**
- Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- Install [OpenSSL for Windows](https://wiki.openssl.org/index.php/Binaries)

---

## Backend Development

### Setup

```bash
# Clone repository
git clone https://github.com/HanSoBored/CryptoScope
cd CryptoScope

# Install build dependencies (Linux)
sudo apt-get install -y pkg-config libssl-dev
```

### Running the Backend

```bash
# Development mode
cargo run

# With verbose logging
RUST_LOG=debug cargo run

# Release build
cargo run --release
```

### Common Commands

```bash
# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy -- -D warnings

# Generate password hash for admin auth
cargo run --bin generate-hash

# Check for outdated dependencies
cargo outdated
```

### Backend Development Workflow

1. **Make changes** to `src/` files
2. **Run** `cargo run` to test
3. **Format** with `cargo fmt`
4. **Lint** with `cargo clippy`
5. **Test** with `cargo test`

---

## Frontend Development

### Setup

```bash
# Navigate to frontend directory
cd frontend

# Install dependencies
npm install
```

### Running the Frontend

```bash
# Development server (requires backend on port 3000)
npm run dev

# Build for production
npm run build

# Production preview
npm run start

# Lint code
npm run lint
```

### Frontend Development Workflow

1. **Start backend** on port 3000 (see above)
2. **Make changes** to `frontend/src/` files
3. **Auto-refresh** happens automatically in dev mode
4. **Lint** with `npm run lint`
5. **Build** with `npm run build` before committing

---

## Full Stack Development

### Terminal Setup

Run both services simultaneously:

```bash
# Terminal 1: Start backend
cd CryptoScope
cargo run

# Terminal 2: Start frontend
cd CryptoScope/frontend
npm run dev
```

### Access Points

| Service | URL | Description |
|---------|-----|-------------|
| Frontend | http://localhost:3001 | Web UI |
| Backend API | http://localhost:3000 | REST API |
| API Docs | http://localhost:3000/api-docs/swagger-ui | Swagger UI |

### Hot Reload Setup

**Backend:**
- Uses `cargo run` which auto-recompiles on file changes
- For faster reload, install `cargo-watch`:
  ```bash
  cargo install cargo-watch
  cargo watch -x run
  ```

**Frontend:**
- Next.js dev server auto-refreshes on changes
- Fast Refresh preserves component state

---

## Code Quality

### Rust

```bash
# Format all code
cargo fmt

# Run linter with strict warnings
cargo clippy -- -D warnings

# Check for security issues
cargo audit
```

### TypeScript/JavaScript

```bash
# Navigate to frontend
cd frontend

# Format code
npm run format

# Run linter
npm run lint

# Type check
npm run type-check
```

### Pre-commit Hooks (Optional)

Set up automatic linting before commits:

```bash
# Install Husky
npm install -D husky
npx husky install

# Add pre-commit hook
npx husky add .husky/pre-commit "npm run lint && npm run format"
```

---

## Testing

### Backend Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run tests with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```

### Frontend Tests

```bash
cd frontend

# Run tests
npm test

# Run with coverage
npm test -- --coverage
```

### Integration Testing

```bash
# Start backend
cargo run

# In another terminal, test API
curl http://localhost:3000/health
curl http://localhost:3000/api/v1/exchanges
```

---

## Troubleshooting

### Build Errors

**Rust build fails:**
```bash
# Clean build cache
cargo clean
cargo build

# Update Rust
rustup update
```

**Frontend build fails:**
```bash
# Clear node modules
cd frontend
rm -rf node_modules package-lock.json
npm install
```

### Runtime Issues

**Backend won't start:**
```bash
# Check if port 3000 is in use
sudo lsof -i :3000

# Check .env file
cat .env
```

**Frontend can't connect to backend:**
```bash
# Verify backend is running
curl http://localhost:3000/health

# Check API URL configuration
cat frontend/.env.local
```

### Common Errors

| Error | Solution |
|-------|----------|
| `openssl-sys` build error | Install OpenSSL dev packages |
| `node-sass` build error | Use `sass` instead or update Node |
| Port already in use | Kill process or change port |
| Module not found | Run `npm install` or `cargo build` |

---

## Related Documentation

- [Deployment Guide](DEPLOYMENT.md) - Docker deployment
- [Configuration Guide](CONFIGURATION.md) - Environment variables
- [API Reference](API.md) - API endpoints
