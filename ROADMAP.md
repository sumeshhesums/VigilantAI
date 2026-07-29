# VigilantAI — Enterprise Product Roadmap

> **Enterprise AI-Powered Security Surveillance Platform**
> Product Roadmap — v1.0.0

---

## Table of Contents

| Section | Title |
|---------|-------|
| 1 | Product Vision |
| 2 | Architecture Overview |
| 3 | Phase 1 — Project Stabilization |
| 4 | Phase 2 — Production Readiness |
| 5 | Phase 3 — Cloud Deployment |
| 6 | Phase 4 — Mobile Platform |
| 7 | Phase 5 — Real Camera Integration |
| 8 | Phase 6 — Advanced AI |
| 9 | Phase 7 — Enterprise Features |
| 10 | Phase 8 — Enterprise Dashboard |
| 11 | Known Issues & Technical Debt |
| 12 | Appendix: Every Phase Must Include |

---

## 1. Product Vision

VigilantAI is a full-stack, AI-powered security surveillance platform designed for real-time threat detection, intelligent video analytics, and centralized security operations. It combines edge computing, deep learning, and modern web technologies to deliver a scalable, production-ready surveillance solution.

### Long-Term Goal

Transform VigilantAI into a SaaS Enterprise Platform supporting:

- **Web** — Next.js dashboard
- **Android & iOS** — Flutter mobile app
- **Cloud Deployment** — AWS, Azure, GCP
- **Real Cameras** — RTSP, ONVIF, Hikvision, Dahua, Axis
- **Live Streaming** — Sub-100ms latency
- **Enterprise Authentication** — SSO, LDAP, OIDC
- **Monitoring** — Prometheus, Grafana, Loki, Tempo
- **Analytics** — Real-time dashboards, reporting
- **Notifications** — Email, SMS, Push, Webhook
- **AI Detection** — YOLO, RT-DETR, Grounding DINO
- **Multi-Tenant Customers** — Organizations, RBAC, isolation

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                        Docker / Kubernetes                          │
│                                                                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐              │
│  │   Dashboard   │  │   Backend    │  │   Camera     │              │
│  │  (Next.js)   │  │  (Rust/Axum) │  │   Gateway    │              │
│  │    :3000     │  │    :8080     │  │  (Rust/Axum) │              │
│  └──────┬───────┘  └──────┬───────┘  │    :8082     │              │
│         │                 │           └──────┬───────┘              │
│         └─────────────────┼──────────────────┘                      │
│                           │                                         │
│                   ┌───────▼───────┐                                 │
│                   │   AI Service  │                                 │
│                   │ (Python/FastAPI)                                │
│                   │    :8081      │                                 │
│                   └───────┬───────┘                                 │
│                           │                                         │
│  ┌──────────────┐  ┌──────▼───────┐  ┌──────────────┐              │
│  │  PostgreSQL  │  │    Redis     │  │  Prometheus  │              │
│  │    :5432     │  │    :6379     │  │   + Grafana  │              │
│  └──────────────┘  └──────────────┘  │  :9090/3001  │              │
│                                       └──────────────┘              │
│  ┌──────────────┐  ┌──────────────┐                                 │
│  │     Loki     │  │   Promtail   │                                 │
│  │    :3100     │  │    :9080     │                                 │
│  └──────────────┘  └──────────────┘                                 │
└─────────────────────────────────────────────────────────────────────┘
```

### Current Tech Stack

| Layer | Technology |
|-------|-----------|
| Backend API | Rust, Axum 0.7, Tokio, SQLx 0.8 |
| AI Service | Python 3.11+, FastAPI, YOLOv8, OpenCV |
| Camera Gateway | Rust, Axum 0.7, RTSP |
| Dashboard | Next.js 15, React 18, TypeScript 5, Tailwind CSS |
| Database | PostgreSQL 16 (Alpine) |
| Cache | Redis 7 (Alpine) |
| Monitoring | Prometheus, Grafana 11, Loki, Promtail |
| Auth | JWT RS256, Argon2id, RBAC |
| Containerization | Docker, Docker Compose |

---

## 3. Phase 1 — Project Stabilization

**Status:** IN PROGRESS
**Target:** Immediate

### Objectives

1. Fix all runtime issues and bugs
2. Ensure 100% working local deployment
3. Improve reliability and error handling
4. Complete missing database migrations
5. Fix Docker Compose configuration (circular deps, volumes)
6. Add JWT key generation scripts
7. Add seed data for first-run experience
8. Fix hardcoded values (role, CORS)
9. Add Windows/PowerShell development scripts
10. Verify all services build, test, and run

### Deliverables

| # | Task | Status |
|---|------|--------|
| 1.1 | Fix JWT key config — add generation script (OpenSSL + PowerShell) | PENDING |
| 1.2 | Fix docker-compose circular dependency (prometheus → backend) | PENDING |
| 1.3 | Fix evidence_data volume definition in docker-compose | PENDING |
| 1.4 | Fix hardcoded `"user"` role in auth_service.rs | PENDING |
| 1.5 | Fix camera_repository LIMIT/OFFSET bind order | PENDING |
| 1.6 | Add proper health check that validates DB + Redis connectivity | PENDING |
| 1.7 | Add seed data SQL script (roles, permissions, admin user) | PENDING |
| 1.8 | Add JWT key generation script (generate-keys.ps1 + generate-keys.sh) | PENDING |
| 1.9 | Add Windows PowerShell development scripts (start.ps1, stop.ps1) | PENDING |
| 1.10 | Clean up temp files in project root | PENDING |
| 1.11 | Verify backend builds with `cargo build` | PENDING |
| 1.12 | Verify backend tests pass with `cargo test` | PENDING |
| 1.13 | Verify Docker Compose starts all services | PENDING |
| 1.14 | Update .env.example with correct defaults | PENDING |
| 1.15 | Update README.md with Phase 1 status | PENDING |

### Risk Analysis

| Risk | Impact | Likelihood | Mitigation |
|------|--------|-----------|------------|
| Build failures due to Rust version mismatches | High | Low | Use workspace-level rust-toolchain.toml |
| Docker image build failures | High | Medium | Test docker-compose build after fixes |
| Database migration conflicts | Medium | Low | All migrations are additive, no destructive changes |
| Python dependency issues | Medium | Low | requirements.txt pinned to specific versions |
| Hardcoded values cause authentication failures | High | High | Priority fix — hardcoded role addressed in 1.4 |

### Rollback Plan

- All changes are additive and backwards-compatible
- No destructive migrations
- Environment variables maintain backwards compatibility
- Old config files preserved as `.example` variants

---

## 4. Phase 2 — Production Readiness

**Status:** Planned
**Target:** After Phase 1

### Objectives

1. Configuration management with environment validation
2. Secrets management (Vault or encrypted env)
3. Health check enhancements (deep health with dependency checks)
4. Structured logging with correlation IDs
5. Rate limiting middleware
6. Request timeout middleware
7. CORS configuration with allowlist
8. Security headers (HSTS, CSP, X-Frame-Options)
9. Database connection pooling optimization
10. Redis connection pooling optimization
11. Graceful shutdown improvements
12. Container security (non-root, read-only rootfs)
13. Docker image size optimization
14. Prometheus alerting rules refinement
15. Grafana dashboard improvements

### Deliverables

| # | Task |
|---|------|
| 2.1 | Config validation at startup — fail fast on missing required vars |
| 2.2 | Add `dotenvy` `.env` file validation |
| 2.3 | Structured JSON logging across all services |
| 2.4 | Add rate limiting middleware to backend |
| 2.5 | Add request timeout middleware |
| 2.6 | Configurable CORS allowlist (from env) |
| 2.7 | Security headers middleware |
| 2.8 | Health endpoint checks DB + Redis connectivity |
| 2.9 | DB pool config with env-based min/max connections |
| 2.10 | Redis connection manager with health checks |
| 2.11 | Implement token blacklist cleanup job |
| 2.12 | Docker security — non-root, read-only, no shell |
| 2.13 | Multi-stage Docker builds for Rust (already done — validate) |
| 2.14 | Prometheus alert rules — validate and fix scrape targets |
| 2.15 | Grafana dashboards — validate and add system health panels |

---

## 5. Phase 3 — Cloud Deployment

**Status:** Planned
**Target:** After Phase 2

### Objectives

1. Docker optimization for production
2. Reverse proxy (nginx/traefik) with HTTPS
3. Domain configuration with Let's Encrypt
4. AWS deployment (ECS/EKS)
5. Azure deployment (AKS)
6. GCP deployment (GKE)
7. CI/CD pipeline (GitHub Actions)
8. Blue/green deployment strategy
9. Database backup and restore
10. Disaster recovery plan
11. Load testing
12. Auto-scaling configuration
13. CDN for static assets
14. WAF configuration

---

## 6. Phase 4 — Mobile Platform

**Status:** Planned
**Target:** After Phase 3

### Objectives

1. Flutter mobile app (Android + iOS)
2. Push notifications (FCM/APNs)
3. JWT authentication flow
4. Camera dashboard
5. Incident viewer
6. Evidence viewer
7. Live streaming viewer
8. Offline support
9. Biometric authentication
10. Mobile-specific RBAC

---

## 7. Phase 5 — Real Camera Integration

**Status:** Planned
**Target:** After Phase 4

### Objectives

1. RTSP protocol support (already partial)
2. ONVIF protocol implementation
3. Hikvision camera integration
4. Dahua camera integration
5. Axis camera integration
6. Live stream transcoding
7. Camera auto-discovery
8. Camera health monitoring
9. Recording management
10. PTZ control

---

## 8. Phase 6 — Advanced AI

**Status:** Planned
**Target:** After Phase 5

### Objectives

1. Weapon detection
2. Fire and smoke detection
3. Intrusion detection (zone-based)
4. PPE detection (hard hat, vest, gloves)
5. Vehicle detection and classification
6. Crowd detection and counting
7. Face recognition (compliant with privacy regulations)
8. License plate recognition (ANPR/LPR)
9. Behavior analysis (loitering, running, fighting)
10. Multi-model ensemble inference

---

## 9. Phase 7 — Enterprise Features

**Status:** Planned
**Target:** After Phase 6

### Objectives

1. Organizations / multi-tenant architecture
2. User management with SCIM provisioning
3. Audit logging (immutable, tamper-evident)
4. Reporting engine (scheduled, exportable)
5. Role management with custom roles
6. Notification rules engine
7. Email notifications (SMTP, SendGrid, SES)
8. SMS notifications (Twilio)
9. Push notifications (FCM/APNs)
10. Webhook integrations
11. SLA management
12. Billing and subscription integration

---

## 10. Phase 8 — Enterprise Dashboard

**Status:** Planned
**Target:** After Phase 7

### Objectives

1. Modern UI refresh
2. Map view with camera locations
3. Live camera grid with multi-view
4. Analytics dashboard with charts
5. AI statistics and model performance
6. System health overview
7. Enterprise branding (themes, logos)
8. Custom dashboard layouts
9. Dark mode / light mode
10. Accessibility (WCAG 2.1 AA)
11. Internationalization (i18n)
12. Real-time updates via WebSocket

---

## 11. Known Issues & Technical Debt

### Critical (Phase 1)

| ID | Issue | Severity | Status |
|----|-------|----------|--------|
| C-1 | JWT keys empty in `.env.example` — no generation script | High | FIXING |
| C-2 | Hardcoded `"user"` role in `auth_service.rs:71` | High | FIXING |
| C-3 | Docker Compose: prometheus `depends_on: backend` creates circular dependency | Medium | FIXING |
| C-4 | `evidence_data` volume referenced but not defined in docker-compose volumes | High | FIXING |
| C-5 | Camera pagination `LIMIT/OFFSET` bind order reversed in `camera_repository.rs:73-76` | Medium | FIXING |
| C-6 | No seed data — first run has no admin user or roles | High | FIXING |
| C-7 | Temp files in root (`tok.txt`, `temp_*.json`, etc.) | Low | FIXING |

### Medium Priority (Phase 2)

| ID | Issue | Severity | Target |
|----|-------|----------|--------|
| M-1 | CORS wide open (`allow_origin(Any)`) | Medium | Phase 2 |
| M-2 | No rate limiting on API endpoints | Medium | Phase 2 |
| M-3 | Health endpoint doesn't check DB/Redis connectivity | Medium | Phase 2 |
| M-4 | Token blacklist check silently swallows Redis errors | Medium | Phase 2 |
| M-5 | No request timeout middleware | Medium | Phase 2 |
| M-6 | AI Service CORS wide open | Low | Phase 2 |
| M-7 | Grafana dashboards use placeholder data | Low | Phase 2 |

### Future (Phase 3+)

| ID | Issue | Severity | Target |
|----|-------|----------|--------|
| F-1 | No HTTPS/TLS termination | High | Phase 3 |
| F-2 | No CI/CD pipeline | High | Phase 3 |
| F-3 | No database backup strategy | High | Phase 3 |
| F-4 | No mobile app | Medium | Phase 4 |
| F-5 | No real camera protocol support (ONVIF) | Medium | Phase 5 |
| F-6 | Limited AI detection models | Medium | Phase 6 |
| F-7 | No multi-tenant isolation | Medium | Phase 7 |

---

## 12. Appendix: Every Phase Must Include

### Architecture Review
- Review existing architecture for phase-specific changes
- Document architecture decisions (ADRs)
- Identify cross-cutting concerns

### Implementation Plan
- Detailed task breakdown
- Dependencies between tasks
- Estimated effort

### Files to Modify
- List of all files that will be changed
- New files to create

### Risk Analysis
- Identify risks specific to this phase
- Likelihood and impact assessment
- Mitigation strategies

### Testing Strategy
- Unit tests for new code
- Integration tests for API changes
- End-to-end verification

### Docker Verification
- Build all Docker images
- Start all services
- Verify health endpoints

### Browser Verification
- Access dashboard
- Verify all pages load
- Verify responsive design

### API Verification
- Test all API endpoints
- Verify authentication
- Verify RBAC enforcement

### Production Verification
- Verify logging
- Verify metrics
- Verify alerting
- Verify security headers

### Rollback Plan
- Revert strategy for each change
- Data migration rollback
- Configuration rollback

---

## Version History

| Version | Date | Theme | Status |
|---------|------|-------|--------|
| v1.0.0 | 2026-07 | Core Platform | Released |
| v1.0.1 | 2026-07 | Phase 1: Stabilization | IN PROGRESS |
| v1.1.0 | TBD | Phase 2: Production Readiness | Planned |
| v1.2.0 | TBD | Phase 3: Cloud Deployment | Planned |
| v2.0.0 | TBD | Phase 4: Mobile Platform | Planned |
| v2.1.0 | TBD | Phase 5: Real Cameras | Planned |
| v2.2.0 | TBD | Phase 6: Advanced AI | Planned |
| v3.0.0 | TBD | Phase 7: Enterprise Features | Planned |
| v3.1.0 | TBD | Phase 8: Enterprise Dashboard | Planned |
