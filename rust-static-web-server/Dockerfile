# Multi-stage Dockerfile for rust-static-web-server
# Builds for amd64 to run on Azure Container Instances

# Stage 1: Build the application
FROM --platform=linux/amd64 rust:1.83-slim-bullseye AS builder

WORKDIR /build

# Copy only the web server code (no dependencies on other workspaces)
COPY rust-static-web-server/Cargo.toml ./rust-static-web-server/Cargo.toml
COPY rust-static-web-server/src ./rust-static-web-server/src

WORKDIR /build/rust-static-web-server

# Build release version
RUN cargo build --release

# Stage 2: Create slim runtime image
FROM --platform=linux/amd64 debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy static files (including updated config.js)
COPY rust-static-web-server/static ./static

# Copy binary from builder
COPY --from=builder /build/rust-static-web-server/target/release/rust-static-web-server /app/rust-static-web-server

EXPOSE 8080
ENV RUST_LOG=info
CMD ["/app/rust-static-web-server"]
