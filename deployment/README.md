# Deployment

Docker configuration for CryptoScope.

## Structure

```
deployment/docker/
├── Dockerfile.backend       # Backend Rust + Axum
├── Dockerfile.frontend      # Frontend Next.js
├── docker-compose.yml       # Production configuration
├── docker-compose.dev.yml   # Development configuration (hot-reload)
└── .dockerignore
```

## Usage

### Production Build

```bash
cd deployment/docker
docker-compose up -d --build
```

### Development (with hot-reload)

```bash
cd deployment/docker
docker-compose -f docker-compose.dev.yml up -d
```

### Access Services

- **Backend API**: http://localhost:3000
- **Frontend**: http://localhost:3001
- **SQLite Database**: `deployment/docker/data/cryptoscope.db`

### Shutdown

```bash
cd deployment/docker
docker-compose down
```

For development:
```bash
cd deployment/docker
docker-compose -f docker-compose.dev.yml down
```

### Cleanup (remove volumes)

```bash
cd deployment/docker
docker-compose down -v
```
