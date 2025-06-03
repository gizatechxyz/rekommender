FROM rust:1.75 as builder

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY rust-toolchain.toml ./

COPY src ./src

# Build with explicit CPU features for SIMD
ENV RUSTFLAGS="-C target-cpu=haswell -C target-feature=+avx2,+fma,+f16c"

RUN cargo build --release --target x86_64-unknown-linux-gnu

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-gnu/release/rekt-recommender-api /app/

# Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_LOG_LEVEL=normal
ENV PORT=8080
ENV MAX_UPLOAD_SIZE_MB=100

EXPOSE 8080

CMD ["./rekt-recommender-api"]