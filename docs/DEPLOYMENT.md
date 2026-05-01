# Deployment Guide 🚀

Complete guide for deploying CryptoScope with Docker.

## Table of Contents

- [Quick Start](#quick-start)
- [Docker Architecture](#docker-architecture)
- [Production Deployment](#production-deployment)
- [Development Mode](#development-mode)
- [Manual Docker Commands](#manual-docker-commands)
- [Troubleshooting](#troubleshooting)

---

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

---

## Docker Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Docker Network                           │
│                                                              │
│  ┌─────────────────┐         ┌─────────────────┐            │
│  │    Frontend     │         │     Backend     │            │
│  │   (Next.js)     │ ──────► │    (Axum/Rust)  │            │
│  │   Port: 3001    │         │    Port: 3000   │            │
│  └────────┬────────┘         └────────┬────────┘            │
│           │                           │                      │
└───────────┼───────────────────────────┼──────────────────────┘
             │                           │
             ▼                           ▼
     http://localhost:3001       http://localhost:3000
```

### Services

| Service | Port | Description |
|---------|------|-------------|
| `backend` | 3000 | Rust Axum API server |
| `frontend` | 3001 | Next.js web application |

### Volumes

| Volume | Mount Point | Purpose |
|--------|-------------|---------|
| `./data` | `/app/data` | SQLite database persistence |

---

## Production Deployment

### Prerequisites

- Docker 20+ and Docker Compose 2+
- `.env` file configured with required variables (see [Configuration](CONFIGURATION.md))
- Open ports 3000 and 3001 (or configure custom ports)

### Deployment Steps

1. **Clone and configure:**
   ```bash
   git clone https://github.com/HanSoBored/CryptoScope
   cd CryptoScope
   cp .env.example .env
   # Edit .env with your production values
   ```

2. **Set required environment variables:**
   ```bash
   # Generate JWT secret
   openssl rand -base64 32

   # Generate admin password hash
   docker compose run backend cargo run --bin generate-hash
   ```

3. **Start services:**
   ```bash
   docker compose up -d
   ```

4. **Verify deployment:**
   ```bash
   docker compose ps
   docker compose logs -f
   ```

### Production Security Checklist

- [ ] `RUST_ENV=production` is set
- [ ] `CORS_ORIGINS` is configured with your domain
- [ ] `CORS_PERMISSIVE=false` (default)
- [ ] `JWT_SECRET` is a strong random value (32+ chars)
- [ ] `ADMIN_PASS_HASH` uses Argon2id hash (not plaintext)
- [ ] Database volume is backed up regularly

### Updating Deployment

```bash
# Pull latest changes
git pull

# Rebuild and restart
docker compose up -d --build

# Or without rebuild (if using tagged images)
docker compose pull
docker compose up -d
```

---

## Development Mode

For active development with automatic code reloading:

### Start Development Environment

```bash
docker compose -f docker-compose.dev.yml up -d
```

### Features

| Component | Hot Reload |
|-----------|------------|
| Backend | ✅ Auto-recompiles on Rust code changes (cargo-watch) |
| Frontend | ✅ Auto-refreshes on Next.js code changes (npm run dev) |

### Development Workflow

```bash
# View logs
docker compose -f docker-compose.dev.yml logs -f

# Restart a specific service
docker compose -f docker-compose.dev.yml restart backend

# Stop development environment
docker compose -f docker-compose.dev.yml down
```

### Volume Mounts

Source code is mounted from host for live editing:
- Backend: `./src` → `/app/src`
- Frontend: `./frontend` → `/app/frontend`

---

## Manual Docker Commands

### Build Images

```bash
docker build -f Dockerfile.backend -t cryptoscope-backend:latest .
docker build -f Dockerfile.frontend -t cryptoscope-frontend:latest .
```

### Run Individual Services

```bash
# Run backend only
docker run -d -p 3000:3000 -v $(pwd)/data:/app/data cryptoscope-backend:latest

# Run frontend only
docker run -d -p 3001:3001 -e NEXT_PUBLIC_API_URL=http://localhost:3000 cryptoscope-frontend:latest
```

### Container Management

```bash
# View running containers
docker ps

# View logs
docker logs <container-id>

# Execute command in running container
docker exec -it <container-id> sh

# Remove container
docker rm <container-id>
```

---

## Troubleshooting

### Common Issues

**Backend won't start:**
```bash
# Check logs
docker compose logs backend

# Verify .env file exists and has required variables
cat .env
```

**Frontend can't connect to backend:**
```bash
# Verify NEXT_PUBLIC_API_URL is correct
docker compose exec frontend env | grep NEXT_PUBLIC

# Check backend is running
curl http://localhost:3000/health
```

**Database errors:**
```bash
# Check volume permissions
ls -la ./data

# Reset database (warning: deletes all data)
rm -rf ./data/*
docker compose up -d
```

**Port conflicts:**
```bash
# Check what's using the ports
sudo lsof -i :3000
sudo lsof -i :3001

# Or change ports in docker-compose.yml
```

### Health Checks

```bash
# Backend health
curl http://localhost:3000/health

# Frontend health
curl http://localhost:3001

# API documentation
curl http://localhost:3000/api-docs/openapi.json
```

---

## Related Documentation

- [Configuration Guide](CONFIGURATION.md) - Environment variables and settings
- [Development Guide](DEVELOPMENT.md) - Local development setup
- [API Reference](API.md) - API endpoints and authentication
