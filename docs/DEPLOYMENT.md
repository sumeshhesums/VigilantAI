# VigilantAI — Deployment Guide

> **Enterprise Security Intelligence Platform**
> Deployment Guide — v1.0

---

## Table of Contents

| Section | Title                                |
|---------|--------------------------------------|
| 1       | Prerequisites                        |
| 2       | Quick Start (Docker Compose)         |
| 3       | Docker Compose Deployment            |
| 4       | Kubernetes Deployment                |
| 5       | Environment Configuration            |
| 6       | Database Migration                   |
| 7       | JWT Key Generation                   |
| 8       | SSL/TLS Setup                        |
| 9       | Makefile Targets                     |
| 10      | Scaling Considerations               |
| 11      | Backup and Recovery                  |
| 12      | Upgrade Procedures                   |

---

## 1. Prerequisites

### 1.1 Docker Compose Deployment

| Tool               | Minimum Version | Purpose                     |
|--------------------|-----------------|-----------------------------|
| Docker             | 24.0+           | Container runtime           |
| Docker Compose     | v2.20+          | Multi-container orchestration|
| Git                | 2.40+           | Source code retrieval       |
| Make (GNU)         | 4.0+            | Build automation            |
| OpenSSL            | 3.0+            | JWT key generation          |

### 1.2 Kubernetes Deployment

| Tool               | Minimum Version | Purpose                     |
|--------------------|-----------------|-----------------------------|
| kubectl            | 1.28+           | Kubernetes CLI              |
| Helm               | 3.14+           | Package management          |
| Docker             | 24.0+           | Container image building    |
| A Kubernetes Cluster| 1.28+          | Container orchestration     |

### 1.3 Hardware Requirements

| Component          | Minimum                                        | Recommended                                  |
|--------------------|------------------------------------------------|----------------------------------------------|
| Application Node   | 8 vCPU, 16 GB RAM, 100 GB SSD                  | 16 vCPU, 32 GB RAM, 200 GB NVMe SSD         |
| GPU Node (AI)      | 8 vCPU, 32 GB RAM, 1x NVIDIA T4 (16 GB VRAM)  | 16 vCPU, 64 GB RAM, 4x NVIDIA A10 (24 GB)   |
| Database Node      | 8 vCPU, 32 GB RAM, 500 GB NVMe SSD             | 16 vCPU, 64 GB RAM, 2 TB NVMe SSD           |
| Monitoring Node    | 4 vCPU, 8 GB RAM, 500 GB SSD                   | 8 vCPU, 16 GB RAM, 1 TB NVMe SSD            |

---

## 2. Quick Start (Docker Compose)

The fastest way to get VigilantAI running locally:

```bash
# 1. Clone the repository
git clone https://github.com/vigilantai/vigilantai.git
cd vigilantai

# 2. Copy and configure environment variables
cp .env.example .env
# Edit .env — at minimum, set JWT_PRIVATE_KEY and JWT_PUBLIC_KEY

# 3. Generate JWT keys (if you don't have them)
openssl genrsa 4096 2>/dev/null | openssl pkcs8 -topk8 -nocrypt -outform PEM > jwt_private.pem
openssl rsa -in jwt_private.pem -pubout -out jwt_public.pem
# Paste contents into .env as JWT_PRIVATE_KEY and JWT_PUBLIC_KEY (multiline)

# 4. Start all services
make docker-up

# 5. Verify all services are healthy
make docker-ps
```

**Service URLs after startup:**

| Service       | URL                               |
|---------------|-----------------------------------|
| Dashboard     | http://localhost:3000              |
| Backend API   | http://localhost:8080              |
| AI Service    | http://localhost:8081              |
| Camera Gateway| http://localhost:8082              |
| Prometheus    | http://localhost:9090              |
| Grafana       | http://localhost:3001 (admin/admin)|
| Loki          | http://localhost:3100              |

---

## 3. Docker Compose Deployment

### 3.1 Services

The `docker-compose.yml` defines 10 services:

| Service           | Image                          | Port   | Purpose                      |
|-------------------|--------------------------------|--------|------------------------------|
| `postgres`        | `postgres:16-alpine`          | 5432   | Primary database             |
| `redis`           | `redis:7-alpine`              | 6379   | Cache, session store         |
| `backend`         | Custom build (Rust)            | 8080   | API server                   |
| `ai-service`      | Custom build (Python)          | 8081   | AI inference                 |
| `camera-gateway`  | Custom build (Rust)            | 8082   | Camera stream ingestion      |
| `dashboard`       | Custom build (Next.js)         | 3000   | Web dashboard                |
| `prometheus`      | `prom/prometheus:v2.54.1`      | 9090   | Metrics collection           |
| `loki`            | `grafana/loki:3.2.1`           | 3100   | Log aggregation              |
| `promtail`        | `grafana/promtail:3.2.1`       | 9080   | Log collection               |
| `grafana`         | `grafana/grafana:11.2.2`       | 3001   | Dashboard visualization      |

### 3.2 Volumes

| Volume            | Purpose                          |
|-------------------|----------------------------------|
| `pgdata`          | PostgreSQL data persistence      |
| `redisdata`       | Redis data persistence           |
| `evidence_data`   | Evidence file storage            |
| `prometheus_data` | Prometheus time-series data      |
| `grafana_data`    | Grafana dashboards and config    |
| `loki_data`       | Loki log storage                 |

### 3.3 Network

All services communicate on the `vigilant-net` bridge network (172.20.0.0/16).

### 3.4 Step-by-Step Docker Compose Deployment

```bash
# Step 1: Generate JWT keys
openssl genrsa 4096 2>/dev/null | openssl pkcs8 -topk8 -nocrypt -outform PEM > jwt_private.pem
openssl rsa -in jwt_private.pem -pubout -out jwt_public.pem

# Step 2: Configure environment
cp .env.example .env
# Edit .env and paste JWT key contents

# Step 3: Build all images
make docker-build

# Step 4: Start infrastructure first (PostgreSQL + Redis)
make infra-up

# Step 5: Wait for infrastructure health
make docker-ps

# Step 6: Start all services
make docker-up

# Step 7: Verify health
make docker-ps
curl http://localhost:8080/api/v1/health
curl http://localhost:8081/health
curl http://localhost:8082/health
```

### 3.5 Viewing Logs

```bash
# All services
make docker-logs

# Specific service
docker compose logs -f backend
docker compose logs -f ai-service
docker compose logs -f camera-gateway
```

### 3.6 Stopping Services

```bash
# Stop all services
make docker-down

# Stop and remove all data (nuclear option)
make docker-clean
```

---

## 4. Kubernetes Deployment

### 4.1 Namespace and Configuration

```bash
# Create the namespace
kubectl apply -f k8s/namespace.yaml

# Apply configuration
kubectl apply -f k8s/configmap.yaml

# Create secrets (replace with actual values)
kubectl apply -f k8s/secret.example.yaml
```

### 4.2 Data Stores

```bash
# PostgreSQL
kubectl apply -f k8s/postgres/

# Redis
kubectl apply -f k8s/redis/

# Persistent storage
kubectl apply -f k8s/persistent-volume.yaml
kubectl apply -f k8s/persistent-volume-claim.yaml
```

### 4.3 Application Services

```bash
# Backend
kubectl apply -f k8s/backend/

# AI Service
kubectl apply -f k8s/ai-service/

# Camera Gateway
kubectl apply -f k8s/camera-gateway/

# Dashboard
kubectl apply -f k8s/dashboard/
```

### 4.4 Monitoring Stack

```bash
# Prometheus
kubectl apply -f k8s/prometheus/

# Grafana
kubectl apply -f k8s/grafana/

# Loki
kubectl apply -f k8s/loki/

# Promtail (log collection)
kubectl apply -f k8s/promtail/
```

### 4.5 Ingress

```bash
kubectl apply -f k8s/ingress.yaml
```

### 4.6 Horizontal Pod Autoscalers

```bash
kubectl apply -f k8s/hpa/backend-hpa.yaml
kubectl apply -f k8s/hpa/ai-service-hpa.yaml
kubectl apply -f k8s/hpa/camera-gateway-hpa.yaml
```

**HPA Configuration:**

| Service          | Min Replicas | Max Replicas | CPU Target |
|------------------|-------------|-------------|------------|
| backend          | 2           | 10          | 70%        |
| ai-service       | 1           | 5           | 70%        |
| camera-gateway   | 1           | 5           | 70%        |

### 4.7 Verify Kubernetes Deployment

```bash
# Check all pods are running
kubectl get pods -n vigilantai

# Check services
kubectl get svc -n vigilantai

# Check HPAs
kubectl get hpa -n vigilantai

# Check ingress
kubectl get ingress -n vigilantai

# View logs
kubectl logs -f deployment/backend -n vigilantai
kubectl logs -f deployment/ai-service -n vigilantai
```

---

## 5. Environment Configuration

### 5.1 Environment Variables

Copy `.env.example` to `.env` and configure:

```bash
cp .env.example .env
```

### 5.2 PostgreSQL

| Variable           | Default                          | Description                    |
|--------------------|----------------------------------|--------------------------------|
| `POSTGRES_USER`    | `vigilant`                       | Database username              |
| `POSTGRES_PASSWORD`| `changeme`                       | Database password              |
| `POSTGRES_DB`      | `vigilantai`                     | Database name                  |
| `POSTGRES_PORT`    | `5432`                           | Database port                  |
| `DATABASE_URL`     | `postgres://vigilant:changeme@postgres:5432/vigilantai` | Full connection URL |

### 5.3 Redis

| Variable           | Default                          | Description                    |
|--------------------|----------------------------------|--------------------------------|
| `REDIS_PORT`       | `6379`                           | Redis port                     |
| `REDIS_URL`        | `redis://redis:6379`             | Redis connection URL           |

### 5.4 Backend

| Variable                        | Default     | Description                          |
|---------------------------------|-------------|--------------------------------------|
| `BACKEND_HOST`                  | `0.0.0.0`   | Bind address                         |
| `BACKEND_PORT`                  | `8080`      | Listen port                          |
| `RUST_LOG`                      | `backend=info,tower_http=info` | Log level configuration |
| `JWT_PRIVATE_KEY`               | (required)  | RSA private key for JWT signing      |
| `JWT_PUBLIC_KEY`                | (required)  | RSA public key for JWT verification  |
| `JWT_ACCESS_TOKEN_EXPIRY_SECS`  | `900`       | Access token lifetime (15 min)       |
| `JWT_REFRESH_TOKEN_EXPIRY_SECS` | `604800`    | Refresh token lifetime (7 days)      |
| `EVIDENCE_STORAGE_PATH`         | `/data/evidence` | Evidence file storage path      |
| `EVIDENCE_MAX_FILE_SIZE`        | `20971520`  | Max upload size (20 MB)              |
| `NOTIFICATION_ENABLED`          | `true`      | Enable notifications                 |
| `NOTIFICATION_WEBHOOK_URL`      | (empty)     | Webhook notification URL             |
| `NOTIFICATION_WEBHOOK_TIMEOUT_SECS` | `10`    | Webhook timeout                      |
| `NOTIFICATION_EMAIL_ENABLED`    | `false`     | Enable email notifications           |
| `NOTIFICATION_MAX_RETRIES`      | `3`         | Max notification retry attempts      |

### 5.5 AI Service

| Variable                          | Default     | Description                      |
|-----------------------------------|-------------|----------------------------------|
| `AI_SERVICE_HOST`                 | `0.0.0.0`   | Bind address                     |
| `AI_SERVICE_PORT`                 | `8081`      | Listen port                      |
| `AI_SERVICE_LOG_LEVEL`            | `INFO`      | Python log level                 |
| `AI_SERVICE_DEFAULT_MODEL`        | `yolov8n`   | YOLO model variant               |
| `AI_SERVICE_DEVICE`               | `cpu`       | Compute device (`cpu` or `cuda:0`)|
| `AI_SERVICE_AUTO_LOAD`            | `true`      | Load model on startup            |
| `AI_SERVICE_CONFIDENCE_THRESHOLD` | `0.5`       | Min detection confidence         |
| `AI_SERVICE_IOU_THRESHOLD`        | `0.45`      | IoU threshold for NMS            |

### 5.6 Camera Gateway

| Variable                    | Default                           | Description               |
|-----------------------------|-----------------------------------|---------------------------|
| `GATEWAY_PORT`              | `8082`                            | Listen port               |
| `GATEWAY_AI_SERVICE_URL`    | `http://ai-service:8081`          | AI service endpoint       |
| `GATEWAY_BACKEND_URL`       | `http://backend:8080`             | Backend API endpoint      |
| `GATEWAY_AUTH_TOKEN`        | (empty)                           | Internal auth token       |

### 5.7 Dashboard

| Variable                | Default                                      | Description          |
|-------------------------|----------------------------------------------|----------------------|
| `NEXT_PUBLIC_API_URL`   | `http://localhost:8080/api/v1`               | Backend API base URL |

### 5.8 Grafana

| Variable                          | Default     | Description                      |
|-----------------------------------|-------------|----------------------------------|
| `GF_SECURITY_ADMIN_USER`         | `admin`     | Grafana admin username           |
| `GF_SECURITY_ADMIN_PASSWORD`     | `admin`     | Grafana admin password           |
| `GF_USERS_ALLOW_SIGN_UP`         | `false`     | Disable public sign-up           |
| `GF_SERVER_ROOT_URL`             | `http://localhost:3001` | Grafana base URL          |
| `GRAFANA_PORT`                   | `3001`      | External Grafana port            |

---

## 6. Database Migration

The backend uses SQLx with compile-time checked queries and built-in migration support.

```bash
# Run migrations (via backend binary)
cargo run --bin backend -- migrate

# Or via SQLx CLI (if installed)
sqlx migrate run --source migrations/

# Check migration status
sqlx migrate info --source migrations/
```

**Migration files** are located in `backend/migrations/` and run automatically on backend startup when configured.

---

## 7. JWT Key Generation

Generate an RSA 4096-bit key pair for JWT signing:

```bash
# Generate private key
openssl genrsa 4096 2>/dev/null | openssl pkcs8 -topk8 -nocrypt -outform PEM > jwt_private.pem

# Extract public key
openssl rsa -in jwt_private.pem -pubout -out jwt_public.pem

# Copy to .env (multiline)
echo "JWT_PRIVATE_KEY=" >> .env
echo "-----BEGIN PRIVATE KEY-----" >> .env
base64 -w 0 jwt_private.pem | sed 's/.\{64\}/&\n/g' >> .env
echo "-----END PRIVATE KEY-----" >> .env

echo "JWT_PUBLIC_KEY=" >> .env
echo "-----BEGIN PUBLIC KEY-----" >> .env
base64 -w 0 jwt_public.pem | sed 's/.\{64\}/&\n/g' >> .env
echo "-----END PUBLIC KEY-----" >> .env
```

**Key rotation:** Generate new key pairs periodically. Deploy the new public key alongside the old one during a transition period, then remove the old key.

---

## 8. SSL/TLS Setup

### 8.1 Docker Compose (Development)

For local development, use self-signed certificates:

```bash
# Generate self-signed certificate
openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
  -keyout nginx-selfsigned.key \
  -out nginx-selfsigned.crt \
  -subj "/CN=localhost"

# Create nginx config for TLS termination
# Place in front of the backend and dashboard services
```

### 8.2 Production (Let's Encrypt)

```bash
# Install certbot
sudo apt install certbot

# Obtain certificate
sudo certbot certonly --standalone -d app.vigilantai.com -d api.vigilantai.com

# Certificates stored at
# /etc/letsencrypt/live/app.vigilantai.com/fullchain.pem
# /etc/letsencrypt/live/app.vigilantai.com/privkey.pem

# Auto-renewal
sudo certbot renew --dry-run
```

### 8.3 Kubernetes (cert-manager)

```bash
# Install cert-manager
kubectl apply -f https://github.com/cert-manager/cert-manager/releases/latest/download/cert-manager.yaml

# Create ClusterIssuer
cat <<EOF | kubectl apply -f -
apiVersion: cert-manager.io/v1
kind: ClusterIssuer
metadata:
  name: letsencrypt-prod
spec:
  acme:
    server: https://acme-v02.api.letsencrypt.org/directory
    email: admin@vigilantai.com
    privateKeySecretRef:
      name: letsencrypt-prod
    solvers:
    - http01:
        ingress:
          class: nginx
EOF
```

---

## 9. Makefile Targets

| Target           | Description                                      | Command                              |
|------------------|--------------------------------------------------|--------------------------------------|
| `make help`      | Show all available targets                       | `make help`                          |
| `make dev`       | Start infrastructure and build services          | `make dev`                           |
| `make build`     | Build all Rust services                          | `cargo build --release -p backend && cargo build --release -p camera-gateway` |
| `make test`      | Run all Rust tests                               | `cargo test -p backend && cargo test -p camera-gateway` |
| `make lint`      | Run clippy linter on all workspace members       | `cargo clippy --workspace --all-targets -- -D warnings` |
| `make fmt`       | Check formatting across the workspace            | `cargo fmt --all -- --check`        |
| `make fmt-fix`   | Auto-fix formatting across the workspace         | `cargo fmt --all`                    |
| `make check`     | Run format check, lint, and tests                | `make fmt && make lint && make test` |
| `make infra-up`  | Start PostgreSQL and Redis via Docker Compose    | `docker compose up -d postgres redis`|
| `make infra-down`| Stop infrastructure services                     | `docker compose down`                |
| `make infra-logs`| Tail infrastructure logs                         | `docker compose logs -f postgres redis` |
| `make docker-build` | Build all Docker images                       | `docker compose build`               |
| `make docker-up` | Start all services                               | `docker compose up -d`               |
| `make docker-down` | Stop all services                              | `docker compose down`                |
| `make docker-logs` | Tail all service logs                          | `docker compose logs -f`             |
| `make docker-restart` | Restart all services                        | `docker compose restart`             |
| `make docker-ps` | Show running services status                     | `docker compose ps`                  |
| `make docker-clean` | Stop all services and remove volumes          | `docker compose down -v && cargo clean` |

---

## 10. Scaling Considerations

### 10.1 Horizontal Scaling

| Service          | Scaling Strategy                | Trigger               | Min | Max |
|------------------|--------------------------------|-----------------------|-----|-----|
| Backend          | HPA (CPU-based)                | CPU > 70%             | 2   | 10  |
| AI Service       | HPA (CPU-based)                | CPU > 70%             | 1   | 5   |
| Camera Gateway   | HPA (CPU-based)                | CPU > 70%             | 1   | 5   |
| Dashboard        | Static export + CDN            | Traffic               | 1   | N/A |
| PostgreSQL       | Read replicas                  | Read load             | 1   | 3   |
| Redis            | Cluster mode                   | Memory / connections  | 1   | 6   |

### 10.2 Vertical Scaling

| Service          | Resource Increase              | When                  |
|------------------|--------------------------------|-----------------------|
| AI Service       | GPU upgrade (T4 → A10 → A100) | More cameras / higher FPS |
| PostgreSQL       | RAM + CPU increase             | Query latency         |
| Redis            | Memory increase                | Eviction rate         |

### 10.3 Camera Capacity Planning

| Cameras | Backend | AI Service    | GPU         | PostgreSQL     |
|---------|---------|---------------|-------------|----------------|
| 50      | 1x      | 1x (CPU)      | None        | 1x (4 CPU, 16 GB) |
| 200     | 2x      | 2x            | 1x T4       | 1x (8 CPU, 32 GB) |
| 500     | 3x      | 3x            | 2x A10      | 1x (16 CPU, 64 GB) + 1 replica |
| 1000+   | 5x+     | 5x            | 4x A10      | 1x (16 CPU, 64 GB) + 2 replicas |

---

## 11. Backup and Recovery

### 11.1 PostgreSQL Backup

```bash
# Full backup
docker compose exec postgres pg_dump -U vigilant vigilantai > backup_$(date +%Y%m%d_%H%M%S).sql

# Restore
cat backup_20260723_120000.sql | docker compose exec -T postgres psql -U vigilant vigilantai
```

### 11.2 Evidence Storage Backup

```bash
# Backup evidence files
docker compose exec backend tar czf /tmp/evidence_backup.tar.gz -C /data evidence/
docker compose cp backend:/tmp/evidence_backup.tar.gz ./backups/
```

### 11.3 Automated Backup (Cron)

```bash
# Add to crontab for daily backups at 2 AM
0 2 * * * docker compose exec -T postgres pg_dump -U vigilant vigilantai | gzip > /backups/pg_$(date +\%Y\%m\%d).sql.gz
0 3 * * * docker compose exec backend tar czf /backups/evidence_$(date +\%Y\%m\%d).tar.gz -C /data evidence/
```

### 11.4 Recovery Procedures

| Scenario                  | Recovery Action                                    | RTO    |
|---------------------------|----------------------------------------------------|--------|
| PostgreSQL data loss      | Restore from pg_dump backup                        | < 30 min |
| Evidence data loss        | Restore from filesystem backup                     | < 1 hr  |
| Full system failure       | Rebuild from images + restore data backups         | < 2 hr  |
| Redis data loss           | Redis is cache-only; restart clears and repopulates| < 5 min  |

---

## 12. Upgrade Procedures

### 12.1 Docker Compose Upgrade

```bash
# 1. Pull latest images
docker compose pull

# 2. Rebuild custom images
make docker-build

# 3. Stop services (preserves volumes)
make docker-down

# 4. Start with new images
make docker-up

# 5. Verify health
make docker-ps
```

### 12.2 Kubernetes Rolling Update

```bash
# Apply updated manifests
kubectl apply -f k8s/

# Watch rollout status
kubectl rollout status deployment/backend -n vigilantai
kubectl rollout status deployment/ai-service -n vigilantai
kubectl rollout status deployment/camera-gateway -n vigilantai
```

### 12.3 Rollback

```bash
# Kubernetes rollback
kubectl rollout undo deployment/backend -n vigilantai

# Docker Compose rollback (use previous image tag)
# Edit docker-compose.yml to specify previous tag
make docker-down && make docker-up
```
