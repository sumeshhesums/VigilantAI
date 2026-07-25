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
RUN touch backend/src/main.rs && cargo build --release --bin backend

# -- generate development RSA key pair --
RUN mkdir -p /app/keys \
    && openssl genpkey -algorithm RSA -pkeyopt rsa_keygen_bits:2048 -outform PEM \
       -out /app/keys/private_key.pem \
    && openssl rsa -pubout -in /app/keys/private_key.pem \
       -out /app/keys/public_key.pem

# -- stage 4: runtime --
FROM debian:bookworm-slim AS runner
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    libjpeg62-turbo \
    libpng16-16 \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --system vigilant && useradd --system --gid vigilant vigilant

WORKDIR /app
COPY --from=builder /app/target/release/backend /app/backend
COPY --from=builder /app/backend/migrations /app/migrations
COPY --from=builder /app/keys /app/keys
COPY backend/entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

RUN chown -R vigilant:vigilant /app

USER vigilant

EXPOSE 8080

ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["/app/backend"]
