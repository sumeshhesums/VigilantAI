# VigilantAI — Architecture Reference

> **Enterprise Security Intelligence Platform**
> Architecture Reference Guide — v1.0

---

## Table of Contents

| Section | Title                                |
|---------|--------------------------------------|
| 1       | System Overview                      |
| 2       | Technology Stack                     |
| 3       | Core Services                        |
| 4       | Data Stores                          |
| 5       | Monitoring Stack                     |
| 6       | Service Communication                |
| 7       | Application Architecture Diagram     |
| 8       | Data Flow                            |
| 9       | Authentication Flow                  |
| 10      | Design Decisions and Trade-offs      |

---

## 1. System Overview

VigilantAI is an AI-powered security surveillance platform that processes real-time camera feeds, applies YOLO-based object detection, and delivers actionable security intelligence through a modern web dashboard. The platform is composed of four core services, three data stores, and a full observability stack.

```
┌─────────────────────────────────────────────────────────────────────┐
│                     VigilantAI Platform                             │
│                                                                     │
│  ┌──────────┐  ┌──────────────┐  ┌───────────┐  ┌──────────────┐  │
│  │ Dashboard │  │   Backend    │  │   AI      │  │   Camera     │  │
│  │ (Next.js) │  │ (Rust/Axum)  │  │  Service  │  │   Gateway    │  │
│  │   :3000   │  │    :8080     │  │ (Python)  │  │  (Rust/Axum) │  │
│  │           │  │              │  │   :8081   │  │    :8082     │  │
│  └─────┬─────┘  └──────┬───────┘  └─────┬─────┘  └──────┬───────┘  │
│        │               │                │                │          │
│        │         ┌─────┴─────┐    ┌─────┴─────┐   ┌─────┴─────┐   │
│        │         │ PostgreSQL │    │           │   │  IP Cams  │   │
│        │         │    Redis   │    │  YOLO v8  │   │  (RTSP)   │   │
│        │         │  Evidence  │    │  OpenCV   │   │           │   │
│        │         │    FS      │    │           │   │           │   │
│        │         └───────────┘    └───────────┘   └───────────┘   │
│        │                                                           │
│  ┌─────┴──────────────────────────────────────────────────────┐    │
│  │              Monitoring Stack                               │    │
│  │   Prometheus │ Grafana │ Loki │ Promtail │ Alertmanager    │    │
│  └────────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 2. Technology Stack

| Layer            | Component          | Technology              | Version      |
|------------------|--------------------|-------------------------|--------------|
| **Backend**      | API Server         | Rust                    | 1.78+        |
|                  | Web Framework      | Axum                    | 0.7          |
|                  | Async Runtime      | Tokio                   | 1.x          |
|                  | Database Driver    | SQLx                    | 0.8          |
|                  | Serialization      | Serde / serde_json      | 1.x          |
|                  | HTTP Middleware     | tower-http              | 0.5          |
|                  | JWT                | jsonwebtoken            | 9.x          |
|                  | Password Hashing   | argon2                  | 0.5          |
|                  | Redis Client       | redis                   | 0.25         |
|                  | Metrics            | prometheus              | 0.13         |
|                  | Logging            | tracing / tracing-subscriber | 0.1 / 0.3 |
| **AI Service**   | Language           | Python                  | 3.11+        |
|                  | Web Framework      | FastAPI                 | latest       |
|                  | Detection Model    | YOLO (Ultralytics)      | v8           |
|                  | Computer Vision    | OpenCV                  | latest       |
| **Camera Gateway**| Language           | Rust                    | 1.78+        |
|                  | Web Framework      | Axum                    | 0.7          |
|                  | Stream Protocol    | RTSP                    | -            |
| **Dashboard**    | Framework          | Next.js                 | 15           |
|                  | UI Library         | React                   | 19           |
|                  | Language           | TypeScript              | 5.x          |
|                  | Styling            | Tailwind CSS            | 4.x          |
|                  | Runtime            | Node.js                 | 22+          |
| **Data Stores**  | Primary Database   | PostgreSQL              | 16           |
|                  | Cache / Queue      | Redis                   | 7            |
|                  | Evidence Storage   | Filesystem              | -            |
| **Monitoring**   | Metrics            | Prometheus              | v2.54.1      |
|                  | Dashboards         | Grafana                 | 11.2.2       |
|                  | Log Aggregation    | Loki                    | 3.2.1        |
|                  | Log Collection     | Promtail                | 3.2.1        |
|                  | Alerting           | Alertmanager            | v0.27        |
| **Deployment**   | Containers         | Docker                  | latest       |
|                  | Orchestration      | Kubernetes              | 1.28+        |
|                  | Build System       | Cargo (Rust) + npm      | -            |
|                  | Make Targets       | Make                    | GNU Make     |

---

## 3. Core Services

### 3.1 Backend API Service (Rust/Axum)

| Property              | Value                                                |
|-----------------------|------------------------------------------------------|
| Language              | Rust 2021 edition                                    |
| Framework             | Axum 0.7 with Tower middleware                       |
| Runtime               | Tokio (work-stealing scheduler)                      |
| Port                  | 8080                                                 |
| Health Endpoint       | `GET /api/v1/health`                                 |
| Metrics Endpoint      | `GET /metrics` (Prometheus format)                   |
| Database              | PostgreSQL 16 via SQLx 0.8                           |
| Auth                  | JWT RS256 with access/refresh tokens                 |
| Authorization         | RBAC with 6 roles and 23 permissions                 |
| Password Hashing      | Argon2id                                             |
| Evidence Storage      | Filesystem at `EVIDENCE_STORAGE_PATH`                |
| Connection Pool       | Min 5, Max 20 (configurable)                         |
| Graceful Shutdown     | 30-second drain period                               |

**Responsibilities:**
- REST API for all platform operations (auth, users, cameras, incidents, evidence, notifications)
- JWT authentication and RBAC authorization
- WebSocket connections for real-time dashboard updates
- Audit logging for all state-changing operations
- Evidence upload, storage, and integrity verification (SHA-256)
- Prometheus metrics exposition

### 3.2 AI Service (Python/FastAPI)

| Property              | Value                                                |
|-----------------------|------------------------------------------------------|
| Language              | Python 3.11+                                         |
| Framework             | FastAPI                                              |
| Port                  | 8081 (internal)                                      |
| Health Endpoint       | `GET /health`                                        |
| Default Model         | YOLOv8n (`AI_SERVICE_DEFAULT_MODEL`)                 |
| Device                | CPU or CUDA (`AI_SERVICE_DEVICE`)                    |
| Confidence Threshold  | 0.5 (`AI_SERVICE_CONFIDENCE_THRESHOLD`)              |
| IoU Threshold         | 0.45 (`AI_SERVICE_IOU_THRESHOLD`)                    |

**Responsibilities:**
- Load and manage YOLO model weights
- Receive frames from camera-gateway
- Run object detection inference
- Return detection results (class, bounding box, confidence, tracking ID)
- GPU acceleration support (CUDA) with CPU fallback

### 3.3 Camera Gateway (Rust/Axum)

| Property              | Value                                                |
|-----------------------|------------------------------------------------------|
| Language              | Rust 2021 edition                                    |
| Framework             | Axum 0.7                                              |
| Port                  | 8082                                                 |
| Health Endpoint       | `GET /health`                                        |
| AI Service URL        | `http://ai-service:8081`                             |
| Backend URL           | `http://backend:8080`                                |
| Stream Protocol       | RTSP (port 554 default)                              |

**Responsibilities:**
- Establish and maintain RTSP connections to IP cameras
- Extract and normalize frames from camera streams
- Forward frames to AI service for inference
- Publish detection results to the backend
- Camera stream reconnection with exponential backoff
- Track online/offline camera status

### 3.4 Dashboard (Next.js 15)

| Property              | Value                                                |
|-----------------------|------------------------------------------------------|
| Framework             | Next.js 15 with App Router                           |
| UI                    | React 19 + Tailwind CSS 4                            |
| Language              | TypeScript 5                                         |
| Port                  | 3000                                                 |
| API URL               | `NEXT_PUBLIC_API_URL`                                |
| Build Output          | Static export (`next build`)                         |
| Container             | nginx:alpine                                         |

**Responsibilities:**
- Security operations dashboard (live camera feeds, alert console)
- Incident management interface
- Camera fleet management
- User and role administration
- Real-time WebSocket updates for alerts and incidents
- Evidence browsing and download

---

## 4. Data Stores

### 4.1 PostgreSQL 16

| Property              | Value                                                |
|-----------------------|------------------------------------------------------|
| Version               | 16 (Alpine)                                          |
| Port                  | 5432                                                 |
| Auth                  | SCRAM-SHA-256                                        |
| Connection URL        | `DATABASE_URL` env var                               |
| Volumes               | `pgdata:/var/lib/postgresql/data`                    |
| Health Check          | `pg_isready` (10s interval)                          |

**Stores:** Users, roles, permissions, cameras, camera groups, sites, incidents, evidence metadata, notifications, audit logs, detection events, rules, refresh tokens.

### 4.2 Redis 7

| Property              | Value                                                |
|-----------------------|------------------------------------------------------|
| Version               | 7 (Alpine)                                           |
| Port                  | 6379                                                 |
| Persistence           | AOF (append-only file)                               |
| Eviction              | allkeys-lru                                          |
| Max Memory            | 2 GB (configurable)                                  |
| Volumes               | `redisdata:/data`                                    |
| Health Check          | `redis-cli ping` (10s interval)                      |

**Stores:** JWT blocklist, session context, rate limiting counters, permission cache.

### 4.3 Filesystem (Evidence Storage)

| Property              | Value                                                |
|-----------------------|------------------------------------------------------|
| Path                  | `/data/evidence` (configurable via `EVIDENCE_STORAGE_PATH`) |
| Directory Structure   | `{site_id}/{YYYY-MM-DD}/{uuid}.{ext}`                |
| Max File Size         | 20 MB (`EVIDENCE_MAX_FILE_SIZE`)                     |
| Allowed Formats       | JPEG, PNG, MP4                                       |
| Integrity             | SHA-256 content hash computed at creation            |
| Retention             | Configurable (default: 90 days)                      |

---

## 5. Monitoring Stack

| Component     | Version     | Port  | Purpose                                      |
|---------------|-------------|-------|----------------------------------------------|
| Prometheus    | v2.54.1     | 9090  | Time-series metrics collection (30-day retention) |
| Grafana       | 11.2.2      | 3001  | Dashboard visualization and alerting UI      |
| Loki          | 3.2.1       | 3100  | Log aggregation and querying                 |
| Promtail      | 3.2.1       | 9080  | Log collection from containers and host      |
| Alertmanager  | v0.27       | 9093  | Alert routing and notification               |

**Prometheus Scrape Targets:**

| Job Name                       | Target                      | Metrics Path |
|--------------------------------|-----------------------------|-------------|
| `vigilantai-backend`          | `backend:8080`              | `/metrics`  |
| `vigilantai-camera-gateway`   | `camera-gateway:8082`       | `/metrics`  |
| `vigilantai-ai-service`       | `ai-service:8081`           | `/metrics`  |
| `vigilantai-dashboard`        | `dashboard:3000`            | `/api/metrics` |
| `vigilantai-postgres`         | `postgres-exporter:9187`    | `/metrics`  |
| `vigilantai-redis`            | `redis-exporter:9121`       | `/metrics`  |

---

## 6. Service Communication

```
                    External Traffic
                          │
                          ▼
                   ┌──────────────┐
                   │   Dashboard   │
                   │  (Next.js)   │
                   └──────┬───────┘
                          │ HTTP (REST + WebSocket)
                          ▼
                   ┌──────────────┐
                   │   Backend    │◄──── JWT Auth + RBAC
                   │ (Rust/Axum) │
                   └──┬───┬───┬──┘
                      │   │   │
          ┌───────────┘   │   └───────────┐
          │               │               │
          ▼               ▼               ▼
   ┌────────────┐  ┌────────────┐  ┌────────────┐
   │ PostgreSQL │  │   Redis    │  │  Evidence  │
   │  (SQLx)   │  │  (cache)   │  │  Storage   │
   └────────────┘  └────────────┘  └────────────┘

   Backend ◄──── HTTP (internal API) ──── Camera Gateway
                                              │
                                              ▼
                                        ┌────────────┐
                                        │ AI Service │
                                        │ (YOLO)     │
                                        └────────────┘
                                              ▲
                                              │ RTSP
                                        ┌────────────┐
                                        │ IP Cameras │
                                        └────────────┘
```

### 6.1 Communication Protocols

| Path                          | Protocol  | Auth Method           | Direction     |
|-------------------------------|-----------|----------------------|---------------|
| Client → Dashboard            | HTTPS     | TLS 1.3              | Inbound       |
| Dashboard → Backend           | REST/WSS  | JWT Bearer token     | Outbound      |
| Backend → PostgreSQL          | PostgreSQL| SCRAM-SHA-256        | Outbound      |
| Backend → Redis               | Redis     | AUTH command         | Outbound      |
| Backend ↔ Evidence Storage    | File I/O  | Filesystem perms     | Bidirectional |
| Camera Gateway → AI Service   | HTTP      | Internal API key     | Outbound      |
| Camera Gateway → Backend      | HTTP      | Internal API key     | Outbound      |
| Camera Gateway → IP Cameras   | RTSP      | Credential-based     | Outbound      |
| Prometheus → All Services     | HTTP      | Network isolation    | Scraping      |
| Loki ← Promtail              | HTTP      | Network isolation    | Log shipping  |

---

## 7. Application Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           PRESENTATION LAYER                                │
│                                                                             │
│  ┌───────────────────────────────────────────────────────────────────────┐  │
│  │                    Next.js 15 Dashboard (:3000)                       │  │
│  │                                                                       │  │
│  │  ┌──────────┐ ┌───────────┐ ┌──────────┐ ┌───────────┐ ┌──────────┐ │  │
│  │  │  Live    │ │  Alert    │ │ Incident │ │  Camera   │ │  Admin   │ │  │
│  │  │ Monitor  │ │ Console   │ │ Manager  │ │  Fleet    │ │  Panel   │ │  │
│  │  └──────────┘ └───────────┘ └──────────┘ └───────────┘ └──────────┘ │  │
│  └──────────────────────────────┬────────────────────────────────────────┘  │
│                                 │ REST + WebSocket                          │
├─────────────────────────────────┼───────────────────────────────────────────┤
│                           API GATEWAY LAYER                                 │
│                                 │                                           │
│  ┌──────────────────────────────┼────────────────────────────────────────┐  │
│  │              Axum Middleware Stack (:8080)                            │  │
│  │                                                                       │  │
│  │  ┌────────┐  ┌──────────┐  ┌──────────┐  ┌────────┐  ┌───────────┐ │  │
│  │  │ Rate   │→ │  JWT     │→ │  RBAC    │→ │ Audit  │→ │  Input    │ │  │
│  │  │ Limiter│  │Validator │  │ Enforcer │  │  Log   │  │ Validator │ │  │
│  │  └────────┘  └──────────┘  └──────────┘  └────────┘  └───────────┘ │  │
│  └──────────────────────────────┬────────────────────────────────────────┘  │
├─────────────────────────────────┼───────────────────────────────────────────┤
│                         APPLICATION LAYER                                   │
│                                 │                                           │
│  ┌──────────────────────────────┼────────────────────────────────────────┐  │
│  │                   Service Layer                                       │  │
│  │                                                                       │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌───────────┐ │  │
│  │  │   Auth   │ │  Camera  │ │ Incident │ │ Evidence │ │Notification│ │  │
│  │  │ Service  │ │ Service  │ │ Service  │ │ Service  │ │  Service   │ │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘ └───────────┘ │  │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐              │  │
│  │  │   User   │ │   Role   │ │   Site   │ │  Audit   │              │  │
│  │  │ Service  │ │ Service  │ │ Service  │ │ Service  │              │  │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────┘              │  │
│  └──────────────────────────────┬────────────────────────────────────────┘  │
├─────────────────────────────────┼───────────────────────────────────────────┤
│                      PROCESSING LAYER                                       │
│                                 │                                           │
│  ┌────────────────────────┐     │     ┌──────────────────────────────────┐  │
│  │   Camera Gateway       │     │     │         AI Service               │  │
│  │   (Rust/Axum :8082)    ├─────┼────►│    (Python/FastAPI :8081)        │  │
│  │                        │     │     │                                  │  │
│  │  ┌──────────────────┐  │     │     │  ┌────────────┐ ┌────────────┐ │  │
│  │  │  RTSP Connection │  │     │     │  │   YOLO v8  │ │   OpenCV   │ │  │
│  │  │    Manager       │  │     │     │  │  Detector  │ │ Processor  │ │  │
│  │  └──────────────────┘  │     │     │  └────────────┘ └────────────┘ │  │
│  │  ┌──────────────────┐  │     │     └──────────────────────────────────┘  │
│  │  │ Frame Extractor  │  │     │                                           │
│  │  └──────────────────┘  │     │     ┌──────────────────────────────────┐  │
│  │  ┌──────────────────┐  │     │     │        IP Camera Fleet           │  │
│  │  │ Health Monitor   │  │◄────┼─────┤      (RTSP streams)              │  │
│  │  └──────────────────┘  │     │     └──────────────────────────────────┘  │
│  └────────────────────────┘     │                                           │
├─────────────────────────────────┼───────────────────────────────────────────┤
│                          DATA LAYER                                         │
│                                 │                                           │
│  ┌──────────────┐  ┌───────────┴──────┐  ┌──────────────┐  ┌────────────┐ │
│  │ PostgreSQL 16 │  │    Redis 7       │  │   Evidence   │  │ Prometheus │ │
│  │   :5432       │  │    :6379         │  │  Filesystem  │  │   :9090    │ │
│  │               │  │                  │  │              │  │            │ │
│  │ - Users       │  │ - JWT blocklist  │  │ - Video clips│  │ - Metrics  │ │
│  │ - Cameras     │  │ - Session cache  │  │ - Snapshots  │  │ - Alerts   │ │
│  │ - Incidents   │  │ - Rate limits    │  │ - Exports    │  │            │ │
│  │ - Evidence    │  │ - Permission     │  │              │  └────────────┘ │
│  │ - Audit logs  │  │   cache          │  │              │  ┌────────────┐ │
│  │ - Rules       │  │                  │  │              │  │   Loki     │ │
│  └──────────────┘  └──────────────────┘  └──────────────┘  │   :3100    │ │
│                                                             │            │ │
│                                                             │ - Logs     │ │
│                                                             └────────────┘ │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 8. Data Flow

### 8.1 Camera Feed to Dashboard (Primary Data Flow)

```
 IP Camera ──RTSP──► Camera Gateway ──Frames──► AI Service ──Detections──► Backend
   (554)               (8082)                   (8081)                    (8080)
                                                                         │
                                                     ┌───────────────────┘
                                                     │
                                                     ▼
                                              ┌─────────────┐
                                              │  PostgreSQL  │──── Dashboard (:3000)
                                              │   Redis      │          │
                                              │  Evidence FS │     WebSocket Push
                                              └─────────────┘          │
                                                                       ▼
                                                                 Security Operator
```

### 8.2 Data Flow Stages

| Stage | Component        | Input              | Processing                              | Output                    | Max Latency |
|-------|------------------|--------------------|-----------------------------------------|---------------------------|-------------|
| 1     | Camera Gateway   | RTSP stream        | Frame extraction, normalization         | Normalized frame          | < 100ms     |
| 2     | AI Service       | Normalized frame   | YOLO inference, NMS, tracking           | Detection results JSON    | < 200ms     |
| 3     | Backend          | Detection results  | Event generation, rule evaluation       | Events, alerts            | < 100ms     |
| 4     | Backend          | Alerts             | Incident creation, evidence capture     | Incidents, evidence refs  | < 200ms     |
| 5     | Backend          | Notifications      | WebSocket broadcast                     | Real-time push            | < 1s        |
| 6     | Dashboard        | WebSocket messages | UI rendering                            | Visual alerts             | < 1s        |

**End-to-end: Camera frame → Dashboard alert < 5 seconds**

### 8.3 Authentication Data Flow

```
 1. User submits credentials
         │
         ▼
 2. POST /api/v1/auth/login
         │
         ▼
 3. Backend validates credentials (Argon2id)
         │
         ├── Invalid → 401 Unauthorized
         │
         ▼
 4. Generate JWT access token (RS256, 15min expiry)
    Generate refresh token (7-day expiry)
         │
         ▼
 5. Store refresh token hash in PostgreSQL
    Store session context in Redis
         │
         ▼
 6. Return access_token (body) + refresh_token (httpOnly cookie)
         │
         ▼
 7. Subsequent requests carry Authorization: Bearer {access_token}
         │
         ▼
 8. Middleware validates JWT signature (RS256 public key)
         │
         ├── Invalid/Expired → 401 → Client uses refresh token
         │
         ▼
 9. RBAC middleware checks permissions against endpoint requirements
         │
         ├── Unauthorized → 403 Forbidden
         │
         ▼
 10. Request proceeds to route handler
```

### 8.4 JWT Token Structure

**Access Token Claims:**

```json
{
  "sub": "user-uuid",
  "email": "operator@vigilantai.com",
  "roles": ["security_analyst"],
  "sites": ["site-uuid-1"],
  "permissions": ["incidents.read", "evidence.read", "alerts.read"],
  "iss": "vigilantai",
  "aud": "vigilantai-api",
  "iat": 1719200000,
  "exp": 1719200900,
  "jti": "token-uuid"
}
```

| Parameter      | Access Token              | Refresh Token          |
|----------------|---------------------------|------------------------|
| Algorithm      | RS256                     | RS256                  |
| Expiry         | 15 minutes                | 7 days                 |
| Storage        | Client memory / header    | httpOnly cookie        |
| Rotation       | New on refresh            | Single-use, rotated    |
| Revocation     | Redis blocklist           | Database deletion      |

---

## 9. Design Decisions and Trade-offs

| Decision | Choice | Rationale | Trade-off |
|----------|--------|-----------|-----------|
| **Rust for backend** | Axum/Tokio | Memory safety, zero-cost abstractions, predictable latency for security-critical workloads | Steeper learning curve; smaller ecosystem than Go/Java |
| **Python for AI** | FastAPI + YOLO | Mature ML/CV ecosystem; GPU acceleration; rapid model iteration | Separate process boundary adds latency; GIL limits CPU parallelism |
| **PostgreSQL 16** | Primary DB | ACID compliance, JSON support, full-text search, proven reliability | Heavier than SQLite; requires separate server |
| **Redis 7** | Cache/session store | Sub-millisecond latency; pub/sub for real-time; TTL-based expiry | Additional infrastructure component; single point of failure without clustering |
| **JWT RS256** | Asymmetric signing | Public key verification without network calls; supports key rotation | No built-in revocation (requires blocklist); token cannot be invalidated server-side without Redis |
| **Argon2id** | Password hashing | Memory-hard; resistant to GPU/ASIC attacks; OWASP recommended | Slower than bcrypt; requires more memory per hash |
| **Next.js 15** | Dashboard | Server components, optimized bundling, TypeScript-first | Build complexity; Node.js runtime requirement |
| **Docker Compose** | Local dev/MVP | Simple multi-container orchestration; single-file config | Not suitable for production HA; no auto-scaling |
| **Kubernetes** | Production deployment | Auto-scaling (HPA), self-healing, rolling updates, service discovery | Operational complexity; requires cluster management expertise |
| **Filesystem for evidence** | Initial storage | Simple; no external dependencies; fast local I/O | Not distributed; no replication; limited scalability |
| **Prometheus + Grafana** | Observability | Industry standard; rich query language; pre-built alerting | Prometheus pull-based model may not suit all network topologies |
| **Loki for logs** | Log aggregation | Label-based indexing (cost-effective); Grafana integration | Less full-text search capability than Elasticsearch |

### 9.1 Known Limitations

| Limitation | Impact | Planned Mitigation |
|------------|--------|-------------------|
| Logout is a no-op (no token blacklist) | Invalidated tokens remain valid until expiry | Implement Redis-based JWT blocklist |
| Camera pagination loads all records | Memory pressure with large camera fleets | Implement cursor-based pagination |
| AI inference hardcoded to YOLO | Cannot swap detection models without code changes | Introduce model registry abstraction |
| Batch inference endpoint is a stub | No bulk frame processing capability | Implement actual batch pipeline |
| CORS not configured in backend | Potential cross-origin issues in production | Add CORS middleware with configurable origins |
| Hardcoded "user" role in JWT claims | All tokens include "user" regardless of actual roles | Load actual roles from database during token generation |

### 9.2 Future Considerations

| Area | Technology | Purpose |
|------|------------|---------|
| Distributed Tracing | OpenTelemetry + Tempo | End-to-end request tracing across services |
| API Versioning | URL path (`/api/v2/`) | Breaking change management |
| Multi-tenancy | Row-level security | Isolated tenant data |
| Edge Deployment | Lightweight gateway + model | On-premise camera processing |
| Mobile App | React Native | Mobile security operations |
| Helm Charts | Kubernetes packaging | Standardized deployment |
