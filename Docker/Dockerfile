# Multi-stage build for optimized production image
FROM rust:1.89-slim as builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libpq-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Solana CLI
RUN sh -c "$(curl -sSfL https://release.solana.com/v1.17.0/install)"
ENV PATH="/root/.local/share/solana/install/active_release/bin:$PATH"

# Install Anchor CLI
RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
RUN avm install latest
RUN avm use latest

# Create app directory
WORKDIR /app

# Copy workspace configuration
COPY Cargo.toml Cargo.lock ./
COPY engine/Cargo.toml ./engine/

# Copy source code
COPY engine/src ./engine/src/
COPY programs ./programs/
COPY shared ./shared/

# Build dependencies first (for better caching)
RUN cargo build --release --bin defi-arbitrage-engine

# Build Anchor programs
RUN anchor build

# Production stage
FROM debian:bookworm-slim as runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN groupadd -r arbitrage && useradd -r -g arbitrage arbitrage

# Create app directory
WORKDIR /app

# Copy binary from builder stage
COPY --from=builder /app/target/release/defi-arbitrage-engine /app/
COPY --from=builder /app/target/deploy /app/programs/

# Copy configuration files
COPY config.toml /app/
COPY migrations /app/migrations/

# Create directories for logs and data
RUN mkdir -p /app/logs /app/data && \
    chown -R arbitrage:arbitrage /app

# Switch to non-root user
USER arbitrage

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

# Expose ports
EXPOSE 8080 8081 9091

# Set environment variables
ENV RUST_LOG=info,defi_arbitrage_engine=debug
ENV RUST_BACKTRACE=1

# Run the application
CMD ["./defi-arbitrage-engine", "run"]