# VigilantAI

Enterprise Security Intelligence Platform

## Overview

VigilantAI is a mission-critical security intelligence platform that provides real-time threat detection, event correlation, and evidence management using AI-powered video analytics.

## Architecture

| Service           | Technology          | Description                              |
|-------------------|---------------------|------------------------------------------|
| backend           | Rust (Axum)         | Core API, authentication, event processing |
| camera-gateway    | Rust                | RTSP stream ingestion, frame capture     |
| ai-service        | Python (FastAPI)    | AI inference engine, detection models    |
| dashboard         | Next.js             | Web UI for operators and analysts        |

## Repository Structure

```
.
├── backend/            # Rust API service (Axum)
├── camera-gateway/     # Rust camera gateway service
├── ai-service/         # Python AI inference service (FastAPI)
├── dashboard/          # Next.js frontend
├── deploy/             # Deployment configurations
├── scripts/            # Operational and utility scripts
├── tests/              # Integration and end-to-end tests
├── docs/               # Architecture and operations documentation
├── .github/            # GitHub workflows and templates
├── Cargo.toml          # Rust workspace root
├── docker-compose.yml  # Local development stack
├── Makefile            # Common development commands
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

## Prerequisites

- Rust 1.75+ (via rustup)
- Python 3.11+
- Node.js 20+
- Docker and Docker Compose
- PostgreSQL 16+
- Redis 7+

## Quick Start

```bash
# Start infrastructure services
make infra-up

# Set up the AI service
cd ai-service && python -m venv .venv && .venv\Scripts\activate && pip install -r requirements.txt

# Set up the dashboard
cd dashboard && npm install

# Build Rust services
make build

# Run all services locally
make dev
```

## Common Commands

Run `make help` to see all available targets.

## License

Proprietary. All rights reserved.
