# Docker build and push commands for rust-simple-rest-api

Date: 2025-11-26

Status: Image build requires internet access (Docker can't reach registry). Commands documented here for manual execution when internet is available.

Prerequisites
- Docker daemon running and logged in to Docker Hub
- Binary pre-built locally: `cargo build --release` (already done)
- Azure CLI logged in and ACR created: `az acr login --name acrrustapp`

Docker build, tag, and push commands

1) Build Docker image locally
```bash
cd /Users/weo/Development/rust/hello
docker build -f rust-simple-rest-api/Dockerfile -t rust-simple-rest-api:local .
```

Expected: Image built successfully with tag `rust-simple-rest-api:local`

2) Tag image for ACR
```bash
ACR_LOGIN_SERVER=$(az acr list --resource-group rg-rust-app -o json --query "[?name=='acrrustapp'].loginServer | [0]" -o tsv)
docker tag rust-simple-rest-api:local ${ACR_LOGIN_SERVER}/rust-simple-rest-api:latest
# Or directly:
docker tag rust-simple-rest-api:local acrrustapp.azurecr.io/rust-simple-rest-api:latest
```

Expected: Image tagged as `acrrustapp.azurecr.io/rust-simple-rest-api:latest`

3) Login to ACR
```bash
az acr login --name acrrustapp
```

Expected: Successfully authenticated to ACR

4) Push image to ACR
```bash
docker push acrrustapp.azurecr.io/rust-simple-rest-api:latest
```

Expected: Image pushed and available in ACR for ACI deployment

Dockerfile notes
- Uses pre-built binary from `rust-simple-rest-api/target/release/rust-simple-rest-api`
- Based on `debian:bullseye-slim` for small runtime image
- Exposes port 3000 and sets `RUST_LOG=info`

Next steps
- Once image is pushed to ACR, proceed to task 3: Deploy API to Azure Container Instances
