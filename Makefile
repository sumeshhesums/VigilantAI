.PHONY: help build build-backend build-gateway test test-backend test-gateway lint fmt check dev infra-up infra-down clean

help: ## Show this help
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

# ── Build ──────────────────────────────────────────────

build: build-backend build-gateway ## Build all Rust services

build-backend: ## Build the backend API service
	cargo build --release -p backend

build-gateway: ## Build the camera gateway service
	cargo build --release -p camera-gateway

# ── Test ───────────────────────────────────────────────

test: test-backend test-gateway ## Run all Rust tests

test-backend: ## Run backend tests
	cargo test -p backend

test-gateway: ## Run camera-gateway tests
	cargo test -p camera-gateway

# ── Quality ────────────────────────────────────────────

lint: ## Run clippy on all workspace members
	cargo clippy --workspace --all-targets -- -D warnings

fmt: ## Check formatting across the workspace
	cargo fmt --all -- --check

fmt-fix: ## Auto-fix formatting across the workspace
	cargo fmt --all

check: fmt lint test ## Run format check, lint, and tests

# ── Dev ────────────────────────────────────────────────

dev: infra-up ## Start infrastructure and build services
	@echo "Infrastructure is running. Start services in their respective directories."

# ── Infrastructure ─────────────────────────────────────

infra-up: ## Start PostgreSQL and Redis via Docker Compose
	docker compose up -d

infra-down: ## Stop infrastructure services
	docker compose down

infra-logs: ## Tail infrastructure logs
	docker compose logs -f

# ── Clean ──────────────────────────────────────────────

clean: ## Remove build artifacts
	cargo clean
