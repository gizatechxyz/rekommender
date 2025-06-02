FROM rust:1.75 as builder
WORKDIR /app

RUN rustup toolchain install nightly-2025-04-06 \
    && rustup default nightly-2025-04-06

COPY Cargo.toml Cargo.lock ./
COPY src ./src

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
EXPOSE 8080
CMD ["./rekt-recommender-api"]
