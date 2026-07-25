# VigilantAI

### AI-Powered Security Surveillance Platform

![Build Status](https://img.shields.io/badge/build-passing-brightgreen)
![License](https://img.shields.io/badge/license-MIT-blue)
![Version](https://img.shields.io/badge/version-1.0.0-orange)
![Rust](https://img.shields.io/badge/language-Rust-orange)
![Python](https://img.shields.io/badge/language-Python-3776AB)
![Next.js](https://img.shields.io/badge/framework-Next.js-black)

---

## Overview

VigilantAI Enterprise v1.0 is a full-stack, AI-powered security surveillance platform designed for real-time threat detection, intelligent video analytics, and centralized security operations. It combines edge computing, deep learning, and modern web technologies to deliver a scalable, production-ready surveillance solution.

### Key Features

- **Real-time Object Detection** — YOLOv8-powered detection running at the edge with sub-100ms inference
- **Facial Recognition** — High-accuracy identification and verification with anti-spoofing
- **Behavioral Analytics** — Anomaly detection for loitering, intrusion, and unusual patterns
- **Event Correlation Engine** — Cross-camera event linking and timeline reconstruction
- **Multi-camera Tracking** — Person and vehicle re-identification across geographically distributed cameras
- **Alert Management** — Configurable alert rules with escalation policies and notification channels
- **Edge-Cloud Hybrid** — On-device inference with cloud-based aggregation and model training
- **Role-based Access Control** — Granular permissions with SSO and LDAP integration
- **Audit Logging** — Immutable, tamper-evident logs for compliance (SOC 2, GDPR)
- **RESTful & WebSocket APIs** — Full programmatic access for integration with third-party systems
- **Distributed Architecture** — Horizontally scalable microservices on Kubernetes
- **Responsive Web Dashboard** — Modern UI for monitoring, investigation, and system configuration

### Technology Stack

| Layer            | Technology                         |
|------------------|------------------------------------|
| Edge Inference   | Rust, ONNX Runtime, CUDA           |
| Model Training   | Python, PyTorch, Ultralytics       |
| API Gateway      | Rust (Actix-Web), gRPC             |
| Web Dashboard    | Next.js 14, TypeScript, Tailwind   |
| Message Broker   | NATS JetStream                     |
| Time-series DB   | TimescaleDB (PostgreSQL extension) |
| Object Storage   | MinIO (S3-compatible)              |
| Observability    | Prometheus, Grafana, Loki, Tempo   |
| Orchestration    | Docker, Kubernetes, Helm            |
| CI/CD            | GitHub Actions, ArgoCD             |

---

## Architecture

### Application Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                          Kubernetes Cluster                             │
│                                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌────────────┐  │
│  │   Web UI      │  │  API Gateway │  │  Event       │  │  AI        │  │
│  │  (Next.js)    │  │  (Rust)      │  │  Processor   │  │  Pipeline  │  │
│  │  :3000        │  │  :8080       │  │  (Python)    │  │  (Python)  │  │
│  │               │  │              │  │  :8081       │  │  :8082     │  │
│  └──────┬───────┘  └──────┬───────┘  └──────┬───────┘  └─────┬──────┘  │
│         │                  │                  │                │         │
│         └──────────────────┼──────────────────┼────────────────┘         │
│                            │                  │                          │
│                   ┌────────▼──────────────────▼────────┐                │
│                   │        NATS JetStream              │                │
│                   │        (Message Broker)            │                │
│                   └────────────────────────────────────┘                │
│                                                                         │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐                  │
│  │  TimescaleDB │  │  MinIO       │  │  Redis       │                  │
│  │  (Events)    │  │  (Artifacts) │  │  (Cache)     │                  │
│  │  :5432       │  │  :9000       │  │  :6379       │                  │
│  └──────────────┘  └──────────────┘  └──────────────┘                  │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘
         ▲                                   ▲
         │                                   │
    ┌────┴─────┐                      ┌──────┴───────┐
    │  Edge    │                      │  Prometheus  │
    │  Nodes   │                      │  + Grafana   │
    │  (Rust)  │                      │  :9090/:3001 │
    └──────────┘                      └──────────────┘
```

### Data Flow

```
┌──────────┐    RTSP/ONVIF    ┌──────────┐    NATS     ┌──────────┐
│  Camera  │ ───────────────> │  Edge    │ ──────────> │  Event   │
│  Feed    │                  │  Node    │             │ Processor│
└──────────┘                  └────┬─────┘             └────┬─────┘
                                   │                        │
                              Inference                Correlation
                              (YOLOv8)                 & Enrichment
                                   │                        │
                                   ▼                        ▼
                             ┌──────────┐            ┌──────────┐
                             │  MinIO   │            │TimescaleDB│
                             │  (clips, │            │  (events, │
                             │  frames) │            │  metadata)│
                             └──────────┘            └─────┬────┘
                                                          │
                                                          ▼
                                                     ┌──────────┐
                                                     │  Web UI  │
                                                     │(alerts,  │
                                                     │ timeline)│
                                                     └──────────┘
```

---

## Quick Start

### Prerequisites

- **Docker** >= 24.0 and **Docker Compose** >= 2.20
- **Rust** >= 1.75 (for edge node builds)
- **Python** >= 3.11 (for AI pipeline)
- **Node.js** >= 20 LTS (for web dashboard)
- **kubectl** >= 1.28 (for Kubernetes deployment)
- Minimum 16 GB RAM and 4 CPU cores for local development

### Docker Compose (Recommended)

```bash
git clone https://github.com/your-org/vigilantai.git && cd vigilantai
cp .env.example .env
docker compose up -d
```

The dashboard will be available at `http://localhost:3000`. Default credentials are in `.env.example`.

### Kubernetes

```bash
helm repo add vigilantai https://charts.vigilantai.dev
helm repo update
helm install vigilantai vigilantai/vigilantai --namespace vigilantai --create-namespace
```

Verify the deployment:

```bash
kubectl get pods -n vigilantai
```

---

## Project Structure

```
vigilantai/
├── edge/                    # Rust edge inference node
│   ├── src/
│   │   ├── main.rs
│   │   ├── detector.rs      # YOLOv8 inference wrapper
│   │   ├── stream.rs        # RTSP/ONVIF stream capture
│   │   ├── nats_client.rs   # Message broker integration
│   │   └── config.rs        # Configuration management
│   ├── models/              # ONNX model artifacts
│   ├── Cargo.toml
│   └── Dockerfile
│
├── api/                     # Rust API gateway
│   ├── src/
│   │   ├── main.rs
│   │   ├── routes/          # HTTP route handlers
│   │   ├── middleware/       # Auth, rate limiting, CORS
│   │   ├── models/          # Database models
│   │   └── services/        # Business logic
│   ├── migrations/          # SQL migrations
│   ├── Cargo.toml
│   └── Dockerfile
│
├── pipeline/                # Python AI pipeline & event processor
│   ├── src/
│   │   ├── __init__.py
│   │   ├── event_processor.py
│   │   ├── face_recognition.py
│   │   ├── behavior_analytics.py
│   │   ├── model_trainer.py
│   │   └── config.py
│   ├── models/              # Training scripts and configs
│   ├── tests/
│   ├── pyproject.toml
│   └── Dockerfile
│
├── web/                     # Next.js web dashboard
│   ├── src/
│   │   ├── app/             # App router pages
│   │   ├── components/      # React components
│   │   ├── hooks/           # Custom React hooks
│   │   ├── lib/             # API client, utilities
│   │   └── styles/          # Global styles
│   ├── public/
│   ├── package.json
│   └── Dockerfile
│
├── helm/                    # Kubernetes Helm charts
│   └── vigilantai/
│       ├── Chart.yaml
│       ├── values.yaml
│       └── templates/
│
├── docker/                  # Docker Compose configurations
│   ├── docker-compose.yml
│   └── docker-compose.prod.yml
│
├── monitoring/              # Observability stack configs
│   ├── prometheus/
│   ├── grafana/
│   │   └── dashboards/
│   └── loki/
│
├── docs/                    # Documentation
│   ├── API.md
│   ├── DEPLOYMENT.md
│   ├── SECURITY.md
│   ├── ARCHITECTURE.md
│   └── ROADMAP.md
│
├── scripts/                 # Utility and CI/CD scripts
├── .env.example
├── Makefile
├── justfile
├── rust-toolchain.toml
└── README.md
```

---

## Services

| Service            | Port  | Description                                          | Health Endpoint       |
|--------------------|-------|------------------------------------------------------|-----------------------|
| **Web Dashboard**  | 3000  | Next.js frontend for monitoring and configuration    | `GET /api/health`     |
| **API Gateway**    | 8080  | Rust REST/WebSocket API with JWT authentication      | `GET /health`         |
| **Event Processor**| 8081  | Python service for event correlation and alerting    | `GET /health`         |
| **AI Pipeline**    | 8082  | Python ML inference and behavioral analytics         | `GET /health`         |

### Supporting Infrastructure

| Component          | Port  | Description                        |
|--------------------|-------|------------------------------------|
| TimescaleDB        | 5432  | Time-series event storage          |
| Redis              | 6379  | Session cache and rate limiting    |
| MinIO              | 9000  | Object storage for clips/frames    |
| NATS               | 4222  | Message broker (JetStream)         |
| Prometheus         | 9090  | Metrics collection and alerting    |
| Grafana            | 3001  | Dashboards and visualization       |
| Loki               | 3100  | Log aggregation                    |

---

## Monitoring

VigilantAI ships with a complete observability stack.

### Components

- **Prometheus** — Scrapes metrics from all services at 15s intervals
- **Grafana** — Pre-configured dashboards for system health, AI performance, and security events
- **Loki** — Centralized log aggregation with structured querying
- **Tempo** — Distributed tracing across service boundaries

### Dashboards

| Dashboard               | Description                                 |
|-------------------------|---------------------------------------------|
| System Health           | CPU, memory, disk, and network across nodes |
| AI Pipeline Performance | Inference latency, throughput, model accuracy|
| Event Overview          | Alert volume, response times, false positive rates |
| Camera Health           | Stream status, frame rates, connection uptime |
| Security Audit          | Authentication events, access patterns, RBAC changes |

### Access

```bash
# Grafana (default credentials: admin / vigilantai)
open http://localhost:3001

# Prometheus
open http://localhost:9090

# Logs via Grafana Explore
open http://localhost:3001/explore?left={"datasource":"loki"}
```

---

## Development

### Setup

```bash
# Clone and install dependencies
git clone https://github.com/your-org/vigilantai.git && cd vigilantai

# Rust components
cd edge && cargo build && cd ..
cd api && cargo build && cd ..

# Python pipeline
cd pipeline && python -m venv .venv && .venv\Scripts\activate
pip install -e ".[dev]"

# Web dashboard
cd web && npm install
```

### Makefile Targets

```bash
make build          # Build all services
make test           # Run full test suite
make lint           # Lint all languages
make format         # Format all code
make up             # Start local dev environment
make down           # Stop local dev environment
make logs           # Tail all service logs
make migrate        # Run database migrations
make seed           # Seed demo data
make docker-build   # Build all Docker images
```

### Code Style

| Language | Tool       | Command                  |
|----------|------------|--------------------------|
| Rust     | rustfmt    | `cargo fmt`              |
| Rust     | clippy     | `cargo clippy -- -D warnings` |
| Python   | ruff       | `ruff check .`           |
| Python   | black      | `black .`                |
| TypeScript| ESLint    | `npm run lint`           |
| TypeScript| Prettier  | `npm run format`         |

### Testing

```bash
# Rust
cd edge && cargo test
cd api && cargo test

# Python
cd pipeline && pytest -v

# Web
cd web && npm test

# Integration tests
make test-integration
```

---

## Configuration

### Environment Variables

| Variable                  | Description                          | Default               |
|---------------------------|--------------------------------------|-----------------------|
| `DATABASE_URL`            | TimescaleDB connection string        | `postgres://...`      |
| `REDIS_URL`               | Redis connection string              | `redis://localhost:6379` |
| `NATS_URL`                | NATS server URL                      | `nats://localhost:4222` |
| `MINIO_ENDPOINT`          | MinIO server address                 | `localhost:9000`      |
| `MINIO_ACCESS_KEY`        | MinIO access key                     | `minioadmin`          |
| `MINIO_SECRET_KEY`        | MinIO secret key                     | `minioadmin`          |
| `JWT_SECRET`              | Secret for JWT token signing         | *(required)*          |
| `JWT_EXPIRY_HOURS`        | Token expiry in hours                | `24`                  |
| `LOG_LEVEL`               | Logging level                        | `info`                |
| `API_PORT`                | API gateway listen port              | `8080`                |
| `WEB_PORT`                | Web dashboard listen port            | `3000`                |
| `AI_MODEL_PATH`           | Path to ONNX model files             | `./models`            |
| `INFERENCE_DEVICE`        | Inference device (`cpu` or `cuda`)   | `cpu`                 |
| `ALERT_WEBHOOK_URL`       | External webhook for alerts          | *(optional)*          |
| `SENTRY_DSN`              | Sentry error tracking DSN            | *(optional)*          |

See `.env.example` for the full list with descriptions.

---

## Deployment

### Docker Compose

```bash
# Production deployment
docker compose -f docker/docker-compose.prod.yml up -d --build

# Verify services
docker compose -f docker/docker-compose.prod.yml ps
```

### Kubernetes

```bash
# Install with custom values
helm install vigilantai ./helm/vigilantai \
  --namespace vigilantai \
  --create-namespace \
  --set image.tag=v1.0.0 \
  --set replicas.api=3

# Check rollout status
kubectl rollout status deployment/vigilantai-api -n vigilantai
```

### Horizontal Scaling

```bash
# Scale API gateway
kubectl scale deployment/vigilantai-api --replicas=5 -n vigilantai

# Or via HPA (pre-configured in Helm chart)
kubectl get hpa -n vigilantai
```

The Helm chart includes HPA configurations for the API Gateway and Event Processor, scaling based on CPU utilization and custom metrics (request rate, queue depth).

---

## API

Full API documentation is available at [`docs/API.md`](docs/API.md).

### Authentication

All API requests require a Bearer token:

```bash
curl -H "Authorization: Bearer <token>" http://localhost:8080/api/v1/cameras
```

Tokens are obtained via the `/auth/login` endpoint using email and password. The response includes an access token and refresh token.

### Endpoints Overview

| Method  | Endpoint                        | Description              |
|---------|---------------------------------|--------------------------|
| POST    | `/auth/login`                   | Authenticate and get JWT |
| POST    | `/auth/refresh`                 | Refresh access token     |
| GET     | `/api/v1/cameras`               | List all cameras         |
| POST    | `/api/v1/cameras`               | Register a new camera    |
| GET     | `/api/v1/events`                | Query security events    |
| GET     | `/api/v1/events/:id`            | Get event details        |
| POST    | `/api/v1/alerts/rules`          | Create alert rule        |
| GET     | `/api/v1/dashboard/summary`     | Dashboard summary data   |
| WS      | `/ws/events`                    | Real-time event stream   |

---

## Security

VigilantAI implements defense-in-depth security across all layers.

- **Authentication** — JWT-based with short-lived access tokens and rotating refresh tokens
- **Password Hashing** — Argon2id with per-user salts (memory-hard, GPU-resistant)
- **Authorization** — Role-Based Access Control (RBAC) with configurable permission sets
- **Encryption** — TLS 1.3 in transit, AES-256 at rest for stored artifacts
- **Audit Logging** — All administrative and data access events are logged immutably
- **Input Validation** — Strict request validation on all API endpoints
- **CORS Policy** — Configurable origin allowlist for web dashboard

For detailed security practices, threat model, and vulnerability reporting, see [`docs/SECURITY.md`](docs/SECURITY.md).

---

## Documentation

| Document                       | Description                                  |
|--------------------------------|----------------------------------------------|
| [`docs/API.md`](docs/API.md)                | Full REST & WebSocket API reference          |
| [`docs/DEPLOYMENT.md`](docs/DEPLOYMENT.md)  | Deployment guides for Docker and Kubernetes  |
| [`docs/SECURITY.md`](docs/SECURITY.md)      | Security architecture and practices         |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | System design and component details      |
| [`docs/ROADMAP.md`](docs/ROADMAP.md)        | Product roadmap and version plan             |

---

## Roadmap

| Version | Timeline   | Milestone                                      |
|---------|------------|------------------------------------------------|
| v1.0    | 2026 Q1    | Core platform: detection, tracking, dashboard  |
| v1.1    | 2026 Q2    | Facial recognition GA, LDAP/SSO integration    |
| v1.2    | 2026 Q3    | Multi-tenant support, advanced analytics       |
| v2.0    | 2026 Q4    | Federated learning, edge AI marketplace        |

See [`docs/ROADMAP.md`](docs/ROADMAP.md) for the full roadmap with detailed deliverables and timelines.

---

## Contributing

We welcome contributions from the community.

1. **Fork** the repository and create a feature branch from `main`
2. **Make your changes** following the code style guidelines above
3. **Write or update tests** for any new functionality
4. **Run the full test suite** and ensure all checks pass:
   ```bash
   make test && make lint
   ```
5. **Submit a pull request** with a clear description of the change and link any related issues

### Pull Request Guidelines

- Keep PRs focused on a single change
- Include screenshots or recordings for UI changes
- Update documentation if adding or modifying public APIs
- Follow conventional commit messages (`feat:`, `fix:`, `docs:`, `refactor:`)
- All PRs require at least one review before merging

---

## License

This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## Support

- **Issues** — [GitHub Issues](https://github.com/your-org/vigilantai/issues)
- **Discussions** — [GitHub Discussions](https://github.com/your-org/vigilantai/discussions)
- **Security Vulnerabilities** — Report privately via [SECURITY.md](docs/SECURITY.md)

---

<p align="center">
  <sub>Built with care for security teams who need to see everything.</sub>
</p>
