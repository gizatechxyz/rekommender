# Build stage
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src ./src

# Build dependencies - this is the caching Docker layer!
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install CA certificates and other runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/rekt-recommender-api /app/

# Set environment variables
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_LOG_LEVEL=normal

# Expose port
EXPOSE 8080

# Run the binary
CMD ["./rekt-recommender-api"]