# Build stage
FROM rust:latest AS builder

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy all source files
COPY Cargo.toml Cargo.lock* ./
COPY askama.toml ./
COPY src ./src
COPY templates ./templates

# Build application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates tzdata && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/gengoka-backend .

# Copy static files for admin panel
COPY static ./static

# Set environment
ENV TZ=Asia/Tokyo
ENV RUST_LOG=info

EXPOSE 8080

CMD ["./gengoka-backend"]
