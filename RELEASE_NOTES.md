# VigilantAI Enterprise v1.0.0

## Version

v1.0.0 - Initial Enterprise Release

## Release Date

July 2026

## Major Features

- JWT RS256 authentication with access/refresh tokens
- RBAC with 6 roles (`system_admin`, `security_admin`, `security_analyst`, `operator`, `viewer`, `api_integration`) and 23 permissions
- Camera management (CRUD, enable/disable, status tracking)
- Incident management (CRUD, severity levels, status workflow, evidence linking)
- Evidence management (file upload, SHA-256 dedup, storage, retrieval, deletion)
- Notification system (webhook + email, retry logic, rate limiting)
- AI inference service (YOLOv8, object detection, confidence scoring)
- Camera gateway (multi-camera support, frame processing, AI backend integration)
- Next.js dashboard (11 routes, dark theme, shadcn/ui, React Query)
- Docker containerization (11 services with health checks)
- Kubernetes deployment (35+ manifests, HPA, Ingress, PV/PVC)
- Prometheus monitoring (10+ metrics per service, 12 alert rules, 12 recording rules)
- Grafana dashboards (5 dashboards, 65+ panels, auto-provisioned)
- Loki centralized logging (structured JSON, Promtail collection)

## Architecture

VigilantAI Enterprise is a microservices-based security surveillance platform composed of 11 services. The backend is built with Rust (Axum) for core API services and Python (FastAPI) for AI inference. The frontend is a Next.js 15 dashboard with React 18 and shadcn/ui. Data is stored in PostgreSQL 16 with Redis 7 for caching and session management. Observability is provided by Prometheus, Grafana, and Loki. The entire stack is containerized with Docker and deployable to Kubernetes with full HPA auto-scaling and Ingress routing.

## Known Limitations

- Logout is a no-op (no token blacklist)
- Camera pagination loads all records then slices
- AI inference limited to YOLO models
- Batch inference endpoint is a stub
- CORS not configured in backend
- Hardcoded "user" role in JWT claims
- Prometheus uses emptyDir in K8s (data lost on restart)
- Grafana uses emptyDir in K8s (data lost on restart)

## Future Work

- **v1.1.0:** WebSocket real-time updates, batch inference, enhanced analytics
- **v1.2.0:** Multi-tenant, API versioning, advanced RBAC
- **v2.0.0:** Edge deployment, mobile app, federated learning

## Upgrade Notes

This is the initial release. No upgrade path is needed.

## Dependencies

| Dependency       | Version |
| ---------------- | ------- |
| Rust             | 1.82    |
| Axum             | 0.7     |
| SQLx             | 0.8     |
| Python           | 3.12    |
| FastAPI          | 0.115   |
| Ultralytics      | 8.3     |
| Node.js          | 20      |
| Next.js          | 15      |
| React            | 18      |
| PostgreSQL       | 16      |
| Redis            | 7       |
| Prometheus       | v2.54   |
| Grafana          | 11.2    |
| Loki             | 3.2     |
