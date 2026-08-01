# -- stage 1: cargo-chef prepare --
FROM rust:1.89-bookworm AS chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# -- stage 2: build dependencies --
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# -- stage 3: build application --
COPY . .
RUN touch camera-gateway/src/main.rs && cargo build --release --bin camera-gateway

# -- stage 4: runtime --
FROM debian:bookworm-slim AS runner
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system gateway && useradd --system --gid gateway gateway

WORKDIR /app
COPY --from=builder /app/target/release/camera-gateway /app/camera-gateway

RUN chown -R gateway:gateway /app

USER gateway

EXPOSE 8082

CMD ["/app/camera-gateway"]
