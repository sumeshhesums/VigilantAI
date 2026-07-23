# VigilantAI — Developer Guide

> **Enterprise Security Intelligence Platform**
> Developer Guide — Version 1.0

---

## Table of Contents

| Section | Title                                           |
|---------|-------------------------------------------------|
| 1       | Document Control                                |
| 2       | Revision History                                |
| 3       | Introduction                                    |
| 4       | Purpose                                         |
| 5       | Scope                                           |
| 6       | Intended Audience                               |
| 7       | References                                      |
| 8       | Project Overview                                |
| 9       | High-Level System Overview                      |
| 10      | Repository Structure                            |
| 11      | Technology Stack Overview                       |
| 12      | Development Philosophy                          |
| 13      | Local Development Environment                   |
| 14      | Required Software                               |
| 15      | Hardware Requirements                           |
| 16      | IDE Recommendations                             |
| 17      | VS Code Configuration                           |
| 18      | Environment Variables                           |
| 19      | Configuration Strategy                          |
| 20      | Backend Development Guidelines                  |
| 21      | Rust Project Organization                       |
| 22      | AI Service Development Guidelines               |
| 23      | Python Project Organization                     |
| 24      | Frontend Development Guidelines                 |
| 25      | Next.js Project Organization                    |
| 26      | Database Development                            |
| 27      | Migration Strategy                              |
| 28      | Repository Layer Standards                      |
| 29      | Service Layer Standards                         |
| 30      | API Development Standards                       |
| 31      | Request Validation Standards                    |
| 32      | Response Standards                              |
| 33      | Error Handling Standards                        |
| 34      | Logging Standards                               |
| 35      | Authentication Integration                      |
| 36      | Authorization Integration                       |
| 37      | RBAC Development Rules                          |
| 38      | Site Scope Enforcement                          |
| 39      | Evidence Handling Standards                     |
| 40      | AI Integration Workflow                         |
| 41      | Dependency Management                           |
| 42      | Package Version Strategy                        |
| 43      | Git Workflow                                    |
| 44      | Branching Strategy                              |
| 45      | Commit Message Convention                       |
| 46      | Pull Request Guidelines                         |
| 47      | Code Review Checklist                           |
| 48      | Coding Standards                                |
| 49      | Naming Conventions                              |
| 50      | Folder Organization                             |
| 51      | Documentation Standards                         |
| 52      | Testing Philosophy                              |
| 53      | Unit Testing Guidelines                         |
| 54      | Integration Testing Guidelines                  |
| 55      | API Testing Guidelines                          |
| 56      | AI Testing Guidelines                           |
| 57      | Security Testing Responsibilities               |
| 58      | Performance Considerations                      |
| 59      | Memory Management Guidelines                    |
| 60      | Concurrency Guidelines                          |
| 61      | Async Programming Guidelines                    |
| 62      | Database Performance Guidelines                 |
| 63      | Caching Guidelines                              |
| 64      | Debugging Guide                                 |
| 65      | Logging During Development                      |
| 66      | Troubleshooting Guide                           |
| 67      | Common Development Tasks                        |
| 68      | Build Process Overview                          |
| 69      | Release Process Overview                        |
| 70      | CI/CD Integration Overview                      |
| 71      | Local Deployment Workflow                       |
| 72      | Production Awareness                            |
| 73      | Security Best Practices                         |
| 74      | Secret Handling Guidelines                      |
| 75      | Dependency Update Process                       |
| 76      | Vulnerability Management Responsibilities       |
| 77      | Common Mistakes to Avoid                        |
| 78      | Frequently Asked Questions                      |
| 79      | Developer Onboarding Checklist                  |
| 80      | Glossary                                        |
| 81      | Appendices                                      |

---

## 1. Document Control

| Field              | Value                                      |
|--------------------|---------------------------------------------|
| **Document Title** | Developer Guide                             |
| **Product Name**   | VigilantAI Enterprise Security Intelligence Platform |
| **Document Type**  | Engineering Handbook                        |
| **Version**        | 1.0                                         |
| **Date**           | 2026-07-21                                  |
| **Classification** | Internal — Confidential                     |
| **Owner**          | Engineering — Platform                      |
| **Approved By**    | *[Pending Approval]*                        |
| **Review Cycle**   | Quarterly                                   |
| **Distribution**   | All Engineers, DevOps, QA, Technical Writers |

---

## 2. Revision History

| Version | Date       | Author          | Changes                                      |
|---------|------------|-----------------|----------------------------------------------|
| 1.0     | 2026-07-21 | Platform Team   | Initial document creation                    |

---

## 3. Introduction

### 3.1 Purpose

This Developer Guide explains how to build, run, test, debug, maintain, and contribute to the VigilantAI Enterprise Security Intelligence Platform. It is a practical engineering handbook — not an architecture document, not an API specification, not source code.

This guide enables a new engineer to become productive using only this document and the preceding architecture documents (01-08).

### 3.2 Scope

This document covers:

- Development environment setup and configuration
- Project structure and code organization
- Coding standards and best practices
- Testing strategy and guidelines
- Git workflow and code review process
- Debugging and troubleshooting procedures
- Build, release, and CI/CD processes
- Security practices and secret handling
- Common development tasks and workflows

### 3.3 Intended Audience

| Role                          | Primary Use                                  |
|-------------------------------|----------------------------------------------|
| Backend Engineers (Rust)      | API development, service layer, database     |
| AI Engineers (Python)         | Detection engine, model integration          |
| Frontend Engineers (React)    | Dashboard UI, real-time features             |
| DevOps Engineers              | Build pipeline, deployment, monitoring       |
| QA Engineers                  | Testing strategy, test automation            |
| Technical Writers             | Documentation maintenance                    |
| New Team Members              | Onboarding and ramp-up                       |

### 3.4 References

| Document                                      | Description                                  |
|-----------------------------------------------|----------------------------------------------|
| docs/01-Executive-Summary.md                  | Product overview, architecture vision        |
| docs/02-Business-Requirements.md              | Business goals, personas, requirements       |
| docs/03-System-Requirements-Specification.md  | Functional/non-functional requirements       |
| docs/04-Software-Architecture.md              | Tech stack, component architecture           |
| docs/05-Database-Design.md                    | Entity definitions, storage design           |
| docs/06-API-Specification.md                  | API contracts, endpoints, rate limits        |
| docs/07-Security-Architecture.md              | Security controls, encryption, compliance    |
| docs/08-Deployment-Architecture.md            | Deployment topology, infrastructure          |

---

## 4. Project Overview

### 4.1 What VigilantAI Is

VigilantAI is an Enterprise Security Intelligence Platform that transforms traditional video surveillance into intelligent, AI-driven security operations. It combines AI-powered computer vision with a high-performance Rust event processing engine to provide real-time security monitoring.

The platform analyzes live camera feeds, detects security events using computer vision models, creates and manages incidents, applies configurable business rules, preserves forensic evidence, and delivers real-time visibility through a modern Security Operations dashboard.

### 4.2 What VigilantAI Is Not

- It is not a Video Management System (VMS) — it does not replace existing NVR/VMS recording infrastructure
- It does not provide playback or recording functions
- It is an intelligence layer that sits above existing camera infrastructure

### 4.3 Core Platform Modules

| Module                    | Responsibility                                              |
|---------------------------|-------------------------------------------------------------|
| Camera Gateway            | RTSP stream ingestion, frame extraction, connection pooling |
| AI Detection Engine       | Object detection, classification, tracking, zone evaluation |
| Event Processor           | Event generation, rule evaluation, alert triggering          |
| Incident Manager          | Incident lifecycle, escalation, assignment                   |
| Evidence Manager          | Evidence capture, integrity verification, chain of custody   |
| Rule Engine               | Configurable detection rules and business logic              |
| Notification Dispatcher   | Dashboard alerts, email delivery, webhook delivery           |
| Audit Service             | Immutable audit trail recording                              |
| Dashboard                 | Real-time security operations interface                      |

---

## 5. High-Level System Overview

### 5.1 System Architecture

```mermaid
graph TB
    subgraph "Presentation Layer"
        Dashboard[Next.js Dashboard]
    end

    subgraph "API Layer"
        LB[Load Balancer]
        API[Axum API Server]
        WS[WebSocket Server]
    end

    subgraph "Processing Layer"
        GW[Camera Gateway]
        AI[AI Inference Service]
        EP[Event Processor]
    end

    subgraph "Data Layer"
        PG[(PostgreSQL)]
        Redis[(Redis)]
        Evidence[Evidence Storage]
    end

    subgraph "Platform Services"
        Auth[Authentication]
        RBAC[Authorization]
        Audit[Audit Service]
    end

    Dashboard --> LB
    LB --> API
    API --> WS
    API --> GW
    GW --> AI
    AI --> EP
    API --> PG
    API --> Redis
    API --> Evidence
    API --> Auth
    API --> RBAC
    API --> Audit
```

### 5.2 Technology Stack Summary

| Layer           | Technology          | Purpose                                    |
|-----------------|---------------------|--------------------------------------------|
| Frontend        | Next.js, React, TypeScript, Tailwind CSS | Dashboard UI                    |
| Backend API     | Rust, Axum, Tokio, SQLx | REST API, WebSocket, business logic       |
| AI Service      | Python, FastAPI, YOLO, OpenCV | Computer vision, detection inference    |
| Camera Gateway  | Rust, Tokio         | RTSP ingestion, frame extraction           |
| Database        | SQLite (MVP), PostgreSQL (production) | Persistent data storage        |
| Cache           | Redis               | Session management, rule caching           |
| Deployment      | Docker, Docker Compose | Containerized deployment                  |
| Observability   | Prometheus, Grafana, Loki | Metrics, dashboards, logs             |

### 5.3 Request Lifecycle

```mermaid
sequenceDiagram
    participant Client
    participant Router
    participant Middleware
    participant Handler
    participant Service
    participant Repository
    participant Database

    Client->>Router: HTTP Request
    Router->>Middleware: Route Match
    Middleware->>Middleware: Rate Limit Check
    Middleware->>Middleware: JWT Validation
    Middleware->>Middleware: RBAC Authorization
    Middleware->>Middleware: Audit Log Entry
    Middleware->>Handler: Authorized Request
    Handler->>Handler: Request Validation (Serde)
    Handler->>Service: Business Operation
    Service->>Repository: Data Access
    Repository->>Database: SQL Query (SQLx)
    Database-->>Repository: Result Set
    Repository-->>Service: Domain Model
    Service-->>Handler: Service Response
    Handler-->>Client: HTTP Response (JSON)
```

---

## 6. Repository Structure

### 6.1 Top-Level Layout

```
vigilantai/
├── backend/              # Rust API server (Axum)
├── ai-service/           # Python AI inference service (FastAPI)
├── dashboard/            # Next.js frontend
├── camera-gateway/       # Rust RTSP gateway
├── docs/                 # Architecture and design documents
├── deploy/               # Docker Compose, deployment configs
├── scripts/              # Utility scripts (dev, CI, ops)
├── tests/                # Cross-service integration tests
├── .github/              # GitHub Actions workflows
├── docker-compose.yml    # Multi-container orchestration
├── Makefile              # Common development commands
└── README.md             # Project introduction
```

### 6.2 Backend Structure (Rust)

```
backend/
├── src/
│   ├── main.rs           # Application entry point
│   ├── app.rs            # Application state construction
│   ├── config.rs         # Configuration loading
│   ├── routes/           # API route definitions
│   │   ├── mod.rs
│   │   ├── auth.rs       # Authentication routes
│   │   ├── users.rs      # User management routes
│   │   ├── cameras.rs    # Camera management routes
│   │   ├── events.rs     # Detection event routes
│   │   ├── incidents.rs  # Incident management routes
│   │   ├── evidence.rs   # Evidence management routes
│   │   ├── rules.rs      # Rule management routes
│   │   ├── dashboard.rs  # Dashboard data routes
│   │   └── health.rs     # Health check routes
│   ├── middleware/        # Tower middleware
│   │   ├── mod.rs
│   │   ├── auth.rs       # JWT validation
│   │   ├── rbac.rs       # Authorization
│   │   ├── rate_limit.rs # Rate limiting
│   │   ├── audit.rs      # Audit logging
│   │   └── cors.rs       # CORS configuration
│   ├── services/         # Business logic
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   ├── user_service.rs
│   │   ├── camera_service.rs
│   │   ├── event_service.rs
│   │   ├── incident_service.rs
│   │   ├── evidence_service.rs
│   │   ├── rule_service.rs
│   │   └── audit_service.rs
│   ├── repositories/     # Data access
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   ├── camera_repository.rs
│   │   ├── event_repository.rs
│   │   ├── incident_repository.rs
│   │   ├── evidence_repository.rs
│   │   ├── rule_repository.rs
│   │   └── audit_repository.rs
│   ├── models/           # Domain models
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── camera.rs
│   │   ├── event.rs
│   │   ├── incident.rs
│   │   ├── evidence.rs
│   │   └── rule.rs
│   └── errors.rs         # Error types
├── migrations/           # SQLx migrations
│   ├── 001_initial/
│   └── ...
├── Cargo.toml            # Rust dependencies
└── .env                  # Environment variables (gitignored)
```

### 6.3 AI Service Structure (Python)

```
ai-service/
├── app/
│   ├── __init__.py
│   ├── main.py           # FastAPI application
│   ├── config.py         # Configuration
│   ├── routes/
│   │   ├── __init__.py
│   │   ├── health.py     # Health endpoints
│   │   └── inference.py  # Inference endpoints
│   ├── services/
│   │   ├── __init__.py
│   │   ├── detector.py   # YOLO detection
│   │   ├── tracker.py    # Object tracking
│   │   └── preprocessor.py # Frame preprocessing
│   ├── models/
│   │   ├── __init__.py
│   │   └── schemas.py    # Pydantic models
│   └── utils/
│       ├── __init__.py
│       └── logging.py    # Structured logging
├── models/               # YOLO model weights
├── tests/
├── requirements.txt
├── pyproject.toml
└── .env
```

### 6.4 Dashboard Structure (Next.js)

```
dashboard/
├── src/
│   ├── app/              # Next.js App Router
│   │   ├── layout.tsx
│   │   ├── page.tsx
│   │   ├── dashboard/
│   │   ├── cameras/
│   │   ├── incidents/
│   │   ├── evidence/
│   │   └── settings/
│   ├── components/       # React components
│   │   ├── ui/           # Base UI components
│   │   ├── dashboard/    # Dashboard-specific components
│   │   └── shared/       # Shared components
│   ├── hooks/            # Custom React hooks
│   ├── lib/              # Utilities, API client
│   ├── types/            # TypeScript type definitions
│   └── styles/           # Global styles, Tailwind config
├── public/               # Static assets
├── next.config.js
├── tailwind.config.js
├── tsconfig.json
├── package.json
└── .env.local
```

### 6.5 Directory Structure Diagram

```mermaid
graph TB
    ROOT[vigilantai/] --> BE[backend/]
    ROOT --> AI[ai-service/]
    ROOT --> FE[dashboard/]
    ROOT --> GW[camera-gateway/]
    ROOT --> DOCS[docs/]
    ROOT --> DEPLOY[deploy/]
    ROOT --> SCRIPTS[scripts/]
    ROOT --> TESTS[tests/]

    BE --> BE_SRC[src/]
    BE_SRC --> BE_ROUTES[routes/]
    BE_SRC --> BE_MW[middleware/]
    BE_SRC --> BE_SVC[services/]
    BE_SRC --> BE_REPO[repositories/]
    BE_SRC --> BE_MODEL[models/]

    AI --> AI_APP[app/]
    AI_APP --> AI_ROUTES[routes/]
    AI_APP --> AI_SVC[services/]
    AI_APP --> AI_MODELS[models/]

    FE --> FE_SRC[src/]
    FE_SRC --> FE_APP[app/]
    FE_SRC --> FE_COMP[components/]
    FE_SRC --> FE_HOOKS[hooks/]
    FE_SRC --> FE_LIB[lib/]
```

---

## 7. Technology Stack Overview

### 7.1 Backend Stack

| Component    | Selection | Version   | Purpose                                      |
|--------------|-----------|-----------|----------------------------------------------|
| Language     | Rust      | 1.78+     | Backend services, API, gateway               |
| Web Framework| Axum      | 0.7+      | HTTP/WebSocket request handling              |
| Async Runtime| Tokio     | 1.0+      | Async I/O, task scheduling                   |
| Database     | SQLx      | 0.8+      | Compile-time checked SQL, async pooling      |
| Serialization| Serde     | 1.0+      | JSON serialization/deserialization           |
| Middleware   | Tower     | 0.4+      | Composable middleware layers                 |
| Logging      | Tracing   | 0.1+      | Structured logging, distributed tracing      |
| Error handling| Thiserror| 1.0+      | Derive macros for error types                |

### 7.2 AI Service Stack

| Component    | Selection | Version   | Purpose                                      |
|--------------|-----------|-----------|----------------------------------------------|
| Language     | Python    | 3.11+     | AI inference service                         |
| Web Framework| FastAPI   | 0.100+    | HTTP API for internal service                |
| Detection    | YOLO      | 8.0+      | Real-time object detection                   |
| Vision       | OpenCV    | 4.8+      | Frame processing, color conversion           |
| Validation   | Pydantic  | 2.0+      | Request/response validation                  |
| Server       | Uvicorn   | 0.23+     | ASGI server                                  |

### 7.3 Frontend Stack

| Component    | Selection | Version   | Purpose                                      |
|--------------|-----------|-----------|----------------------------------------------|
| Framework    | Next.js   | 14+       | React framework, static export              |
| UI Library   | React     | 18+       | Component-based UI                          |
| Language     | TypeScript| 5.0+      | Type safety                                 |
| Styling      | Tailwind CSS | 3.3+  | Utility-first CSS                           |
| State        | SWR       | 2.0+      | Data fetching, caching                      |
| Charts       | Recharts  | 2.0+      | Dashboard visualizations                    |

### 7.4 Data Stack

| Component    | Selection | Version   | Purpose                                      |
|--------------|-----------|-----------|----------------------------------------------|
| Primary DB   | PostgreSQL| 16+       | Production database                         |
| Dev DB       | SQLite    | 3.0+      | Local development                           |
| Cache        | Redis     | 7+        | Session management, caching                 |
| Object Store | Filesystem| —         | Evidence storage (S3 in future)             |

### 7.5 Infrastructure Stack

| Component    | Selection | Version   | Purpose                                      |
|--------------|-----------|-----------|----------------------------------------------|
| Containers   | Docker    | 24.0+     | Application packaging                       |
| Orchestration| Docker Compose | 2.20+ | Multi-container deployment               |
| CI/CD       | GitHub Actions | —     | Build, test, deploy pipeline                |
| Monitoring   | Prometheus| 2.51+     | Metrics collection                          |
| Dashboards   | Grafana   | 10.4+     | Operational dashboards                      |
| Logging      | Loki      | 2.9+      | Centralized log aggregation                 |

---

## 8. Development Philosophy

### 8.1 Core Principles

| Principle                       | Meaning                                         |
|---------------------------------|-------------------------------------------------|
| Layered architecture            | Dependencies point inward; no circular deps      |
| Type safety everywhere          | Rust types, TypeScript types, Pydantic models    |
| Fail fast, fail loud            | Errors surface immediately, never silently       |
| Defensive programming           | Validate all inputs; never trust external data   |
| Separation of concerns          | Each module does one thing well                  |
| Test by default                 | Every feature has tests; no untested code        |
| Documentation as code           | Docs live with code; updated in same PR          |
| Security by design              | RBAC, validation, encryption from day one        |

### 8.2 Architecture Rules

| Rule                                         | Enforcement                                   |
|----------------------------------------------|-----------------------------------------------|
| Domain Layer has zero external dependencies  | Code review, architecture linting             |
| Services depend only on Domain + Infrastructure | No direct database access from handlers    |
| API Layer never contains business logic      | Code review                                   |
| All database access goes through repositories| SQLx queries only in repository layer          |
| All external calls use async I/O             | Tokio async/await throughout                   |
| All API endpoints require authentication     | Middleware enforced; public endpoints explicit  |
| All API responses use standard envelope      | JSON format consistent across all endpoints    |

### 8.3 When to Break Rules

| Situation                       | Guideline                                        |
|---------------------------------|-------------------------------------------------|
| Performance-critical path       | Document deviation; justify with benchmarks      |
| Prototyping / spike             | Mark with TODO; refactor before merge            |
| Third-party integration         | Wrap in adapter; isolate from core domain        |
| Emergency hotfix                | Fix the bug; follow up with proper fix           |

---

## 9. Local Development Environment

### 9.1 Setup Overview

```mermaid
graph LR
    A[Clone Repository] --> B[Install Dependencies]
    B --> C[Configure Environment]
    C --> D[Start Docker Services]
    D --> E[Run Database Migrations]
    E --> F[Start Development Servers]
    F --> G[Verify Setup]
```

### 9.2 Step-by-Step Setup

| Step | Action                                    | Verification                          |
|------|-------------------------------------------|---------------------------------------|
| 1    | Clone repository from Git                 | Directory exists, files present       |
| 2    | Install Rust toolchain (rustup)           | `rustc --version` returns 1.78+       |
| 3    | Install Python 3.11+                      | `python3 --version` returns 3.11+     |
| 4    | Install Node.js 18+                       | `node --version` returns 18+          |
| 5    | Install Docker Desktop                    | `docker --version` returns 24.0+      |
| 6    | Install Docker Compose                    | `docker compose version` returns 2.20+|
| 7    | Copy `.env.example` to `.env`             | `.env` file exists                    |
| 8    | Configure environment variables           | All required vars set                 |
| 9    | Run `docker compose up -d`                | All containers running                |
| 10   | Run database migrations                   | Database schema created               |
| 11   | Build backend: `cargo build`              | Binary compiled successfully          |
| 12   | Build AI service: `pip install -r requirements.txt` | Dependencies installed    |
| 13   | Build dashboard: `npm install`            | Dependencies installed                |
| 14   | Start development servers                 | Services accessible on localhost      |
| 15   | Run test suite: `cargo test`              | All tests pass                        |

### 9.3 Quick Start Commands

| Command                              | Purpose                                  |
|--------------------------------------|------------------------------------------|
| `make setup`                         | Full local setup (all steps above)       |
| `make dev`                           | Start all development servers            |
| `make dev-backend`                   | Start backend only                       |
| `make dev-ai`                        | Start AI service only                    |
| `make dev-dashboard`                 | Start dashboard only                     |
| `make test`                          | Run all tests                            |
| `make test-backend`                  | Run Rust tests                           |
| `make test-ai`                       | Run Python tests                         |
| `make test-dashboard`                | Run frontend tests                       |
| `make lint`                          | Run all linters                          |
| `make format`                        | Format all code                          |
| `make docker-up`                     | Start Docker services                    |
| `make docker-down`                   | Stop Docker services                     |
| `make migrate`                       | Run database migrations                  |
| `make clean`                         | Clean build artifacts                    |

---

## 10. Required Software

### 10.1 Development Tools

| Tool                     | Minimum Version | Purpose                              | Install Method           |
|--------------------------|-----------------|--------------------------------------|--------------------------|
| Rust toolchain           | 1.78+           | Backend compilation                  | rustup                   |
| Cargo                    | Latest          | Rust package manager                 | Included with Rust       |
| Python                   | 3.11+           | AI service runtime                   | pyenv / system package   |
| pip                      | Latest          | Python package manager               | Included with Python     |
| Node.js                  | 18+             | Dashboard runtime                    | nvm / system package     |
| npm                      | 9+              | JavaScript package manager           | Included with Node.js    |
| Docker Desktop           | 24.0+           | Container runtime                    | Docker website           |
| Docker Compose           | 2.20+           | Multi-container orchestration        | Included with Docker Desktop |
| Git                      | 2.40+           | Version control                      | System package           |

### 10.2 Optional Tools

| Tool                     | Purpose                              | When Needed          |
|--------------------------|--------------------------------------|----------------------|
| NVIDIA CUDA Toolkit      | GPU acceleration for AI inference    | GPU development      |
| pgAdmin / DBeaver        | Database GUI management              | Database debugging   |
| Postman / HTTPie         | API testing                          | Manual API testing   |
| Redis CLI                | Cache inspection                     | Cache debugging      |
| VS Code                  | IDE (recommended)                    | Daily development    |
| GitHub Copilot           | AI-assisted coding                   | Optional             |

---

## 11. Hardware Requirements

### 11.1 Minimum Development Machine

| Component    | Minimum                              | Recommended                           |
|--------------|--------------------------------------|---------------------------------------|
| CPU          | 4 cores (8 vCPU)                     | 8 cores (16 vCPU)                     |
| RAM          | 16 GB                                | 32 GB                                 |
| Storage      | 256 GB SSD                           | 512 GB NVMe SSD                       |
| GPU          | None (CPU inference)                 | NVIDIA T4 (16 GB) for AI development  |
| Network      | Broadband internet                   | Low-latency connection                |

### 11.2 Docker Resource Allocation

| Container              | CPU    | Memory   | Disk     |
|------------------------|--------|----------|----------|
| PostgreSQL             | 2      | 4 GB     | 20 GB    |
| Redis                  | 1      | 1 GB     | 2 GB     |
| Axum API               | 2      | 2 GB     | 5 GB     |
| AI Inference           | 4      | 8 GB     | 10 GB    |
| Camera Gateway         | 1      | 1 GB     | 2 GB     |
| Next.js Dashboard      | 1      | 1 GB     | 2 GB     |
| Prometheus             | 1      | 2 GB     | 10 GB    |
| Grafana                | 1      | 1 GB     | 5 GB     |
| **Total**              | **13** | **20 GB**| **56 GB**|

---

## 12. IDE Recommendations

### 12.1 Visual Studio Code (Recommended)

| Extension                     | Purpose                              |
|-------------------------------|--------------------------------------|
| rust-analyzer                 | Rust language server                 |
| Python                        | Python language support              |
| Pylance                       | Python type checking                 |
| ESLint                        | JavaScript/TypeScript linting        |
| Prettier                      | Code formatting                      |
| Tailwind CSS IntelliSense     | Tailwind CSS autocompletion          |
| Docker                        | Docker container management          |
| SQLTools                      | Database query editor                |
| Thunder Client                | API testing                          |
| GitLens                       | Git integration                      |
| Error Lens                    | Inline error display                 |
| Code Spell Checker             | Spell checking                      |

### 12.2 JetBrains IDEs

| IDE                 | Best For                    |
|---------------------|-----------------------------|
| RustRover           | Rust development            |
| PyCharm Professional | Python AI development       |
| WebStorm            | Frontend development        |

---

## 13. VS Code Configuration

### 13.1 Workspace Settings

Key configuration areas for the workspace:

| Setting                  | Value                                          |
|--------------------------|-------------------------------------------------|
| Default formatter        | rust-analyzer (Rust), Prettier (JS/TS)          |
| Format on save           | true                                            |
| Editor tab size          | 4 (Rust, Python), 2 (TypeScript)                |
| Files trim trailing whitespace | true                                     |
| Files insert final newline | true                                          |
| Search exclude           | node_modules, target, .git, __pycache__         |

### 13.2 Recommended Launch Configurations

| Configuration                   | Purpose                              |
|---------------------------------|--------------------------------------|
| Launch Rust API                 | Debug backend server                 |
| Launch AI Service               | Debug Python AI service              |
| Launch Next.js Dashboard        | Debug frontend with breakpoints      |
| Attach to Docker                | Debug containerized service          |
| Run All Tests                   | Execute full test suite              |

---

## 14. Environment Variables

### 14.1 Backend Environment Variables

| Variable                 | Required | Description                          | Example                              |
|--------------------------|----------|--------------------------------------|--------------------------------------|
| `DATABASE_URL`           | Yes      | Database connection string           | `postgres://user:pass@localhost:5432/vigilantai` |
| `REDIS_URL`              | Yes      | Redis connection string              | `redis://localhost:6379`             |
| `JWT_PRIVATE_KEY`        | Yes      | RSA private key for JWT signing      | Path to PEM file or inline key       |
| `JWT_PUBLIC_KEY`         | Yes      | RSA public key for JWT verification  | Path to PEM file or inline key       |
| `ENCRYPTION_KEY`         | Yes      | AES-256 key for evidence encryption  | 64-character hex string              |
| `CORS_ORIGINS`           | Yes      | Allowed CORS origins                 | `http://localhost:3000`              |
| `INTERNAL_API_KEY`       | Yes      | Service-to-service auth key          | Random 32+ character string          |
| `LOG_LEVEL`              | No       | Logging level                        | `info` (default), `debug`           |
| `EVIDENCE_DIR`           | Yes      | Evidence storage directory           | `/var/lib/vigilantai/evidence`       |
| `RUST_LOG`               | No       | Rust log filter                      | `vigilantai=debug,tower_http=debug`  |

### 14.2 AI Service Environment Variables

| Variable                 | Required | Description                          | Example                              |
|--------------------------|----------|--------------------------------------|--------------------------------------|
| `DATABASE_URL`           | Yes      | Database connection string           | `postgres://user:pass@localhost:5432/vigilantai` |
| `MODEL_PATH`             | Yes      | Path to YOLO model weights           | `/app/models/yolov8n.pt`            |
| `DEVICE`                 | No       | Inference device                     | `cpu` or `cuda:0`                   |
| `LOG_LEVEL`              | No       | Logging level                        | `info`                              |
| `INTERNAL_API_KEY`       | Yes      | Service-to-service auth key          | Must match backend key              |

### 14.3 Dashboard Environment Variables

| Variable                 | Required | Description                          | Example                              |
|--------------------------|----------|--------------------------------------|--------------------------------------|
| `NEXT_PUBLIC_API_URL`    | Yes      | Backend API URL                      | `http://localhost:8080`              |
| `NEXT_PUBLIC_WS_URL`     | Yes      | WebSocket URL                        | `ws://localhost:8080`                |

### 14.4 PostgreSQL Environment Variables

| Variable                 | Required | Description                          | Example                              |
|--------------------------|----------|--------------------------------------|--------------------------------------|
| `POSTGRES_DB`            | Yes      | Database name                        | `vigilantai`                         |
| `POSTGRES_USER`          | Yes      | Database user                        | `vigilantai`                         |
| `POSTGRES_PASSWORD`      | Yes      | Database password                    | `secure_password_here`               |

---

## 15. Configuration Strategy

### 15.1 Configuration Loading Order

| Priority | Source                     | Override Level          |
|----------|----------------------------|-------------------------|
| 1        | Environment variables      | Highest (runtime)       |
| 2        | `.env` file                | Development override    |
| 3        | Application defaults       | Lowest (built-in)       |

### 15.2 Configuration Files

| File                     | Service        | Purpose                        |
|--------------------------|----------------|--------------------------------|
| `.env`                   | All            | Local environment variables    |
| `.env.example`           | All            | Template for `.env`            |
| `backend/src/config.rs`  | Backend        | Config struct definition       |
| `ai-service/app/config.py` | AI Service  | Config class definition        |
| `next.config.js`         | Dashboard      | Next.js configuration          |
| `docker-compose.yml`     | All            | Container configuration        |

### 15.3 Configuration Best Practice

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Never commit `.env` files                    | Contains secrets                             |
| Always provide `.env.example`                | Documents required variables                 |
| Use typed config structs                     | Compile-time validation of config            |
| Fail on missing required config              | Prevent silent misconfiguration              |
| Log config on startup (redact secrets)       | Aid debugging without exposing secrets       |

---

## 16. Backend Development Guidelines

### 16.1 Architecture Layers

| Layer              | Responsibility                                    | File Location        |
|--------------------|---------------------------------------------------|----------------------|
| API Layer          | HTTP routing, request parsing, response formatting | `routes/`            |
| Service Layer      | Business logic, validation, transaction management | `services/`          |
| Domain Layer       | Entity definitions, error types, business rules    | `models/`            |
| Infrastructure     | Database access, file I/O, external service calls  | `repositories/`      |

### 16.2 Dependency Direction

```mermaid
graph LR
    API[API Layer] --> SVC[Service Layer]
    SVC --> DOM[Domain Layer]
    SVC --> INFRA[Infrastructure Layer]
    INFRA --> DOM
```

**Rule:** Dependencies always point inward. The Domain Layer has zero external dependencies.

### 16.3 Adding a New API Endpoint

| Step | Action                                    | Files Affected              |
|------|-------------------------------------------|-----------------------------|
| 1    | Define domain model (if new entity)       | `models/{entity}.rs`        |
| 2    | Define request/response types             | `routes/{module}.rs`        |
| 3    | Create repository (if new entity)         | `repositories/{entity}.rs`  |
| 4    | Create service method                     | `services/{module}.rs`      |
| 5    | Create route handler                      | `routes/{module}.rs`        |
| 6    | Register route in router                  | `routes/mod.rs`             |
| 7    | Add tests                                 | `tests/` directory          |
| 8    | Update API documentation                  | `docs/06-API-Specification.md` |

### 16.4 Adding a New Service Method

| Step | Action                                    | Guidelines                       |
|------|-------------------------------------------|----------------------------------|
| 1    | Define method signature                   | Async, returns `Result<T, AppError>` |
| 2    | Validate inputs                           | Use domain validation rules      |
| 3    | Call repository methods                   | Through trait interfaces          |
| 4    | Apply business logic                      | In service layer only             |
| 5    | Return domain model                       | Never expose database types       |
| 6    | Handle errors                             | Map to `AppError` variants        |
| 7    | Add audit log entry                       | For sensitive operations          |

### 16.5 Adding a New Repository

| Step | Action                                    | Guidelines                       |
|------|-------------------------------------------|----------------------------------|
| 1    | Define repository trait                   | In domain layer                   |
| 2    | Implement trait with SQLx                 | In infrastructure layer           |
| 3    | Use parameterized queries                 | Never string interpolation       |
| 4    | Return domain models                      | Map database rows to domain types |
| 5    | Handle database errors                    | Map to `AppError` variants        |
| 6    | Add connection pool                       | Injected via application state    |

---

## 17. Rust Project Organization

### 17.1 Module Structure

| Module                  | Purpose                                    |
|-------------------------|--------------------------------------------|
| `main.rs`               | Application entry point, server startup    |
| `app.rs`                | Application state construction             |
| `config.rs`             | Configuration loading and validation       |
| `routes/`               | HTTP route handlers                        |
| `middleware/`           | Tower middleware layers                    |
| `services/`             | Business logic implementations             |
| `repositories/`         | Database access implementations            |
| `models/`               | Domain entity definitions                  |
| `errors.rs`             | Application error types                    |

### 17.2 Error Handling Pattern

| Error Category          | Convention                                  |
|-------------------------|---------------------------------------------|
| Domain errors           | Define in `errors.rs` with `thiserror`      |
| Database errors         | Map to domain errors in repository layer    |
| Validation errors       | Return 400 with descriptive message         |
| Not found               | Return 404 with resource identifier         |
| Unauthorized            | Return 401                                  |
| Forbidden               | Return 403                                  |
| Internal errors         | Return 500, log full error, expose message  |

### 17.3 Async Programming Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Use `async fn` for all I/O handlers          | Non-blocking I/O throughout                   |
| Never block the async runtime                | Avoid `std::thread::sleep` in async context  |
| Use `tokio::spawn` for concurrent tasks      | Parallel execution where needed               |
| Use `tokio::select!` for concurrent waits    | Handle multiple async branches               |
| Use `?` operator for error propagation       | Clean error handling in async contexts        |
| Never use `.await` in non-async functions    | Compile error; architectural violation        |

### 17.4 Database Access Pattern

```mermaid
sequenceDiagram
    participant Handler
    participant Service
    participant Repo
    participant DB

    Handler->>Service: business_operation()
    Service->>Repo: find_by_id(id)
    Repo->>DB: SQLx query (parameterized)
    DB-->>Repo: Row result
    Repo-->>Service: Domain model
    Service->>Service: Apply business logic
    Service->>Repo: save(entity)
    Repo->>DB: SQLx insert/update
    DB-->>Repo: Result
    Repo-->>Service: Saved entity
    Service-->>Handler: Service response
```

### 17.5 SQLx Usage Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Use `sqlx::query!()` macro where possible    | Compile-time SQL validation                   |
| Never interpolate user input into SQL        | Prevent SQL injection                         |
| Use connection pool for all queries          | Efficient connection management                |
| Set query timeout (30 seconds default)       | Prevent long-running queries                  |
| Use transactions for multi-table operations  | Data consistency                              |
| Check `DATABASE_URL` at compile time         | Verify schema compatibility                   |

---

## 18. AI Service Development Guidelines

### 18.1 Service Architecture

| Component              | Responsibility                              |
|------------------------|---------------------------------------------|
| FastAPI application    | HTTP server, request routing                 |
| Detector               | YOLO model loading and inference             |
| Tracker                | Object tracking across frames                |
| Preprocessor           | Frame normalization and preparation          |
| Health endpoints       | Service liveness and readiness               |

### 18.2 Adding a New Detection Model

| Step | Action                                    | Guidelines                       |
|------|-------------------------------------------|----------------------------------|
| 1    | Add model weights to `models/` directory  | Version-controlled               |
| 2    | Create model loader in `services/`        | Validate on load                 |
| 3    | Add inference endpoint                    | Internal API only                |
| 4    | Add health check for model status         | Report loaded/degraded/unavailable |
| 5    | Add unit tests                            | Mock model for testing           |
| 6    | Add integration tests                     | Test with sample frames          |

### 18.3 Python Coding Standards

| Rule                                         | Tool                                   |
|----------------------------------------------|----------------------------------------|
| Type hints on all functions                  | Pylance / mypy                         |
| Docstrings on public functions               | Google style                           |
| Format with `ruff format`                    | Consistent formatting                  |
| Lint with `ruff check`                       | Code quality                           |
| Max function length                          | 50 lines (guideline)                   |
| Max file length                              | 500 lines (guideline)                  |

### 18.4 Inter-Service Communication

| Communication            | Protocol        | Auth Method          |
|--------------------------|-----------------|----------------------|
| Rust API ↔ AI Service    | HTTP (internal) | Internal API key     |
| AI Service → Database    | PostgreSQL      | SCRAM-SHA-256        |
| Rust API → Camera Gateway| HTTP (internal) | Internal API key     |

---

## 19. Frontend Development Guidelines

### 19.1 Component Architecture

| Pattern                   | Implementation                              |
|---------------------------|---------------------------------------------|
| Component structure       | Functional components with hooks             |
| State management          | React hooks + SWR for server state           |
| Styling                   | Tailwind CSS utility classes                 |
| Type safety               | TypeScript strict mode                       |
| API integration           | Custom hooks with fetch/SWR                  |
| Real-time updates         | WebSocket via custom hook                    |

### 19.2 Adding a New Dashboard Page

| Step | Action                                    | Files Affected              |
|------|-------------------------------------------|-----------------------------|
| 1    | Create route in `src/app/`                | `src/app/{page}/page.tsx`   |
| 2    | Define TypeScript types                   | `src/types/`                |
| 3    | Create API client function                | `src/lib/api.ts`            |
| 4    | Create custom hook                        | `src/hooks/`                |
| 5    | Build page components                     | `src/components/dashboard/` |
| 6    | Add to navigation                         | `src/components/ui/`        |
| 7    | Add tests                                 | `__tests__/`                |

### 19.3 Frontend Coding Standards

| Rule                                         | Tool                                   |
|----------------------------------------------|----------------------------------------|
| TypeScript strict mode                       | `tsconfig.json`                        |
| ESLint enabled                               | `.eslintrc.json`                       |
| Prettier formatting                          | `.prettierrc`                          |
| No `any` types                               | ESLint rule                            |
| Component files max 300 lines                | Code review guideline                  |
| One component per file                       | File organization                      |

### 19.4 API Integration Pattern

| Pattern                   | Implementation                              |
|---------------------------|---------------------------------------------|
| API base URL              | `NEXT_PUBLIC_API_URL` environment variable  |
| Authentication            | JWT in httpOnly cookie                      |
| Error handling            | Global error interceptor                    |
| Loading states            | SWR isLoading / isValidating                |
| Optimistic updates        | SWR mutate for instant UI feedback          |
| WebSocket                 | Custom hook with auto-reconnect             |

---

## 20. Database Development

### 20.1 Database Strategy

| Environment     | Database     | Purpose                                    |
|-----------------|--------------|--------------------------------------------|
| Development     | SQLite       | Zero-config local development              |
| Testing         | SQLite       | Fast, isolated test database               |
| Production      | PostgreSQL   | Enterprise features, replication, HA       |

### 20.2 Migration Strategy

| Rule                                         | Implementation                              |
|----------------------------------------------|---------------------------------------------|
| All schema changes via migrations            | `migrations/` directory, versioned           |
| Forward-only in production                   | No down migrations in production            |
| Backward-compatible migrations               | Old code must work with new schema           |
| Test migrations before merge                 | Run against test database                    |
| Migration naming                             | `{timestamp}_{description}.sql`             |

### 20.3 Migration Guidelines

| Practice                        | Guideline                                      |
|---------------------------------|------------------------------------------------|
| Adding a column                 | Add with DEFAULT; no NOT NULL without default   |
| Removing a column               | Deploy code that ignores column first           |
| Renaming a column               | Add new column → migrate data → remove old      |
| Adding an index                 | Use CONCURRENTLY in PostgreSQL                  |
| Removing an index               | Safe to remove directly                         |
| Adding a foreign key            | Verify data consistency first                   |
| Large data migrations           | Batch in chunks of 1000 rows                    |

### 20.4 Repository Layer Standards

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| One repository per entity                    | Clear ownership, single responsibility       |
| Return domain models, never database rows    | Decouple domain from persistence             |
| Use parameterized queries                    | Prevent SQL injection                        |
| Handle errors consistently                   | Map all DB errors to `AppError`              |
| Use connection pooling                       | Efficient resource management                 |
| Set query timeouts                           | Prevent long-running queries                  |
| Use transactions for multi-step operations   | Data consistency                              |

### 20.5 Repository Pattern

| Concept                | Convention                                  |
|------------------------|---------------------------------------------|
| Trait definition       | In domain layer (e.g., `UserRepository`)     |
| Implementation         | In infrastructure layer (e.g., `PgUserRepository`) |
| Method naming          | `find_by_id`, `find_by_email`, `create`, `update`, `delete` |
| Query construction     | SQLx query macros with bind parameters       |
| Result mapping         | `FromRow` derive or manual mapping           |

---

## 21. Service Layer Standards

### 21.1 Service Layer Responsibilities

| Responsibility          | Implementation                              |
|------------------------|---------------------------------------------|
| Business logic          | All domain rules applied here               |
| Validation              | Input validation before persistence          |
| Transaction management  | Multi-step operations in transactions       |
| Error mapping           | Database errors → domain errors             |
| Audit logging           | Record sensitive operations                  |
| Cache management        | Invalidate cache on data changes            |

### 21.2 Service Method Guidelines

| Guideline                        | Implementation                              |
|----------------------------------|---------------------------------------------|
| Method signature                  | `async fn method(&self, input) -> Result<T, AppError>` |
| Input validation                  | Validate before any I/O                     |
| Single responsibility             | One method does one thing                   |
| Idempotent where possible         | Safe to retry without side effects          |
| Return domain models              | Never return raw database types             |
| Handle all errors                 | Never use `.unwrap()` in service layer      |

### 21.3 Service Method Template

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Validate input parameters                 |
| 2    | Check authorization (site scope)          |
| 3    | Query existing state                      |
| 4    | Apply business rules                      |
| 5    | Persist changes                           |
| 6    | Invalidate related cache                  |
| 7    | Record audit log                          |
| 8    | Return result                             |

---

## 22. API Development Standards

### 22.1 Endpoint Naming Convention

| Pattern                          | Example                                   |
|----------------------------------|-------------------------------------------|
| `GET /api/v1/{resource}`         | `GET /api/v1/cameras` (list)              |
| `GET /api/v1/{resource}/{id}`    | `GET /api/v1/cameras/{id}` (get one)      |
| `POST /api/v1/{resource}`        | `POST /api/v1/cameras` (create)           |
| `PUT /api/v1/{resource}/{id}`    | `PUT /api/v1/cameras/{id}` (full update)  |
| `PATCH /api/v1/{resource}/{id}`  | `PATCH /api/v1/cameras/{id}` (partial)    |
| `DELETE /api/v1/{resource}/{id}` | `DELETE /api/v1/cameras/{id}` (delete)    |

### 22.2 Response Envelope

All API responses use a standard envelope format:

| Field       | Type     | Description                              |
|-------------|----------|------------------------------------------|
| `data`      | Object   | Response payload (on success)            |
| `error`     | Object   | Error details (on failure)               |
| `meta`      | Object   | Pagination, metadata                     |
| `request_id`| String   | Unique request identifier                |

### 22.3 HTTP Status Codes

| Code   | Usage                                              |
|--------|----------------------------------------------------|
| 200    | Success (GET, PUT, PATCH, DELETE)                   |
| 201    | Created (POST)                                     |
| 204    | No content (DELETE with no response body)           |
| 400    | Bad request (validation error)                     |
| 401    | Unauthorized (missing/invalid JWT)                  |
| 403    | Forbidden (insufficient permissions)               |
| 404    | Not found (resource doesn't exist)                 |
| 409    | Conflict (duplicate resource)                      |
| 422    | Unprocessable entity (business rule violation)     |
| 429    | Too many requests (rate limited)                   |
| 500    | Internal server error                              |

### 22.4 Request Validation

| Validation Type       | Implementation                                    |
|-----------------------|---------------------------------------------------|
| Body validation       | Serde deserialize with validation                  |
| Query validation      | Axum Query extractor with validation               |
| Path validation       | Validated UUID, non-empty strings                  |
| Content-Type          | Enforced per endpoint                              |
| Content-Length        | Max 10 MB JSON, 100 MB file upload                |

---

## 23. Request Validation Standards

### 23.1 Validation Rules

| Rule                          | Implementation                                    |
|-------------------------------|---------------------------------------------------|
| Required fields               | Reject if missing                                 |
| String length                 | Min/max bounds per field                          |
| Email format                  | RFC 5322 validation                               |
| UUID format                   | Valid UUID v4                                      |
| Numeric ranges                | Min/max values                                    |
| Enum values                   | Only allowed values accepted                      |
| Nested objects                | Recursively validated                             |
| Array bounds                  | Min/max length                                    |

### 23.2 Validation Error Response

| Field       | Type     | Description                              |
|-------------|----------|------------------------------------------|
| `error`     | String   | "Validation Error"                       |
| `details`   | Array    | List of field-level errors               |
| `details[].field` | String | Field name with error             |
| `details[].message`| String | Error description                 |

---

## 24. Response Standards

### 24.1 Success Response Format

| Field       | Type     | Description                              |
|-------------|----------|------------------------------------------|
| `data`      | Object   | Response payload                         |
| `meta`      | Object   | Pagination info (for lists)              |
| `request_id`| String   | Request correlation ID                   |

### 24.2 Error Response Format

| Field       | Type     | Description                              |
|-------------|----------|------------------------------------------|
| `error`     | String   | Error category                           |
| `message`   | String   | Human-readable description               |
| `details`   | Array    | Additional error details                 |
| `request_id`| String   | Request correlation ID                   |

### 24.3 Pagination Response

| Field       | Type     | Description                              |
|-------------|----------|------------------------------------------|
| `data`      | Array    | List of items                            |
| `meta.page` | Integer  | Current page number                      |
| `meta.per_page` | Integer | Items per page                       |
| `meta.total`| Integer  | Total items matching query               |
| `meta.total_pages` | Integer | Total pages                        |

---

## 25. Error Handling Standards

### 25.1 Error Categories

| Category              | HTTP Code | When to Use                                  |
|-----------------------|-----------|----------------------------------------------|
| Validation Error      | 400/422   | Invalid request data                         |
| Authentication Error  | 401       | Missing, expired, or invalid JWT             |
| Authorization Error   | 403       | Valid JWT but insufficient permissions       |
| Not Found             | 404       | Resource doesn't exist                       |
| Conflict              | 409       | Duplicate resource creation                  |
| Rate Limited          | 429       | Too many requests                            |
| Internal Error        | 500       | Unexpected server failure                    |

### 25.2 Error Handling Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Never expose internal error details          | Security: prevent information leakage        |
| Log full error server-side                   | Aid debugging                                |
| Return generic message to client             | Security best practice                       |
| Include request ID in all errors             | Enable correlation for support               |
| Map all errors to `AppError`                 | Consistent error handling throughout         |
| Never use `unwrap()` or `expect()` in production code | Prevent panics                |

### 25.3 AppError Variants

| Variant                 | HTTP Code | Usage                                        |
|-------------------------|-----------|----------------------------------------------|
| `BadRequest`            | 400       | Invalid input                                |
| `Unauthorized`          | 401       | Authentication failure                       |
| `Forbidden`             | 403       | Authorization failure                        |
| `NotFound`              | 404       | Resource not found                           |
| `Conflict`              | 409       | Resource already exists                      |
| `UnprocessableEntity`   | 422       | Business rule violation                      |
| `TooManyRequests`       | 429       | Rate limit exceeded                          |
| `InternalServerError`   | 500       | Unexpected error                             |
| `DatabaseError`         | 500       | Database operation failed                    |
| `ExternalServiceError`  | 502       | External service unavailable                 |

---

## 26. Logging Standards

### 26.1 Structured Logging Format

All logs use JSON-structured format:

```json
{
  "timestamp": "2026-07-21T10:30:00.000Z",
  "level": "info",
  "service": "vigilantai-api",
  "module": "event_service",
  "request_id": "req-abc-123",
  "user_id": "uuid-of-user",
  "message": "Event processed successfully",
  "event_id": "evt-789",
  "duration_ms": 45
}
```

### 26.2 Log Levels

| Level     | Usage                                              |
|-----------|----------------------------------------------------|
| `error`   | Unexpected failures requiring attention             |
| `warn`    | Degraded conditions, recoverable failures           |
| `info`    | Significant business events, state changes          |
| `debug`   | Detailed operational information                    |
| `trace`   | Extremely detailed, debugging only                  |

### 26.3 Logging Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Never log secrets or credentials             | Security: prevent credential leakage         |
| Never log full request bodies                | May contain sensitive data                   |
| Log structured JSON only                     | Machine-parseable, queryable                 |
| Include request_id in all request logs       | Enable correlation across services           |
| Log business events at info level            | Operational visibility                       |
| Log errors with full context                 | Aid debugging                                |
| Use tracing spans for critical paths         | Enable distributed tracing                   |

### 26.4 What to Log

| Event                        | Level   | Data to Include                            |
|------------------------------|---------|--------------------------------------------|
| Request received              | info    | method, path, user_id, request_id          |
| Request completed             | info    | status, duration_ms, request_id            |
| Authentication success        | info    | user_id, ip_address                        |
| Authentication failure        | warn    | email, ip_address, reason                  |
| Business operation            | info    | operation, entity_id, user_id              |
| Database error                | error   | query, error, duration_ms                  |
| External service call         | debug   | service, method, duration_ms, status       |
| Security event                | warn    | event_type, user_id, details               |

---

## 27. Authentication Integration

### 27.1 JWT Token Flow

| Step | Action                                    | Location               |
|------|-------------------------------------------|------------------------|
| 1    | Client sends credentials to `/auth/login` | Route handler          |
| 2    | Service validates credentials             | `auth_service.rs`      |
| 3    | Service generates access token (15 min)   | `auth_service.rs`      |
| 4    | Service generates refresh token (7 days)  | `auth_service.rs`      |
| 5    | Access token returned in response body    | Route handler          |
| 6    | Refresh token set in httpOnly cookie      | Route handler          |
| 7    | Client includes JWT in Authorization header| All API requests      |
| 8    | Middleware validates JWT signature         | `middleware/auth.rs`   |
| 9    | Middleware extracts claims                | `middleware/auth.rs`   |

### 27.2 JWT Claims Structure

| Claim    | Type     | Description                              |
|----------|----------|------------------------------------------|
| `sub`    | UUID     | User ID                                  |
| `email`  | String   | User email                               |
| `roles`  | Array    | Assigned role names                       |
| `sites`  | Array    | Assigned site IDs                         |
| `permissions`| Array| Resolved permission strings               |
| `iss`    | String   | Token issuer                             |
| `aud`    | String   | Token audience                           |
| `iat`    | Integer  | Issued-at timestamp                      |
| `exp`    | Integer  | Expiration timestamp                     |
| `jti`    | UUID     | Token ID (for revocation)                |

### 27.3 Authentication Middleware

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Extract JWT from Authorization header     |
| 2    | Verify JWT signature with public key      |
| 3    | Check token expiry                        |
| 4    | Check token not revoked (Redis blocked list) |
| 5    | Extract claims from token                 |
| 6    | Attach claims to request extensions       |
| 7    | Pass to next middleware/handler            |

---

## 28. Authorization Integration

### 28.1 RBAC Middleware

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Extract user claims from request          |
| 2    | Extract required permission from route    |
| 3    | Check cache for user's permissions        |
| 4    | If cache miss, query database             |
| 5    | Check if user has required permission     |
| 6    | If denied, return 403 Forbidden           |
| 7    | If allowed, check site scope              |
| 8    | Attach site scope to request              |

### 28.2 Site Scope Enforcement

| Operation                    | Enforcement                                 |
|------------------------------|---------------------------------------------|
| List resources               | Filter by user's assigned sites             |
| Get single resource          | Verify resource belongs to assigned site    |
| Create resource              | Validate site_id in user's assigned sites   |
| Update resource              | Verify resource belongs to assigned site    |
| Delete resource              | Verify resource belongs to assigned site    |
| Dashboard aggregation        | Filter data to assigned sites only          |

---

## 29. RBAC Development Rules

### 29.1 Role Hierarchy

| Role                | Scope Level         | Description                          |
|---------------------|---------------------|--------------------------------------|
| `system_admin`      | All sites           | Full platform administration         |
| `security_admin`    | All sites           | Security operations management       |
| `security_analyst`  | Assigned sites      | Alert monitoring, investigation      |
| `operator`          | Assigned sites      | Dashboard monitoring, rule management|
| `viewer`            | Assigned sites      | Read-only access                     |
| `api_integration`   | Assigned sites      | API access for integrations          |

### 29.2 Permission Naming Convention

| Pattern                          | Example                                   |
|----------------------------------|-------------------------------------------|
| `{resource}.{action}`            | `cameras.read`, `evidence.delete`         |
| `{resource}.{specific_action}`   | `alerts.acknowledge`, `incidents.close`   |

### 29.3 Adding a New Permission

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Add permission to permissions table       |
| 2    | Map permission to relevant roles          |
| 3    | Add permission to JWT claims              |
| 4    | Add RBAC check to route                   |
| 5    | Add permission to frontend permission cache|
| 6    | Test with each role                       |

---

## 30. Evidence Handling Standards

### 30.1 Evidence Lifecycle

| Step | Action                                    | Location               |
|------|-------------------------------------------|------------------------|
| 1    | Detection event triggers evidence creation | `event_service.rs`     |
| 2    | Camera Gateway captures frame             | Camera Gateway         |
| 3    | AI validates detection                    | AI Service             |
| 4    | Frame + metadata sent to API              | Internal API           |
| 5    | API creates evidence record               | `evidence_service.rs`  |
| 6    | SHA-256 hash computed                     | `evidence_service.rs`  |
| 7    | Evidence stored in file system            | Evidence Storage       |
| 8    | Chain of custody recorded                 | `audit_service.rs`     |

### 30.2 Evidence Integrity Rules

| Rule                                         | Implementation                              |
|----------------------------------------------|---------------------------------------------|
| Hash on creation                             | SHA-256 of file content                     |
| Verify on every access                       | Hash comparison before serving              |
| Tamper detection                             | Mismatch triggers alert + denial            |
| Append-only access log                       | Never delete access records                 |
| File naming convention                       | `{site_id}/{date}/{uuid}.{ext}`             |

### 30.3 Evidence File Rules

| Rule                                         | Value                                       |
|----------------------------------------------|---------------------------------------------|
| Max file size                                | 10 MB per clip                              |
| Allowed formats                              | JPEG, PNG, MP4                              |
| File permissions                             | 0644 (read-only)                            |
| Directory permissions                        | 0755                                        |
| Storage path                                 | Configured via `EVIDENCE_DIR` env var       |

---

## 31. AI Integration Workflow

### 31.1 AI Service Communication

| Communication            | Direction        | Protocol        | Auth                    |
|--------------------------|------------------|-----------------|-------------------------|
| Camera Gateway → AI      | Push frames      | HTTP POST       | Internal API key        |
| AI → API (detections)    | Push results     | HTTP POST       | Internal API key        |
| API → AI (health check)  | Query status     | GET             | Internal API key        |

### 31.2 Detection Pipeline

| Step | Action                                    | Component            |
|------|-------------------------------------------|----------------------|
| 1    | Frame received from Camera Gateway        | AI Service           |
| 2    | Frame preprocessed (resize, normalize)    | Preprocessor         |
| 3    | YOLO inference executed                   | Detector             |
| 4    | Detections post-processed (NMS, filter)   | Detector             |
| 5    | Objects tracked across frames             | Tracker              |
| 6    | Detection results returned                | AI Service           |

### 31.3 Adding a New Detection Type

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Train or select model for new detection   |
| 2    | Add model weights to `models/` directory  |
| 3    | Add model loader to detector service      |
| 4    | Add post-processing for new detection type|
| 5    | Add detection type to domain model        |
| 6    | Add rule support for new detection type   |
| 7    | Add tests with sample data                |
| 8    | Update documentation                      |

---

## 32. Dependency Management

### 32.1 Rust Dependencies (Cargo.toml)

| Category              | Recommended Packages                          |
|-----------------------|-----------------------------------------------|
| Web framework         | axum, tower, tower-http                       |
| Async runtime         | tokio (full features)                         |
| Database              | sqlx (postgres, runtime-tokio, tls-rustls)   |
| Serialization         | serde, serde_json                             |
| Error handling        | thiserror, anyhow                             |
| Logging               | tracing, tracing-subscriber                   |
| Authentication        | jsonwebtoken                                  |
| Password hashing      | argon2, bcrypt                                |
| HTTP client           | reqwest                                       |
| WebSocket             | axum (ws feature)                             |
| Validation            | validator                                     |
| UUID                  | uuid (v4 feature)                             |
| Time                  | chrono                                        |

### 32.2 Python Dependencies (requirements.txt)

| Category              | Recommended Packages                          |
|-----------------------|-----------------------------------------------|
| Web framework         | fastapi, uvicorn                              |
| Validation            | pydantic                                      |
| Computer vision       | opencv-python                                 |
| Object detection      | ultralytics (YOLO)                            |
| HTTP client           | httpx                                         |
| Logging               | structlog                                     |
| Testing               | pytest, pytest-asyncio                        |
| Linting               | ruff                                          |

### 32.3 Adding New Dependencies

| Step | Action                                    | Guidelines                       |
|------|-------------------------------------------|----------------------------------|
| 1    | Research alternatives                     | Compare maintenance, size, license|
| 2    | Check existing dependencies               | Avoid duplicates                 |
| 3    | Review license compatibility              | Apache 2.0, MIT, BSD preferred   |
| 4    | Check security vulnerabilities            | `cargo audit`, `pip-audit`       |
| 5    | Add to dependency file                    | Pin version for reproducibility  |
| 6    | Test build and all tests                  | No regressions                   |
| 7    | Document in PR description                | Justify the addition             |

---

## 33. Package Version Strategy

### 33.1 Semantic Versioning

| Version Component | When to Increment                          |
|-------------------|--------------------------------------------|
| Major (X.0.0)    | Breaking API changes, major feature additions|
| Minor (0.X.0)    | New features, backward-compatible           |
| Patch (0.0.X)     | Bug fixes, security patches                |

### 33.2 Dependency Pinning

| Ecosystem     | Strategy                                      |
|---------------|-----------------------------------------------|
| Rust          | Use `=` for critical deps; `^` for others     |
| Python        | Pin exact versions in `requirements.txt`       |
| npm           | Use `^` for minor versions; lock with `package-lock.json` |
| Docker images | Pin to specific digest for production          |

---

## 34. Git Workflow

### 34.1 Branching Strategy

```mermaid
graph TB
    MAIN[main] -->|feature branch| FEATURE[feature/*]
    MAIN -->|bugfix branch| BUGFIX[bugfix/*]
    MAIN -->|hotfix branch| HOTFIX[hotfix/*]
    FEATURE -->|PR + review| MAIN
    BUGFIX -->|PR + review| MAIN
    HOTFIX -->|PR + review| MAIN
    MAIN -->|tag| RELEASE[release/*]
```

### 34.2 Branch Naming Convention

| Pattern                          | Example                                   |
|----------------------------------|-------------------------------------------|
| `feature/{ticket}-{description}` | `feature/VA-123-add-evidence-upload`      |
| `bugfix/{ticket}-{description}`  | `bugfix/VA-456-fix-websocket-reconnect`   |
| `hotfix/{ticket}-{description}`  | `hotfix/VA-789-fix-auth-bypass`           |
| `release/{version}`             | `release/1.2.0`                           |

### 34.3 Branch Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| `main` is always deployable                  | Production-ready at all times                 |
| Feature branches from `main`                 | Isolate new work                              |
| Never force-push `main`                      | Preserve history                             |
| Delete branches after merge                  | Keep repository clean                         |
| All changes via Pull Request                 | Code review required                          |

---

## 35. Commit Message Convention

### 35.1 Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

### 35.2 Types

| Type        | Usage                                              |
|-------------|----------------------------------------------------|
| `feat`      | New feature                                        |
| `fix`       | Bug fix                                            |
| `docs`      | Documentation changes                              |
| `style`     | Code style changes (formatting, no logic change)   |
| `refactor`  | Code refactoring (no feature change)               |
| `test`      | Adding or updating tests                           |
| `chore`     | Build process, dependency updates                  |
| `perf`      | Performance improvement                            |
| `ci`        | CI/CD changes                                      |
| `revert`    | Reverting a previous commit                        |

### 35.3 Scope

| Scope         | Meaning                                      |
|---------------|----------------------------------------------|
| `api`         | Backend API changes                          |
| `ai`          | AI service changes                           |
| `dashboard`   | Frontend changes                             |
| `gateway`     | Camera gateway changes                       |
| `db`          | Database changes                             |
| `auth`        | Authentication/authorization changes         |
| `evidence`    | Evidence handling changes                    |
| `infra`       | Infrastructure changes                       |

### 35.4 Examples

| Commit Message                                          |
|---------------------------------------------------------|
| `feat(api): add evidence upload endpoint`              |
| `fix(auth): prevent token reuse after logout`          |
| `docs(api): update evidence API documentation`         |
| `refactor(services): extract rule evaluation logic`    |
| `test(api): add integration tests for camera CRUD`     |
| `chore(deps): update sqlx to 0.8.1`                    |
| `perf(db): add index on detection_events.created_at`   |

---

## 36. Pull Request Guidelines

### 36.1 PR Template

| Field                           | Required | Description                              |
|---------------------------------|----------|------------------------------------------|
| Title                           | Yes      | Follows commit message convention         |
| Description                     | Yes      | What changed and why                     |
| Type of change                  | Yes      | feat, fix, docs, refactor, test, chore   |
| Related issue                   | Yes      | Link to ticket/issue                     |
| Testing done                    | Yes      | Describe tests run                       |
| Screenshots (if UI change)      | No       | Visual verification                      |
| Breaking changes                | Yes      | Document any breaking changes            |
| Deployment notes                | No       | Special deployment considerations        |

### 36.2 PR Quality Checklist

| Check                                          | Required |
|------------------------------------------------|----------|
| All tests passing                              | Yes      |
| Linter clean (no warnings)                     | Yes      |
| Type checking passing                          | Yes      |
| Code review approved (1+ reviewers)            | Yes      |
| No merge conflicts                             | Yes      |
| Documentation updated                          | Yes      |
| No hardcoded secrets                           | Yes      |
| No `unwrap()` or `expect()` in production code | Yes      |
| Database migrations tested                     | Yes      |
| Performance impact assessed                    | Yes      |

### 36.3 PR Size Guidelines

| Size        | Lines Changed | Review Time  | Recommendation           |
|-------------|---------------|--------------|--------------------------|
| Small       | < 100         | < 30 min     | Ideal                    |
| Medium      | 100-500       | 1-2 hours    | Acceptable               |
| Large       | 500-1000      | Half day     | Split if possible        |
| Extra Large | > 1000        | Full day     | Must split               |

---

## 37. Code Review Checklist

### 37.1 Correctness

| Check                                          | Priority |
|------------------------------------------------|----------|
| Logic is correct                               | High     |
| Edge cases handled                             | High     |
| Error handling is comprehensive                | High     |
| No silent failures                             | High     |
| Database queries are correct                   | High     |
| Business rules are implemented correctly       | High     |

### 37.2 Security

| Check                                          | Priority |
|------------------------------------------------|----------|
| Input validation present                       | High     |
| SQL injection prevented (parameterized)        | High     |
| No secrets in code                             | High     |
| Authentication required                        | High     |
| Authorization checked                          | High     |
| Site scope enforced                            | High     |
| No sensitive data in logs                      | High     |

### 37.3 Performance

| Check                                          | Priority |
|------------------------------------------------|----------|
| No N+1 queries                                 | Medium   |
| Appropriate indexes used                       | Medium   |
| No unnecessary database calls                  | Medium   |
| Connection pool not exhausted                  | Medium   |
| Caching used where appropriate                 | Low      |

### 37.4 Code Quality

| Check                                          | Priority |
|------------------------------------------------|----------|
| Follows naming conventions                     | Medium   |
| Functions are focused and small                | Medium   |
| No code duplication                            | Medium   |
| Error messages are descriptive                 | Medium   |
| Code is self-documenting                       | Low      |
| Comments explain why, not what                 | Low      |

### 37.5 Testing

| Check                                          | Priority |
|------------------------------------------------|----------|
| Unit tests for new logic                       | High     |
| Integration tests for new endpoints            | High     |
| Edge cases tested                              | Medium   |
| Error paths tested                             | Medium   |
| Tests are readable and maintainable            | Medium   |

---

## 38. Coding Standards

### 38.1 Rust Coding Standards

| Standard                     | Convention                                  |
|------------------------------|---------------------------------------------|
| Formatting                   | `cargo fmt` (default settings)              |
| Linting                      | `cargo clippy` (all warnings)               |
| Line length                  | 100 characters max                          |
| Function length              | 50 lines max (guideline)                    |
| File length                  | 500 lines max (guideline)                   |
| Module length                | 1000 lines max (guideline)                  |
| Error handling               | `?` operator, no `unwrap()` in production   |
| Naming                       | `snake_case` for variables/functions        |
| Types                        | `PascalCase` for types/traits/enums         |
| Constants                    | `SCREAMING_SNAKE_CASE`                      |
| Imports                      | Grouped: std, external, internal             |
| Doc comments                 | `///` for public items                       |

### 38.2 Python Coding Standards

| Standard                     | Convention                                  |
|------------------------------|---------------------------------------------|
| Formatting                   | `ruff format`                               |
| Linting                      | `ruff check`                                |
| Line length                  | 88 characters max                           |
| Function length              | 50 lines max (guideline)                    |
| File length                  | 500 lines max (guideline)                   |
| Type hints                   | Required on all public functions            |
| Docstrings                   | Google style on public functions            |
| Naming                       | `snake_case` for functions/variables        |
| Classes                      | `PascalCase`                                |
| Constants                    | `SCREAMING_SNAKE_CASE`                      |
| Imports                      | Sorted by isort rules                       |

### 38.3 TypeScript Coding Standards

| Standard                     | Convention                                  |
|------------------------------|---------------------------------------------|
| Formatting                   | Prettier                                    |
| Linting                      | ESLint                                      |
| Line length                  | 100 characters max                          |
| Component length             | 300 lines max (guideline)                   |
| File length                  | 500 lines max (guideline)                   |
| Type annotations             | Required on all exports                     |
| Naming                       | `camelCase` for functions/variables         |
| Components                   | `PascalCase`                                |
| Types/Interfaces             | `PascalCase`                                |
| Constants                    | `SCREAMING_SNAKE_CASE` or `camelCase`       |
| Imports                      | Grouped: external, internal, types          |

---

## 39. Naming Conventions

### 39.1 General Rules

| Element                  | Rust Convention       | Python Convention    | TypeScript Convention |
|--------------------------|----------------------|---------------------|----------------------|
| Functions/methods        | `snake_case`         | `snake_case`        | `camelCase`          |
| Variables                | `snake_case`         | `snake_case`        | `camelCase`          |
| Types/Classes            | `PascalCase`         | `PascalCase`        | `PascalCase`         |
| Constants                | `SCREAMING_SNAKE_CASE`| `SCREAMING_SNAKE_CASE`| `SCREAMING_SNAKE_CASE`|
| File names               | `snake_case.rs`      | `snake_case.py`     | `kebab-case.tsx`     |
| Database tables          | `snake_case` (plural)| `snake_case` (plural)| `snake_case` (plural)|
| Database columns         | `snake_case`         | `snake_case`        | `snake_case`         |
| API endpoints            | `kebab-case`         | `kebab-case`        | `kebab-case`         |
| Environment variables    | `SCREAMING_SNAKE_CASE`| `SCREAMING_SNAKE_CASE`| `SCREAMING_SNAKE_CASE`|

### 39.2 Domain-Specific Naming

| Element                  | Convention                                  |
|--------------------------|---------------------------------------------|
| Repository trait         | `{Entity}Repository`                        |
| Repository impl          | `Pg{Entity}Repository` (PostgreSQL)         |
| Service                  | `{Entity}Service`                           |
| Route handler            | `{action}_{entity}` (e.g., `get_cameras`)  |
| Error type               | `{Entity}Error` or `AppError`              |
| Request body             | `Create{Entity}Request`, `Update{Entity}Request` |
| Response body            | `{Entity}Response`                          |
| JWT claims               | `Claims`                                    |
| Application state        | `AppState`                                  |

---

## 40. Folder Organization

### 40.1 File Placement Rules

| What                          | Where                                       |
|-------------------------------|---------------------------------------------|
| Domain models                 | `models/`                                   |
| Business logic                | `services/`                                 |
| Database queries              | `repositories/`                             |
| HTTP route handlers           | `routes/`                                   |
| Middleware                     | `middleware/`                                |
| Configuration                 | `config.rs` / `config.py`                   |
| Error types                   | `errors.rs`                                 |
| Database migrations           | `migrations/`                               |
| Tests                         | `tests/` directory or `*_test.rs` / `test_*.py` |
| Documentation                 | `docs/`                                     |
| Scripts                       | `scripts/`                                  |
| Deployment configs            | `deploy/`                                   |

### 40.2 Module Coherence

| Principle                     | Guideline                                   |
|-------------------------------|---------------------------------------------|
| Related code together         | Keep related files in same module/directory  |
| One responsibility per file   | Single class/function per file where logical |
| Clear module boundaries       | Modules have well-defined public interfaces  |
| Avoid circular dependencies   | Use dependency inversion                     |

---

## 41. Documentation Standards

### 41.1 Documentation Types

| Type                         | Location          | Tool                     |
|------------------------------|-------------------|--------------------------|
| Architecture documents       | `docs/01-08`      | Markdown                 |
| API documentation            | `docs/06`         | Markdown (OpenAPI later) |
| Code documentation           | In source files   | Doc comments (///, """)  |
| README files                 | Directory root    | Markdown                 |
| Changelog                    | `CHANGELOG.md`    | Markdown                 |
| Contributing guide           | `CONTRIBUTING.md` | Markdown                 |

### 41.2 Code Documentation Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Document all public functions                | Enable API understanding                     |
| Explain why, not what                        | Code is self-explanatory                     |
| Include examples for complex functions       | Aid comprehension                            |
| Document panics and safety requirements      | Prevent misuse                               |
| Keep documentation close to code             | Reduce staleness                             |

### 41.3 README Requirements

| Section                    | Required | Content                                  |
|----------------------------|----------|------------------------------------------|
| Project description        | Yes      | What the project is                      |
| Prerequisites              | Yes      | Required software                        |
| Setup instructions         | Yes      | Step-by-step local setup                 |
| Usage                      | Yes      | How to run the project                   |
| Contributing               | Yes      | How to contribute                        |
| License                    | Yes      | Project license                          |

---

## 42. Testing Philosophy

### 42.1 Testing Pyramid

```mermaid
graph TB
    E2E[End-to-End Tests] --> INT[Integration Tests]
    INT --> UNIT[Unit Tests]

    UNIT -.->|Many| UNIT_COUNT[Fast, Isolated, Cheap]
    INT -.->|Some| INT_COUNT[Realistic, Moderate Speed]
    E2E -.->|Few| E2E_COUNT[Slow, Comprehensive]
```

### 42.2 Testing Principles

| Principle                     | Implementation                              |
|-------------------------------|---------------------------------------------|
| Test by default               | Every feature has tests                     |
| Tests are code                | Same standards as production code           |
| Tests are deterministic       | Same input → same output, always            |
| Tests are isolated            | No shared state between tests               |
| Tests are fast                | Unit tests < 100ms, integration < 5s        |
| Tests are readable            | Clear naming, clear assertions              |
| Tests cover edge cases        | Boundary conditions, error paths            |

### 42.3 Testing Targets

| Metric                        | Target                                      |
|-------------------------------|---------------------------------------------|
| Unit test coverage            | > 80%                                        |
| Integration test coverage     | > 60% of API endpoints                       |
| Critical path coverage        | 100%                                         |
| Test execution time (unit)    | < 30 seconds total                           |
| Test execution time (integration)| < 5 minutes total                         |

---

## 43. Unit Testing Guidelines

### 43.1 Rust Unit Tests

| Guideline                        | Implementation                              |
|----------------------------------|---------------------------------------------|
| Location                         | `#[cfg(test)]` module in same file          |
| Naming                           | `test_{function}_{scenario}`                |
| Arrange-Act-Assert               | Clear test structure                        |
| Mock external dependencies       | Use trait objects or mockall                 |
| Test both success and error paths| Cover happy and unhappy paths               |
| Edge cases                       | Empty inputs, boundary values, max limits   |

### 43.2 Python Unit Tests

| Guideline                        | Implementation                              |
|----------------------------------|---------------------------------------------|
| Location                         | `tests/` directory, mirrors source structure |
| Naming                           | `test_{function}_{scenario}`                |
| Framework                        | pytest with fixtures                         |
| Mocking                          | unittest.mock or pytest-mock                 |
| Async testing                    | pytest-asyncio                               |
| Coverage                         | pytest-cov                                   |

### 43.3 Unit Test Structure

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Arrange: Set up test data and mocks       |
| 2    | Act: Call the function under test         |
| 3    | Assert: Verify expected outcome           |
| 4    | Cleanup: No cleanup needed (isolated)     |

---

## 44. Integration Testing Guidelines

### 44.1 Backend Integration Tests

| Guideline                        | Implementation                              |
|----------------------------------|---------------------------------------------|
| Database                         | Test database (SQLite in-memory or test PG) |
| External services                | Mock or test instances                       |
| API endpoints                    | Full HTTP request/response cycle            |
| Authentication                   | Generate test JWTs                           |
| Authorization                    | Test with different roles                   |
| Site scope                       | Test with different site assignments        |

### 44.2 API Integration Test Matrix

| Endpoint Category   | Tests Required                              |
|---------------------|---------------------------------------------|
| CRUD endpoints      | Create, Read, Update, Delete, List           |
| Authentication      | Login, refresh, logout, invalid credentials  |
| Authorization       | Each role × each endpoint                   |
| Site scope          | Access own site, access other site (denied)  |
| Validation          | Invalid input, missing fields, wrong types   |
| Pagination          | First page, middle page, last page           |
| Filtering           | Each filter, combined filters                |
| Error paths         | Not found, conflict, unauthorized            |

### 44.3 AI Integration Tests

| Test Type                      | Purpose                                      |
|--------------------------------|----------------------------------------------|
| Model loading                  | Verify model loads correctly                 |
| Single frame inference         | Verify detection on known image              |
| Batch inference                | Verify throughput under load                 |
| GPU fallback                   | Verify CPU fallback when GPU unavailable     |
| Model hot-swap                 | Verify model update without restart          |
| Error handling                 | Verify graceful handling of bad input        |

---

## 45. API Testing Guidelines

### 45.1 Manual API Testing

| Tool                  | When to Use                                    |
|-----------------------|------------------------------------------------|
| Thunder Client (VS Code)| Quick endpoint verification                 |
| HTTPie                | Command-line API testing                        |
| curl                  | Quick one-off tests                             |
| Postman               | Collaborative API testing                       |

### 45.2 Automated API Testing

| Tool                  | When to Use                                    |
|-----------------------|------------------------------------------------|
| cargo test (Rust)     | Backend integration tests                       |
| pytest (Python)       | AI service integration tests                    |
| vitest (TypeScript)   | Frontend API client tests                       |

### 45.3 API Test Checklist

| Check                                          | Required |
|------------------------------------------------|----------|
| Happy path (200/201)                           | Yes      |
| Validation errors (400/422)                    | Yes      |
| Authentication required (401)                  | Yes      |
| Authorization enforced (403)                   | Yes      |
| Not found (404)                                | Yes      |
| Rate limiting (429)                            | Yes      |
| Pagination                                     | Yes      |
| Filtering                                      | Yes      |
| Sorting                                        | Yes      |

---

## 46. AI Testing Guidelines

### 46.1 Model Testing

| Test                           | Purpose                                      |
|--------------------------------|----------------------------------------------|
| Accuracy on test set           | Verify detection quality                     |
| Inference latency              | Verify < 200ms requirement                   |
| Throughput                     | Verify FPS under load                        |
| Memory usage                   | Verify within VRAM limits                    |
| Error rate                     | Verify graceful degradation                  |
| Model versioning               | Verify rollback capability                   |

### 46.2 AI Service Testing

| Test                           | Purpose                                      |
|--------------------------------|----------------------------------------------|
| Health endpoint                | Verify service liveness                      |
| Inference endpoint             | Verify detection pipeline                    |
| Concurrent requests            | Verify thread safety                         |
| Large frame handling           | Verify memory management                     |
| GPU unavailable                | Verify CPU fallback                          |
| Model reload                   | Verify hot-swap without downtime             |

---

## 47. Security Testing Responsibilities

### 47.1 Developer Security Checklist

| Check                                          | When to Test                                |
|------------------------------------------------|---------------------------------------------|
| Input validation on all endpoints              | Every new endpoint                          |
| SQL injection prevention                       | Every database query                        |
| Authentication bypass                          | Every public endpoint                       |
| Authorization bypass                           | Every protected endpoint                    |
| Site scope bypass                              | Every resource access                       |
| Secret leakage in logs                         | Every log statement                         |
| XSS prevention                                 | Every user input display                    |
| CSRF protection                                | Every state-changing endpoint               |
| Rate limiting                                  | Every public endpoint                       |

### 47.2 Security Testing Tools

| Tool                  | Purpose                                      |
|-----------------------|----------------------------------------------|
| cargo-audit           | Rust dependency vulnerability scanning        |
| pip-audit             | Python dependency vulnerability scanning      |
| npm audit             | JavaScript dependency vulnerability scanning  |
| Trivy                 | Container image vulnerability scanning        |
| OWASP ZAP             | Dynamic application security testing          |
| SQLx compile-time     | SQL injection prevention                     |

---

## 48. Performance Considerations

### 48.1 Performance Targets

| Metric                        | Target                                      |
|-------------------------------|---------------------------------------------|
| API response time (p50)       | < 50ms                                       |
| API response time (p95)       | < 200ms                                      |
| API response time (p99)       | < 500ms                                      |
| WebSocket event delivery      | < 1 second                                   |
| AI inference latency          | < 200ms                                      |
| Evidence retrieval            | < 10 seconds                                 |
| Database query (simple)       | < 10ms                                       |
| Database query (complex)      | < 100ms                                      |

### 48.2 Performance Anti-Patterns

| Anti-Pattern                   | Solution                                      |
|--------------------------------|-----------------------------------------------|
| N+1 queries                    | Use JOINs or batch queries                     |
| Unbounded result sets          | Always paginate                                |
| Blocking the async runtime     | Use async I/O throughout                       |
| Missing database indexes       | Add indexes for common query patterns          |
| Unnecessary serialization      | Minimize data transferred                      |
| No connection pooling          | Use pool for all database connections          |
| Synchronous file I/O           | Use tokio::fs for async file operations        |

---

## 49. Memory Management Guidelines

### 49.1 Rust Memory Guidelines

| Guideline                        | Rationale                                    |
|----------------------------------|----------------------------------------------|
| Prefer owned types               | Clear ownership, no lifetime issues          |
| Use references for read-only     | Avoid unnecessary cloning                    |
| Clone only when necessary        | Performance impact of unnecessary clones     |
| Use Arc for shared ownership     | Thread-safe shared data                      |
| Use Mutex/RwLock for mutable shared | Synchronized access                     |
| Drop large buffers promptly      | Free memory as soon as possible              |

### 49.2 Python Memory Guidelines

| Guideline                        | Rationale                                    |
|----------------------------------|----------------------------------------------|
| Release frame references promptly| Prevent memory accumulation                  |
| Use generators for large datasets| Lazy evaluation, constant memory             |
| Close file handles explicitly    | Prevent resource leaks                       |
| Use context managers             | Automatic resource cleanup                   |
| Monitor GPU memory               | Prevent OOM on GPU                           |

### 49.3 Frontend Memory Guidelines

| Guideline                        | Rationale                                    |
|----------------------------------|----------------------------------------------|
| Clean up WebSocket connections   | Prevent memory leaks                         |
| Cancel pending requests          | Prevent stale data processing                |
| Use React.memo wisely            | Prevent unnecessary re-renders               |
| Lazy load heavy components       | Reduce initial bundle size                   |
| Virtualize long lists            | Render only visible items                    |

---

## 50. Concurrency Guidelines

### 50.1 Rust Concurrency Rules

| Rule                                         | Implementation                              |
|----------------------------------------------|---------------------------------------------|
| Use `tokio::spawn` for concurrent tasks      | Parallel execution                          |
| Use channels for inter-task communication    | `tokio::sync::mpsc`                         |
| Use `Arc<Mutex<T>>` for shared mutable state | Thread-safe mutation                        |
| Use `RwLock` for read-heavy shared state     | Concurrent reads, exclusive writes          |
| Never hold locks across `.await` points      | Prevent deadlocks                           |
| Use `select!` for concurrent async operations| Handle multiple branches                    |
| Use `join!` for parallel async operations    | Execute concurrently, wait for all          |

### 50.2 Common Concurrency Patterns

| Pattern                   | Use Case                                      |
|---------------------------|-----------------------------------------------|
| Fan-out/fan-in            | Parallel query execution                      |
| Semaphore                 | Limit concurrent connections                  |
| Broadcast channel         | Fan-out events to multiple subscribers        |
| Oneshot channel           | One-time response from async task             |
| Mutex                     | Short-held exclusive access                   |
| RwLock                    | Long-held shared access                       |

---

## 51. Async Programming Guidelines

### 51.1 Async Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| All I/O operations must be async             | Non-blocking throughout                      |
| Never use `std::thread::sleep` in async code | Blocks the runtime                           |
| Use `tokio::time::sleep` instead             | Non-blocking sleep                           |
| Avoid holding locks across await points      | Prevent deadlocks                            |
| Use `.await` at the end of expression        | Maximize concurrency                         |
| Use `?` for error propagation                | Clean async error handling                   |

### 51.2 Async Patterns

| Pattern                   | When to Use                                   |
|---------------------------|-----------------------------------------------|
| `async fn`                | All I/O-bound functions                       |
| `tokio::spawn`            | Independent concurrent tasks                  |
| `tokio::select!`          | Multiple async branches                       |
| `tokio::join!`            | Parallel async operations                     |
| `tokio::sync::watch`      | Broadcast configuration changes               |
| `tokio::sync::Notify`     | One-shot notification                         |

---

## 52. Database Performance Guidelines

### 52.1 Query Optimization

| Guideline                        | Implementation                              |
|----------------------------------|---------------------------------------------|
| Add indexes for WHERE clauses    | Index columns used in filters               |
| Add indexes for JOIN columns     | Index foreign key columns                    |
| Use EXPLAIN ANALYZE              | Verify query plans                          |
| Avoid SELECT *                   | Only select needed columns                  |
| Use LIMIT for large result sets  | Prevent unbounded queries                   |
| Batch inserts                    | Use multi-row INSERT                        |
| Use connections pooling           | Avoid connection churn                      |

### 52.2 Migration Performance

| Guideline                        | Implementation                              |
|----------------------------------|---------------------------------------------|
| Add indexes CONCURRENTLY         | No table locks in PostgreSQL                |
| Batch large data migrations      | Process in chunks of 1000 rows              |
| Test with production-scale data  | Verify migration time                       |
| Plan rollback strategy           | Know how to undo if needed                  |

---

## 53. Caching Guidelines

### 53.1 What to Cache

| Target                        | Cache Duration    | Invalidation Trigger               |
|-------------------------------|-------------------|-------------------------------------|
| User permissions              | 5 minutes         | Role change, permission change      |
| Active rules                  | On change         | Rule update/delete                  |
| Camera fleet config           | On change         | Camera update                       |
| Dashboard metrics             | 30 seconds        | Periodic refresh                    |
| JWT blocked list              | Token lifetime    | Token revocation                    |

### 53.2 Caching Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Always have cache invalidation plan          | Prevent stale data                           |
| Use TTL for automatic expiration             | Prevent unbounded cache growth               |
| Handle cache misses gracefully               | Fall back to database                        |
| Log cache hits/misses for monitoring         | Aid debugging                                |
| Never cache sensitive data without encryption| Security                                    |

---

## 54. Debugging Guide

### 54.1 Debug Workflow

```mermaid
graph TB
    A[Issue Reported] --> B[Reproduce Locally]
    B --> C{Can Reproduce?}
    C -->|Yes| D[Isolate Root Cause]
    C -->|No| E[Check Logs + Metrics]
    E --> F{Found Cause?}
    F -->|Yes| D
    F -->|No| G[Add More Logging]
    G --> B
    D --> H[Implement Fix]
    H --> I[Write Test]
    I --> J[Verify Fix]
    J --> K[Code Review]
```

### 54.2 Debugging Tools

| Tool                  | Purpose                                      |
|-----------------------|----------------------------------------------|
| `RUST_LOG=debug`      | Enable debug logging for Rust                 |
| `LOG_LEVEL=debug`     | Enable debug logging for Python               |
| `cargo test -- --nocapture` | Show println output in tests           |
| `rust-gdb`            | GDB debugger for Rust                        |
| `pdb`                 | Python debugger                              |
| VS Code debugger      | Integrated debugging                         |
| `psql`                | Direct database inspection                   |
| `redis-cli`           | Cache inspection                             |

### 54.3 Common Debug Commands

| Command                                     | Purpose                              |
|---------------------------------------------|--------------------------------------|
| `docker compose logs -f {service}`          | Follow container logs                |
| `docker compose exec {service} sh`          | Shell into container                 |
| `docker compose ps`                         | Check container status               |
| `curl http://localhost:8080/api/v1/health`  | Check API health                     |
| `cargo test -- --nocapture`                 | Run tests with output                |
| `psql $DATABASE_URL`                        | Connect to database                  |
| `redis-cli`                                 | Connect to Redis                     |

---

## 55. Logging During Development

### 55.1 Development Logging Setup

| Service     | Environment Variable        | Recommended Value              |
|-------------|-----------------------------|--------------------------------|
| Backend     | `RUST_LOG`                  | `vigilantai=debug,tower_http=debug` |
| AI Service  | `LOG_LEVEL`                 | `debug`                        |
| Dashboard   | Browser DevTools            | Console tab                     |

### 55.2 What to Log During Development

| What                          | Level   | Why                                       |
|-------------------------------|---------|-------------------------------------------|
| Function entry                | debug   | Trace execution flow                      |
| Database queries              | debug   | Verify correct queries                    |
| External service calls        | debug   | Verify integration                        |
| Validation failures           | debug   | Debug input issues                        |
| Cache hits/misses             | debug   | Verify caching behavior                   |
| Error conditions              | error   | Identify failures                         |

---

## 56. Troubleshooting Guide

### 56.1 Common Issues

| Symptom                       | Likely Cause                    | Solution                              |
|-------------------------------|---------------------------------|---------------------------------------|
| API returns 500               | Database connection failed      | Check DATABASE_URL, DB running        |
| API returns 401               | JWT expired or invalid          | Refresh token, check JWT keys         |
| API returns 403               | Missing permission              | Check user roles, RBAC config         |
| WebSocket disconnects         | Heartbeat timeout               | Check network, increase timeout       |
| Evidence upload fails         | Disk full or permissions        | Check EVIDENCE_DIR, disk space        |
| AI inference slow             | CPU fallback (no GPU)           | Check GPU availability, CUDA drivers  |
| Docker containers won't start | Port conflict or env vars       | Check ports, verify .env config       |
| Migration fails               | Schema mismatch                 | Check migration order, DB version     |
| Build fails (Rust)            | Missing system dependencies     | Install OpenSSL, pkg-config           |
| Build fails (Python)          | Missing system dependencies     | Install OpenCV, build tools           |

### 56.2 Debugging Checklist

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Check container logs for errors           |
| 2    | Verify environment variables are set      |
| 3    | Check database connectivity               |
| 4    | Check Redis connectivity                  |
| 5    | Verify file system permissions            |
| 6    | Check network connectivity between services|
| 7    | Review recent code changes                |
| 8    | Search for similar issues in documentation|
| 9    | Add debug logging to narrow scope         |
| 10   | Ask for help with full context            |

---

## 57. Common Development Tasks

### 57.1 Adding a New API Endpoint

| Step | Action                                    | Files Affected              |
|------|-------------------------------------------|-----------------------------|
| 1    | Define domain model                       | `models/{entity}.rs`        |
| 2    | Define request/response types             | `routes/{module}.rs`        |
| 3    | Create repository trait + impl            | `repositories/{entity}.rs`  |
| 4    | Create service method                     | `services/{module}.rs`      |
| 5    | Create route handler                      | `routes/{module}.rs`        |
| 6    | Register route in router                  | `routes/mod.rs`             |
| 7    | Add migration (if new table)              | `migrations/`               |
| 8    | Add unit tests                            | Same file `#[cfg(test)]`    |
| 9    | Add integration tests                     | `tests/`                    |
| 10   | Update API documentation                  | `docs/06`                   |

### 57.2 Adding a New Database Table

| Step | Action                                    | Files Affected              |
|------|-------------------------------------------|-----------------------------|
| 1    | Create migration file                     | `migrations/`               |
| 2    | Define domain model                       | `models/{entity}.rs`        |
| 3    | Implement FromRow for PostgreSQL          | `models/{entity}.rs`        |
| 4    | Create repository trait                   | `repositories/{entity}.rs`  |
| 5    | Implement repository                      | `repositories/{entity}.rs`  |
| 6    | Update AppState with new repository       | `app.rs`                    |
| 7    | Add tests                                 | `tests/`                    |

### 57.3 Adding a New Middleware

| Step | Action                                    | Files Affected              |
|------|-------------------------------------------|-----------------------------|
| 1    | Define middleware structure                | `middleware/{name}.rs`       |
| 2    | Implement Tower Layer trait               | `middleware/{name}.rs`       |
| 3    | Implement Tower Service trait             | `middleware/{name}.rs`       |
| 4    | Add to middleware stack                    | `app.rs`                    |
| 5    | Add tests                                 | `middleware/{name}.rs`       |

### 57.4 Adding a New WebSocket Event

| Step | Action                                    | Files Affected              |
|------|-------------------------------------------|-----------------------------|
| 1    | Define event payload type                 | `models/`                   |
| 2    | Add event to WebSocket handler            | `routes/websocket.rs`       |
| 3    | Add event to subscription logic           | `services/`                 |
| 4    | Add event to frontend handler             | `dashboard/src/hooks/`      |
| 5    | Update WebSocket documentation            | `docs/06`                   |

### 57.5 Adding a New RBAC Permission

| Step | Action                                    | Files Affected              |
|------|-------------------------------------------|-----------------------------|
| 1    | Add permission to permissions table       | `migrations/`               |
| 2    | Map permission to roles                   | `migrations/`               |
| 3    | Update JWT claims generation              | `services/auth_service.rs`  |
| 4    | Add permission check to route             | `middleware/rbac.rs`        |
| 5    | Update frontend permission cache          | `dashboard/src/lib/`        |
| 6    | Test with each role                       | `tests/`                    |

---

## 58. Build Process Overview

### 58.1 Backend Build

| Step | Command                                   | Purpose                    |
|------|-------------------------------------------|----------------------------|
| 1    | `cargo fetch`                             | Download dependencies      |
| 2    | `cargo check`                             | Type-check without building|
| 3    | `cargo clippy`                            | Lint for common mistakes   |
| 4    | `cargo fmt --check`                       | Check formatting           |
| 5    | `cargo build --release`                   | Optimized build            |
| 6    | `cargo test`                              | Run all tests              |

### 58.2 AI Service Build

| Step | Command                                   | Purpose                    |
|------|-------------------------------------------|----------------------------|
| 1    | `pip install -r requirements.txt`         | Install dependencies       |
| 2    | `ruff check .`                            | Lint Python code           |
| 3    | `ruff format .`                           | Format Python code         |
| 4    | `pytest`                                  | Run all tests              |

### 58.3 Dashboard Build

| Step | Command                                   | Purpose                    |
|------|-------------------------------------------|----------------------------|
| 1    | `npm ci`                                  | Install dependencies       |
| 2    | `npm run lint`                            | Lint TypeScript/React      |
| 3    | `npm run typecheck`                       | Type-check                 |
| 4    | `npm run build`                           | Production build           |
| 5    | `npm test`                                | Run all tests              |

### 58.4 Docker Build

| Step | Command                                   | Purpose                    |
|------|-------------------------------------------|----------------------------|
| 1    | `docker compose build`                    | Build all images           |
| 2    | `docker compose up -d`                    | Start all containers       |
| 3    | `docker compose ps`                       | Verify running             |
| 4    | `docker compose logs -f`                  | Follow logs                |

---

## 59. Release Process Overview

### 59.1 Release Steps

| Step | Action                                    | Responsible               |
|------|-------------------------------------------|---------------------------|
| 1    | Create release branch from main           | Release manager           |
| 2    | Bump version numbers                      | Release manager           |
| 3    | Update CHANGELOG.md                       | Release manager           |
| 4    | Run full test suite                       | CI pipeline               |
| 5    | Build all container images                | CI pipeline               |
| 6    | Deploy to staging                         | CD pipeline               |
| 7    | Run staging validation                    | QA team                   |
| 8    | Manual approval gate                      | Engineering lead          |
| 9    | Deploy to production                      | CD pipeline               |
| 10   | Monitor for 15 minutes                    | SRE team                  |
| 11   | Tag release in Git                        | Release manager           |
| 12   | Merge release branch to main              | Release manager           |

### 59.2 Version Numbering

| Component     | Format          | Example    |
|---------------|-----------------|------------|
| Major release | X.0.0           | 2.0.0      |
| Minor release | 0.X.0           | 1.2.0      |
| Patch release | 0.0.X           | 1.2.3      |
| Pre-release   | 0.0.0-beta.N    | 1.2.0-beta.1 |

---

## 60. CI/CD Integration Overview

### 60.1 CI Pipeline

| Stage                 | Actions                                      |
|-----------------------|----------------------------------------------|
| Lint                  | clippy, ruff, eslint, prettier               |
| Type check            | cargo check, mypy, tsc                        |
| Unit tests            | cargo test, pytest, vitest                   |
| Integration tests     | API tests with test database                 |
| Security scan         | cargo-audit, pip-audit, npm audit             |
| Build                 | cargo build, pip wheel, npm run build        |
| Docker build          | Build container images                        |
| Image scan            | Trivy container scanning                      |

### 60.2 CD Pipeline

| Stage                 | Actions                                      |
|-----------------------|----------------------------------------------|
| Deploy to Dev         | Auto after CI passes                         |
| Deploy to Test        | Auto after Dev deployment                    |
| Deploy to QA          | Auto after Test deployment                   |
| Deploy to Staging     | Manual trigger                               |
| Deploy to Production  | Manual approval gate                         |

---

## 61. Local Deployment Workflow

### 61.1 Full Stack Local Deployment

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | `git clone` repository                    |
| 2    | Copy `.env.example` to `.env`             |
| 3    | Configure environment variables           |
| 4    | `docker compose up -d` (databases only)  |
| 5    | Run database migrations                   |
| 6    | Start backend: `cargo run`                |
| 7    | Start AI service: `python -m uvicorn`    |
| 8    | Start dashboard: `npm run dev`            |
| 9    | Open browser to `http://localhost:3000`   |
| 10   | Verify all services healthy               |

### 61.2 Docker-Only Deployment

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | `docker compose up -d`                    |
| 2    | Wait for all containers healthy           |
| 3    | Open browser to `http://localhost:3000`   |
| 4    | Login with default admin credentials      |

---

## 62. Production Awareness

### 62.1 What Differs in Production

| Aspect                    | Development                  | Production                       |
|---------------------------|------------------------------|----------------------------------|
| Database                  | SQLite                       | PostgreSQL with replication      |
| Cache                     | Optional                     | Redis with Sentinel              |
| TLS                       | Self-signed or none          | Custom CA, TLS 1.3              |
| Secrets                   | .env file                    | Environment variables / Vault    |
| Monitoring                | Console logs                 | Prometheus + Grafana + Loki      |
| Backup                    | None                         | Automated, verified             |
| Logging                   | stdout                       | Centralized (Loki)              |
| Rate limiting             | Relaxed                      | Enforced (100/min)              |
| Error messages            | Detailed                     | Generic (no internals exposed)  |

### 62.2 Production Safety Rules

| Rule                                         | Rationale                                    |
|----------------------------------------------|----------------------------------------------|
| Never test with production data               | Data privacy, compliance                      |
| Never debug on production directly            | Risk of data corruption                       |
| Always use feature flags for risky changes    | Easy rollback                                 |
| Always have rollback plan before deploy       | Minimize downtime                             |
| Always monitor after deploy                   | Catch issues early                            |
| Never commit secrets to Git                   | Security                                     |

---

## 63. Security Best Practices

### 63.1 Development Security Rules

| Rule                                         | Enforcement                                  |
|----------------------------------------------|----------------------------------------------|
| Never hardcode secrets                        | Environment variables only                   |
| Never log sensitive data                      | Filter passwords, tokens, keys               |
| Always validate input                         | Reject invalid data at boundary              |
| Always use parameterized queries              | Prevent SQL injection                        |
| Always hash passwords with Argon2id/bcrypt    | Never store plain text                       |
| Always use HTTPS in production                | Encrypt all traffic                          |
| Always set httpOnly on cookies                | Prevent XSS token theft                      |
| Always set secure flag on cookies             | Prevent cookie interception                  |
| Never expose internal error details           | Security best practice                       |

### 63.2 Secret Handling Guidelines

| Rule                                         | Implementation                              |
|----------------------------------------------|---------------------------------------------|
| Store secrets in environment variables       | Not in code or config files                 |
| Never commit .env files                      | .gitignore enforced                         |
| Never log secrets                            | Log filtering in place                      |
| Rotate secrets on schedule                   | 30-90 day rotation                          |
| Use different secrets per environment        | Dev ≠ Staging ≠ Production                  |
| Limit secret access                          | Service-specific env vars                   |

---

## 64. Dependency Update Process

### 64.1 Update Strategy

| Dependency Type     | Frequency         | Process                              |
|---------------------|-------------------|--------------------------------------|
| Security patches    | Immediately       | Automated PR + fast-track review     |
| Patch updates       | Weekly            | Automated PR + standard review       |
| Minor updates       | Monthly           | Manual review + testing              |
| Major updates       | Quarterly         | Manual review + migration plan       |

### 64.2 Update Process

| Step | Action                                    |
|------|-------------------------------------------|
| 1    | Review changelog for breaking changes     |
| 2    | Check security vulnerability fixes        |
| 3    | Update dependency in manifest             |
| 4    | Run full test suite                       |
| 5    | Verify no regressions                     |
| 6    | Submit PR with update details             |
| 7    | Get code review approval                  |
| 8    | Merge                                     |

---

## 65. Vulnerability Management Responsibilities

### 65.1 Developer Responsibilities

| Responsibility                        | When                                      |
|---------------------------------------|-------------------------------------------|
| Check for known vulnerabilities       | Before adding new dependency              |
| Update vulnerable dependencies        | When notified by security team            |
| Report potential vulnerabilities      | Immediately upon discovery                |
| Follow secure coding practices        | Always                                    |
| Run security scans locally            | Before submitting PR                      |

### 65.2 Vulnerability Response

| CVSS Score      | Response Time   | Action                                    |
|-----------------|-----------------|-------------------------------------------|
| Critical (9-10) | 24 hours        | Immediate fix, emergency deploy           |
| High (7-8)      | 7 days          | Prioritized fix                           |
| Medium (4-6)    | 30 days         | Scheduled fix                             |
| Low (0-3)       | 90 days         | Fix when convenient                       |

---

## 66. Common Mistakes to Avoid

### 66.1 Rust Mistakes

| Mistake                                    | Correct Approach                           |
|--------------------------------------------|--------------------------------------------|
| Using `.unwrap()` in production code       | Use `?` or `.ok_or()` with error mapping   |
| Blocking the async runtime                 | Use async I/O throughout                    |
| Holding locks across `.await`              | Drop locks before awaiting                 |
| Cloning excessively                        | Use references where possible              |
| Ignoring compiler warnings                 | Fix all warnings before merge              |
| Using `println!` for logging               | Use `tracing` macros                        |
| SQL string interpolation                   | Use SQLx parameterized queries             |
| Missing error context                      | Add `.context("description")`              |

### 66.2 Python Mistakes

| Mistake                                    | Correct Approach                           |
|--------------------------------------------|--------------------------------------------|
| Missing type hints                         | Add type hints on all public functions     |
| Blocking the event loop                     | Use async I/O in FastAPI                    |
| Not closing resources                      | Use context managers (`with` statements)   |
| Importing heavy modules at module level    | Lazy import for optional dependencies      |
| Catching all exceptions                    | Catch specific exception types             |

### 66.3 TypeScript Mistakes

| Mistake                                    | Correct Approach                           |
|--------------------------------------------|--------------------------------------------|
| Using `any` type                           | Define proper types                        |
| Missing error boundaries                   | Add React error boundaries                 |
| Not cleaning up subscriptions              | Return cleanup from useEffect              |
| Direct DOM manipulation                    | Use React state and refs                   |
| Hardcoding API URLs                        | Use environment variables                  |

### 66.4 General Mistakes

| Mistake                                    | Correct Approach                           |
|--------------------------------------------|--------------------------------------------|
| Committing secrets to Git                  | Use .gitignore, scan with pre-commit hooks |
| Skipping tests                             | Write tests for every feature              |
| Not updating documentation                 | Update docs in same PR                     |
| Large PRs (>500 lines)                     | Break into smaller, focused PRs            |
| Silent error handling                      | Log errors, return meaningful responses    |
| Not checking edge cases                    | Test boundary conditions explicitly        |

---

## 67. Frequently Asked Questions

### 67.1 General

| Question                                      | Answer                                      |
|-----------------------------------------------|----------------------------------------------|
| What language is the backend written in?       | Rust with Axum framework                     |
| What language is the AI service in?            | Python with FastAPI                          |
| What database is used?                         | SQLite (dev), PostgreSQL (production)        |
| How do I run the project locally?              | See Section 9, Local Development Environment |
| Where is the API documentation?                | `docs/06-API-Specification.md`              |

### 67.2 Development

| Question                                      | Answer                                      |
|-----------------------------------------------|----------------------------------------------|
| How do I add a new endpoint?                   | See Section 57.1                             |
| How do I add a new database table?             | See Section 57.2                             |
| How do I run tests?                            | `make test` or see Section 42                |
| How do I format my code?                       | `make format` or see Section 38              |
| How do I lint my code?                         | `make lint` or see Section 38                |

### 67.3 Debugging

| Question                                      | Answer                                      |
|-----------------------------------------------|----------------------------------------------|
| How do I debug the backend?                    | Set `RUST_LOG=debug`, see Section 64         |
| How do I check database state?                 | Use psql or DBeaver, see Section 54          |
| How do I check Redis state?                    | Use redis-cli, see Section 54                |
| How do I view container logs?                  | `docker compose logs -f {service}`           |
| The API returns 500, what do I do?             | See Section 56.1 troubleshooting table       |

### 67.4 Git

| Question                                      | Answer                                      |
|-----------------------------------------------|----------------------------------------------|
| What branch should I work on?                  | Create feature/* or bugfix/* branch          |
| How do I write commit messages?                | See Section 35, commit convention            |
| How do I create a PR?                          | See Section 36, PR guidelines                |
| How do I get my PR approved?                   | See Section 37, code review checklist        |

---

## 68. Developer Onboarding Checklist

### 68.1 Week 1: Setup and Orientation

| Check                                          | Status |
|------------------------------------------------|--------|
| Repository cloned                              | [ ]    |
| Development environment set up                 | [ ]    |
| All services running locally                   | [ ]    |
| Test suite passing                             | [ ]    |
| Architecture documents read (01-08)            | [ ]    |
| This developer guide read                      | [ ]    |
| IDE configured with recommended extensions     | [ ]    |
| Git workflow understood                        | [ ]    |
| First "hello world" PR submitted               | [ ]    |

### 68.2 Week 2: Deep Dive

| Check                                          | Status |
|------------------------------------------------|--------|
| Backend architecture understood                | [ ]    |
| AI service architecture understood             | [ ]    |
| Frontend architecture understood               | [ ]    |
| Database schema reviewed                       | [ ]    |
| API specification reviewed                     | [ ]    |
| Security architecture reviewed                 | [ ]    |
| Deployment architecture reviewed               | [ ]    |
| First feature PR submitted                     | [ ]    |
| First code review completed                    | [ ]    |

### 68.3 Week 3: Productivity

| Check                                          | Status |
|------------------------------------------------|--------|
| Completed first feature independently          | [ ]    |
| Written tests for new code                     | [ ]    |
| Debugged and fixed first bug                   | [ ]    |
| Participated in code review                    | [ ]    |
| Understood monitoring and alerting             | [ ]    |
| Familiar with common development tasks         | [ ]    |
| Connected with team members                    | [ ]    |
| Identified areas of interest                   | [ ]    |

### 68.4 Week 4: Full Speed

| Check                                          | Status |
|------------------------------------------------|--------|
| Working independently on features              | [ ]    |
| Contributing to code reviews                   | [ ]    |
| Understanding production operations            | [ ]    |
| Aware of security best practices               | [ ]    |
| Following all coding standards                 | [ ]    |
| Contributing to documentation                  | [ ]    |
| Participating in team ceremonies               | [ ]    |
| Onboarding complete                            | [ ]    |

---

## 69. Glossary

| Term                          | Definition                                      |
|-------------------------------|-------------------------------------------------|
| API                           | Application Programming Interface                |
| Axum                          | Rust web framework built on Tokio and Tower      |
| CI/CD                         | Continuous Integration / Continuous Deployment   |
| Docker                        | Container runtime and image format               |
| FastAPI                       | Python web framework for building APIs           |
| JWT                           | JSON Web Token                                   |
| NVR                           | Network Video Recorder                           |
| OIDC                          | OpenID Connect                                   |
| OWASP                         | Open Web Application Security Project            |
| PostgreSQL                    | Open-source relational database                  |
| RBAC                          | Role-Based Access Control                        |
| Redis                         | In-memory data structure store                   |
| RTSP                          | Real-Time Streaming Protocol                     |
| SHA-256                       | Secure Hash Algorithm (256-bit)                  |
| SQLx                          | Rust async SQL toolkit                           |
| SQLite                        | Embedded relational database                     |
| SRE                           | Site Reliability Engineer                        |
| Tokio                         | Rust async runtime                               |
| Tower                         | Rust middleware framework                        |
| VMS                           | Video Management System                          |
| WebSocket                     | Full-duplex communication protocol               |
| YOLO                          | You Only Look Once (object detection model)      |

---

## 70. Appendices

### Appendix A: Git Branch Reference

| Branch Pattern              | Purpose                          | Lifetime        |
|-----------------------------|----------------------------------|-----------------|
| `main`                      | Production-ready code            | Permanent       |
| `feature/*`                 | New feature development          | Until merged    |
| `bugfix/*`                  | Bug fix development              | Until merged    |
| `hotfix/*`                  | Critical production fix          | Until merged    |
| `release/*`                 | Release preparation              | Until released  |

### Appendix B: Commit Type Reference

| Type        | Usage                                              | Example                           |
|-------------|----------------------------------------------------|------------------------------------|
| `feat`      | New feature                                        | `feat(api): add evidence upload`  |
| `fix`       | Bug fix                                            | `fix(auth): prevent token reuse`  |
| `docs`      | Documentation changes                              | `docs: update README`              |
| `style`     | Code style (no logic change)                       | `style: format with rustfmt`      |
| `refactor`  | Code refactoring                                   | `refactor: extract service logic` |
| `test`      | Adding/updating tests                              | `test: add camera CRUD tests`     |
| `chore`     | Build process, dependencies                        | `chore: update dependencies`      |
| `perf`      | Performance improvement                            | `perf(db): add index`              |
| `ci`        | CI/CD changes                                      | `ci: add security scan`            |

### Appendix C: Error Code Reference

| HTTP Code   | Meaning                   | When to Return                              |
|-------------|---------------------------|----------------------------------------------|
| 200         | OK                        | Successful GET, PUT, PATCH, DELETE            |
| 201         | Created                   | Successful POST                              |
| 204         | No Content                | Successful DELETE with no body                |
| 400         | Bad Request               | Invalid request format                       |
| 401         | Unauthorized              | Missing or invalid JWT                       |
| 403         | Forbidden                 | Valid JWT, insufficient permissions          |
| 404         | Not Found                 | Resource doesn't exist                       |
| 409         | Conflict                  | Duplicate resource                           |
| 422         | Unprocessable Entity      | Business rule violation                      |
| 429         | Too Many Requests         | Rate limit exceeded                          |
| 500         | Internal Server Error     | Unexpected server failure                    |

### Appendix D: Port Reference

| Service                 | Port    | Protocol | Purpose                        |
|-------------------------|---------|----------|--------------------------------|
| Next.js Dashboard       | 3000    | HTTP     | Dashboard UI                   |
| Axum API                | 8080    | HTTP     | REST API + WebSocket           |
| AI Inference            | 8081    | HTTP     | Internal AI service            |
| PostgreSQL              | 5432    | TCP      | Database                       |
| Redis                   | 6379    | TCP      | Cache + sessions               |
| Prometheus              | 9090    | HTTP     | Metrics                        |
| Grafana                 | 3001    | HTTP     | Dashboard UI                   |

### Appendix E: Dependency Reference

| Category            | Rust Crate        | Python Package    | npm Package       |
|---------------------|-------------------|-------------------|-------------------|
| Web framework       | axum              | fastapi           | next              |
| Async runtime       | tokio             | uvicorn           | —                 |
| Database            | sqlx              | asyncpg           | —                 |
| Serialization       | serde             | pydantic          | —                 |
| Error handling      | thiserror         | —                 | —                 |
| Logging             | tracing           | structlog         | —                 |
| Authentication      | jsonwebtoken      | python-jose       | —                 |
| HTTP client         | reqwest           | httpx             | —                 |
| Validation          | validator         | pydantic          | zod               |
| Testing             | mockall           | pytest            | vitest            |
| Linting             | clippy            | ruff              | eslint            |
| Formatting          | rustfmt           | ruff format       | prettier          |

### Appendix F: Quick Reference Commands

| Action                         | Command                                      |
|--------------------------------|----------------------------------------------|
| Build backend                  | `cargo build`                                |
| Build backend (release)        | `cargo build --release`                      |
| Run backend tests              | `cargo test`                                 |
| Lint backend                   | `cargo clippy -- -D warnings`                |
| Format backend                 | `cargo fmt`                                  |
| Build AI service               | `pip install -r requirements.txt`            |
| Run AI service tests           | `pytest`                                     |
| Lint AI service                | `ruff check .`                               |
| Format AI service              | `ruff format .`                              |
| Build dashboard                | `npm run build`                              |
| Run dashboard tests            | `npm test`                                   |
| Lint dashboard                 | `npm run lint`                               |
| Format dashboard               | `npm run format`                             |
| Start all services             | `make dev`                                   |
| Run all tests                  | `make test`                                  |
| Run all linters                | `make lint`                                  |
| Format all code                | `make format`                                |
| Start Docker services          | `docker compose up -d`                       |
| Stop Docker services           | `docker compose down`                        |
| View Docker logs               | `docker compose logs -f`                     |
| Run migrations                 | `make migrate`                               |
| Check API health               | `curl http://localhost:8080/api/v1/health`   |

---

*End of VigilantAI Developer Guide*
