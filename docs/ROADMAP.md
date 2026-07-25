# VigilantAI — Product Roadmap

> **Enterprise Security Intelligence Platform**
> Product Roadmap — v1.0

---

## Table of Contents

| Section | Title                                |
|---------|--------------------------------------|
| 1       | Roadmap Overview                     |
| 2       | v1.0.0 (Current Release)             |
| 3       | v1.1.0 (Enhanced Analytics)          |
| 4       | v1.2.0 (Multi-Tenant)               |
| 5       | v2.0.0 (Edge & Mobile)              |
| 6       | Known Limitations                    |
| 7       | Future Considerations                |

---

## 1. Roadmap Overview

```
  v1.0.0              v1.1.0              v1.2.0              v2.0.0
    │                   │                   │                   │
    ▼                   ▼                   ▼                   ▼
┌─────────┐      ┌──────────┐      ┌──────────┐      ┌──────────────┐
│  Core   │ ──►  │ Enhanced │ ──►  │  Multi-  │ ──►  │    Edge      │
│ Platform│      │Analytics │      │  Tenant  │      │  Deployment  │
└─────────┘      └──────────┘      └──────────┘      └──────────────┘
 Auth              WebSocket          API versioning    Federated learning
 Cameras           Batch inference    Advanced RBAC     Mobile app
 Incidents         Real-time alerts   Tenant isolation  Offline support
 Evidence          Dashboard v2       Custom roles      
 Notifications     Reporting          SSO integration   
```

---

## 2. v1.0.0 — Core Platform (Current)

**Status:** Released
**Date:** July 2026

### 2.1 Features

| Module            | Capabilities                                            |
|-------------------|---------------------------------------------------------|
| **Authentication** | JWT RS256, access/refresh tokens, Argon2id passwords   |
| **Authorization**  | RBAC with 6 roles and 23 permissions                   |
| **Users**          | CRUD operations, role assignment, site scoping          |
| **Cameras**        | CRUD, enable/disable, health monitoring                 |
| **Camera Groups**  | Hierarchical grouping, site assignment                  |
| **Incidents**      | CRUD, status transitions, assignment, notes             |
| **Evidence**       | Upload (JPEG/PNG/MP4), SHA-256 integrity, download     |
| **Notifications**  | Send, retry, dashboard/email channels                   |
| **Rules**          | Detection rule configuration, enable/disable            |
| **Sites**          | Site management with geolocation                        |
| **Health**         | Per-service health endpoints, Prometheus metrics        |
| **Dashboard**      | Next.js 15, live feeds, alert console, incident manager|
| **AI Service**     | YOLOv8 inference, CPU/CUDA support                     |
| **Camera Gateway** | RTSP ingestion, frame extraction, reconnection         |
| **Monitoring**     | Prometheus, Grafana (5 dashboards), Loki, Promtail     |
| **Deployment**     | Docker Compose, Kubernetes manifests, HPA              |

### 2.2 Infrastructure

| Component       | Specification                                |
|-----------------|----------------------------------------------|
| Backend         | Rust 1.78+, Axum 0.7, Tokio, SQLx 0.8       |
| AI Service      | Python 3.11+, FastAPI, YOLOv8, OpenCV        |
| Camera Gateway  | Rust 1.78+, Axum 0.7, RTSP client            |
| Dashboard       | Next.js 15, React 19, TypeScript 5, Tailwind 4|
| PostgreSQL      | 16 (Alpine)                                   |
| Redis           | 7 (Alpine)                                    |
| Prometheus      | v2.54.1                                       |
| Grafana         | 11.2.2                                        |
| Loki            | 3.2.1                                         |
| Promtail        | 3.2.1                                         |
| Alertmanager    | v0.27                                         |

---

## 3. v1.1.0 — Enhanced Analytics

**Status:** Planned
**Target:** Q4 2026

### 3.1 Features

| Feature                     | Description                                           | Priority |
|-----------------------------|-------------------------------------------------------|----------|
| Real-time WebSocket updates | Live alert and incident push to dashboard             | High     |
| Batch inference endpoint    | Process multiple frames in a single request           | High     |
| Detection analytics         | Trend analysis, heatmap, time-series charts           | High     |
| Enhanced reporting          | PDF/CSV export, scheduled reports                     | Medium   |
| Rule engine v2              | Complex conditions, time-based rules, zone rules      | High     |
| Dashboard v2                | Improved layout, camera grid, dark mode               | Medium   |
| Evidence viewer             | In-browser video player, frame-by-frame scrubbing     | Medium   |
| Camera health timeline      | Historical uptime, FPS, bitrate visualization         | Low      |
| Notification channels v2    | Email templates, webhook retry, rate limiting          | Medium   |
| Search improvements         | Full-text search across incidents and detections      | Medium   |

### 3.2 Technical Debt

| Item                              | Description                                    |
|-----------------------------------|------------------------------------------------|
| WebSocket implementation          | Add Axum WebSocket upgrade with broadcast       |
| Batch inference                   | Implement actual batch processing in AI service |
| Evidence pagination               | Cursor-based pagination for evidence list       |
| Dashboard state management        | Migrate to React Query for data fetching        |

---

## 4. v1.2.0 — Multi-Tenant

**Status:** Planned
**Target:** Q2 2027

### 4.1 Features

| Feature                     | Description                                           | Priority |
|-----------------------------|-------------------------------------------------------|----------|
| Multi-tenant support        | Tenant isolation, data partitioning                   | High     |
| Advanced RBAC               | Custom roles, permission templates, role inheritance   | High     |
| API versioning (v2)         | URL path versioning, deprecation headers              | High     |
| SSO integration             | OAuth 2.0 / OIDC with Microsoft, Google, Okta         | High     |
| Tenant management           | Tenant CRUD, billing integration, resource quotas     | Medium   |
| Custom dashboards           | User-configurable dashboard layouts                   | Medium   |
| API key management          | API keys for integrations with scoped permissions     | Medium   |
| Audit log export            | SIEM integration, CSV/JSON export                     | Medium   |
| Data residency              | Configurable data storage regions                     | Low      |

### 4.2 Architecture Changes

| Change                          | Description                                       |
|---------------------------------|---------------------------------------------------|
| Row-level security              | PostgreSQL RLS policies for tenant isolation       |
| API gateway v2                  | Path-based versioning with deprecation support     |
| Identity provider integration   | OIDC middleware for SSO                            |
| Tenant-scoped caching           | Redis key namespacing by tenant ID                 |

---

## 5. v2.0.0 — Edge Deployment

**Status:** Future
**Target:** Q4 2027

### 5.1 Features

| Feature                     | Description                                           | Priority |
|-----------------------------|-------------------------------------------------------|----------|
| Edge deployment             | Lightweight gateway + model for on-premise            | High     |
| Federated learning          | Distributed model training across sites               | Medium   |
| Mobile app                  | React Native for iOS/Android                          | High     |
| Offline support             | Local caching, queue-and-sync for disconnected ops   | High     |
| Camera AI on edge           | Run YOLO inference at the camera/edge node            | Medium   |
| Voice alerts                | Text-to-speech for critical alerts                    | Low      |
| Map view                    | Geographic camera and incident visualization          | Medium   |
| Integration marketplace     | Third-party plugin system                             | Low      |

### 5.2 Architecture Changes

| Change                          | Description                                       |
|---------------------------------|---------------------------------------------------|
| Edge gateway binary             | Standalone Rust binary with embedded AI model     |
| Federated model aggregation     | Secure model update distribution                  |
| Mobile push notifications       | FCM/APNs integration                              |
| Offline-first data sync         | CRDT-based conflict resolution                    |

---

## 6. Known Limitations

The following issues were identified during the v1.0.0 security audit:

### 6.1 Logout Is a No-Op

| Field         | Detail                                              |
|---------------|-----------------------------------------------------|
| Severity      | Medium                                              |
| Description   | The logout endpoint does not invalidate the access token |
| Impact        | Tokens remain valid until natural expiry (15 min)   |
| Mitigation    | Implement Redis-based JWT blocklist using token JTI |
| Target        | v1.1.0                                              |

### 6.2 Camera Pagination Loads All Records

| Field         | Detail                                              |
|---------------|-----------------------------------------------------|
| Severity      | Medium                                              |
| Description   | Camera list endpoint loads all records before paginating |
| Impact        | Memory pressure with large camera fleets (1000+)    |
| Mitigation    | Implement cursor-based or keyset pagination         |
| Target        | v1.1.0                                              |

### 6.3 AI Inference Hardcoded to YOLO

| Field         | Detail                                              |
|---------------|-----------------------------------------------------|
| Severity      | Low                                                 |
| Description   | Detection model is hardcoded; cannot swap without code changes |
| Impact        | Cannot use custom or alternative models             |
| Mitigation    | Introduce model registry abstraction layer          |
| Target        | v1.2.0                                              |

### 6.4 Batch Inference Endpoint Is a Stub

| Field         | Detail                                              |
|---------------|-----------------------------------------------------|
| Severity      | Low                                                 |
| Description   | Batch inference endpoint returns empty response     |
| Impact        | No bulk frame processing capability                 |
| Mitigation    | Implement actual batch pipeline in AI service       |
| Target        | v1.1.0                                              |

### 6.5 CORS Not Configured in Backend

| Field         | Detail                                              |
|---------------|-----------------------------------------------------|
| Severity      | Medium                                              |
| Description   | No CORS middleware configured in the Axum backend   |
| Impact        | Cross-origin requests may be blocked by browsers    |
| Mitigation    | Add tower-http CORS layer with configurable origins |
| Target        | v1.0.1 (hotfix)                                     |

### 6.6 Hardcoded "user" Role in JWT Claims

| Field         | Detail                                              |
|---------------|-----------------------------------------------------|
| Severity      | Medium                                              |
| Description   | JWT access tokens include `"roles": ["user"]` regardless of actual assigned roles |
| Impact        | RBAC middleware does not reflect true user permissions from token |
| Mitigation    | Load actual roles from database during token generation |
| Target        | v1.0.1 (hotfix)                                     |

### 6.7 Summary

| ID  | Limitation                         | Severity | Target    |
|-----|-------------------------------------|----------|-----------|
| L-1 | Logout is a no-op                   | Medium   | v1.1.0    |
| L-2 | Camera pagination loads all records | Medium   | v1.1.0    |
| L-3 | AI inference hardcoded to YOLO      | Low      | v1.2.0    |
| L-4 | Batch inference is a stub           | Low      | v1.1.0    |
| L-5 | CORS not configured                 | Medium   | v1.0.1    |
| L-6 | Hardcoded user role in JWT          | Medium   | v1.0.1    |

---

## 7. Future Considerations

### 7.1 Observability

| Technology           | Purpose                                       | Status   |
|----------------------|-----------------------------------------------|----------|
| OpenTelemetry        | Standardized telemetry collection             | Planned  |
| Tempo                | Distributed tracing                           | Planned  |
| OTLP                 | OpenTelemetry Protocol for log/metric/trace export | Planned |
| Jaeger               | Alternative tracing backend                   | Evaluated|

### 7.2 Deployment

| Technology           | Purpose                                       | Status   |
|----------------------|-----------------------------------------------|----------|
| Helm charts          | Kubernetes package management                 | Planned  |
| ArgoCD               | GitOps continuous delivery                    | Evaluated|
| Terraform            | Infrastructure as code                        | Evaluated|
| Crossplane           | Kubernetes-native infrastructure              | Evaluated|

### 7.3 AI/ML

| Technology           | Purpose                                       | Status   |
|----------------------|-----------------------------------------------|----------|
| Model registry       | Centralized model version management          | Planned  |
| TensorRT             | Optimized GPU inference                       | Evaluated|
| ONNX Runtime         | Cross-platform model serving                  | Evaluated|
| Active learning      | Human-in-the-loop model improvement           | Researched|
| Custom model training| User-uploadable training data and models      | Researched|

### 7.4 Integration

| Technology           | Purpose                                       | Status   |
|----------------------|-----------------------------------------------|----------|
| SIEM integration     | Export events to Splunk, Sentinel, etc.       | Planned  |
| Webhook v2           | Configurable webhook payloads and retries     | Planned  |
| SCIM                 | User provisioning from external directories   | Evaluated|
| MQTT                 | IoT device communication                      | Researched|

### 7.5 Product

| Feature              | Purpose                                       | Status   |
|----------------------|-----------------------------------------------|----------|
| Mobile app           | iOS/Android security operations               | Planned  |
| Map view             | Geographic camera/incident visualization      | Evaluated|
| Custom reports       | User-defined report builder                   | Researched|
| Audit log explorer   | Interactive audit trail visualization         | Researched|
| SLA tracking         | Incident SLA monitoring and alerts            | Planned  |

---

## Version History

| Version  | Date       | Theme                      | Status    |
|----------|------------|----------------------------|-----------|
| v1.0.0   | 2026-07-22 | Core Platform              | Released  |
| v1.0.1   | TBD        | CORS + JWT hotfixes        | Planned   |
| v1.1.0   | Q4 2026    | Enhanced Analytics         | Planned   |
| v1.2.0   | Q2 2027    | Multi-Tenant               | Planned   |
| v2.0.0   | Q4 2027    | Edge Deployment & Mobile   | Future    |
