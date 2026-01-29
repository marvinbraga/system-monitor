# Multi-stage build for system-monitor collector
# Stage 1: Build stage
FROM rust:latest as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy workspace files
COPY Cargo.toml Cargo.lock ./

# Copy all workspace members
COPY collector ./collector
COPY shared ./shared
COPY tui-client ./tui-client

# Build the collector in release mode
RUN cargo build --release --package collector

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r monitor && useradd -r -g monitor monitor

# Create app directory
WORKDIR /app

# Create data directory
RUN mkdir -p /data && chown monitor:monitor /data

# Copy binary from builder
COPY --from=builder /app/target/release/collector /app/collector

# Copy any configuration files if they exist
# COPY collector/config/ /app/config/

# Set ownership
RUN chown -R monitor:monitor /app

# Switch to non-root user
USER monitor

# Expose port
EXPOSE 8080

# Environment variables with defaults
ENV RUST_LOG=info \
    DATABASE_URL=/data/system-monitor.db \
    HOST=0.0.0.0 \
    PORT=8080 \
    ANOMALY_DETECTION_ENABLED=true \
    ANOMALY_LOG_PATH=/data/anomalies.log \
    COLLECTION_INTERVAL_SECS=5

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Run the collector
CMD ["/app/collector"]
