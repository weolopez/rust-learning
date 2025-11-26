# Simple Dockerfile: copies pre-built binary from local system
# Assumes cargo build --release has been run locally on the host

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy pre-built binary from host (built locally via cargo build --release)
COPY rust-simple-rest-api/target/release/rust-simple-rest-api /app/rust-simple-rest-api

EXPOSE 3000
ENV RUST_LOG=info
CMD ["/app/rust-simple-rest-api"]
