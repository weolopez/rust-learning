# Multi-stage Dockerfile for Azure Container Instances (requires linux/amd64)
# Stage 1: Build the Rust binary
FROM --platform=linux/amd64 rust:1.83-slim-bullseye AS builder

WORKDIR /build

# Install dependencies for building
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy just this crate's Cargo.toml first
COPY rust-simple-rest-api/Cargo.toml ./rust-simple-rest-api/Cargo.toml

# Copy the dependency crate (rust-gemini-llm-client)
COPY rust-gemini-llm-client ./rust-gemini-llm-client

# Copy the source code
COPY rust-simple-rest-api/src ./rust-simple-rest-api/src

# Build the release binary
WORKDIR /build/rust-simple-rest-api
RUN cargo build --release

# Stage 2: Create the runtime image
FROM --platform=linux/amd64 debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the built binary from builder stage
COPY --from=builder /build/rust-simple-rest-api/target/release/rust-simple-rest-api /app/rust-simple-rest-api

EXPOSE 3000
ENV RUST_LOG=info
CMD ["/app/rust-simple-rest-api"]
