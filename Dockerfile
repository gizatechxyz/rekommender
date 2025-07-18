FROM rust:1.75 as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY rust-toolchain.toml ./
COPY src ./src

# Enable SIMD for x86_64 Cloud Run instances
ENV RUSTFLAGS="-C target-cpu=x86-64-v3 -C target-feature=+avx2,+fma"

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/rekt-recommender-api /app/

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_LOG_LEVEL=normal
ENV PORT=8080
ENV MAX_UPLOAD_SIZE_MB=100
ENV OUTPUT_DIR=/app/outputs
ENV RESULT_TTL_HOURS=24
ENV RUST_BACKTRACE=full

EXPOSE 8080

CMD ["./rekt-recommender-api"]