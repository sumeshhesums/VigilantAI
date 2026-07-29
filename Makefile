.PHONY: help build build-backend build-gateway test test-backend test-gateway lint fmt check \
        dev infra-up infra-down infra-logs docker-build docker-up docker-down docker-logs \
        docker-restart docker-ps docker-clean generate-keys seed init

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ── Initialization ──────────────────────────────────────────

generate-keys: ## Generate JWT RSA key pair for local development
	@mkdir -p keys
	@if command -v openssl > /dev/null 2>&1; then \
		echo "Generating RSA 2048-bit key pair..."; \
		openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -out keys/private_key.pem 2>/dev/null; \
		openssl pkey -in keys/private_key.pem -pubout -out keys/public_key.pem 2>/dev/null; \
		echo "  Keys written to keys/ directory"; \
	else \
		echo "OpenSSL not found. Install OpenSSL or use scripts/generate-keys.ps1 (Windows)."; \
		exit 1; \
	fi

init: generate-keys ## Initialize the project (generate keys, prepare .env)
	@if [ ! -f .env ]; then \
		cp .env.example .env; \
		echo "Created .env from .env.example — edit with your settings."; \
	else \
		echo ".env already exists — skipping."; \
	fi
	@echo "Project initialized. Run 'make docker-up' to start all services."

# ── Build ───────────────────────────────────────────────────

build: build-backend build-gateway ## Build all Rust services

build-backend: ## Build the backend API service
	cargo build --release -p backend

build-gateway: ## Build the camera gateway service
	cargo build --release -p camera-gateway

# ── Test ────────────────────────────────────────────────────

test: test-backend test-gateway ## Run all Rust tests

test-backend: ## Run backend tests
	cargo test -p backend

test-gateway: ## Run camera-gateway tests
	cargo test -p camera-gateway

# ── Quality ─────────────────────────────────────────────────

lint: ## Run clippy on all workspace members
	cargo clippy --workspace --all-targets -- -D warnings

fmt: ## Check formatting across the workspace
	cargo fmt --all -- --check

fmt-fix: ## Auto-fix formatting across the workspace
	cargo fmt --all

check: fmt lint test ## Run format check, lint, and tests

# ── Dev ─────────────────────────────────────────────────────

dev: infra-up ## Start infrastructure and build services
	@echo "Infrastructure is running. Start services in their respective directories."

# ── Infrastructure (infra only) ────────────────────────────

infra-up: ## Start PostgreSQL and Redis via Docker Compose
	docker compose up -d postgres redis

infra-down: ## Stop infrastructure services
	docker compose down

infra-logs: ## Tail infrastructure logs
	docker compose logs -f postgres redis

# ── Docker (full stack) ────────────────────────────────────

docker-build: ## Build all Docker images
	docker compose build

docker-up: ## Start all services
	docker compose up -d

docker-down: ## Stop all services
	docker compose down

docker-logs: ## Tail all service logs
	docker compose logs -f

docker-restart: ## Restart all services
	docker compose restart

docker-ps: ## Show running services status
	docker compose ps

docker-clean: ## Stop all services and remove volumes
	docker compose down -v
	cargo clean
