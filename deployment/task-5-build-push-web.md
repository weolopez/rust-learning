# Task 5: Build and push web server Docker image

Date: 2025-11-26
Status: READY (after Task 4 config.js update)

Purpose
Containerize the rust-static-web-server with the updated production config.js and push it to ACR for deployment to Azure.

Dockerfile for web server
Create: `rust-static-web-server/Dockerfile`

Content (simplified, copies pre-built or source files):
```dockerfile
# Static web server runtime
FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy static files including updated config.js
COPY rust-static-web-server/static ./static

# If binary exists locally, copy it; otherwise use a simple HTTP server
# For now, assume binary is built locally
COPY rust-static-web-server/target/release/rust-static-web-server /app/rust-static-web-server

EXPOSE 8080
ENV RUST_LOG=info
CMD ["/app/rust-static-web-server"]
```

Steps

1) Create Dockerfile (if not already present)
```bash
# Check if Dockerfile exists
ls rust-static-web-server/Dockerfile || echo "Not found, create it"
```

2) Build local binary (if not already built)
```bash
cd rust-static-web-server
cargo build --release
cd ..
```

3) Build Docker image
```bash
docker build -f rust-static-web-server/Dockerfile -t rust-static-web-server:local .
```

4) Tag for ACR
```bash
docker tag rust-static-web-server:local acrrustapp.azurecr.io/rust-static-web-server:latest
```

5) Push to ACR
```bash
# Ensure logged in
az acr login --name acrrustapp

# Push
docker push acrrustapp.azurecr.io/rust-static-web-server:latest
```

6) Verify pushed
```bash
az acr repository show-tags --name acrrustapp --repository rust-static-web-server
```

Output
- Image pushed to: `acrrustapp.azurecr.io/rust-static-web-server:latest`
- Ready for Task 6 deployment

Next step
- Task 6: Deploy web server to ACI on port 8080
