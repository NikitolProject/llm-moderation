# Build stage
FROM rust:1.91-bookworm AS builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY models/Cargo.toml ./models/

# Create dummy files to cache dependencies
RUN mkdir -p src models/src && \
    echo "fn main() {}" > src/main.rs && \
    echo "pub fn dummy() {}" > src/lib.rs && \
    echo "pub fn dummy() {}" > models/src/lib.rs

# Build dependencies only
RUN cargo build --release && \
    rm -rf src models/src

# Copy actual source code
COPY src ./src
COPY models/src ./models/src
COPY migrations ./migrations

# Build the application
RUN touch src/main.rs src/lib.rs models/src/lib.rs && \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/llm_moderation /app/llm_moderation

# Copy migrations for runtime
COPY migrations ./migrations

# Create non-root user
RUN useradd -r -s /bin/false appuser && \
    chown -R appuser:appuser /app

USER appuser

ENV PORT=8000
EXPOSE 8000

CMD ["./llm_moderation"]
