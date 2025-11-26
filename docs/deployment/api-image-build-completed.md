# API image build and push - Completed

Date: 2025-11-26

Status: COMPLETED

Commands executed

1) Built Docker image locally
```bash
cd /Users/weo/Development/rust/hello
docker build -f rust-simple-rest-api/Dockerfile -t rust-simple-rest-api:local .
```

Result: Image built successfully with ID `c2f0a3c58e59`, size 128MB

2) Tagged image for ACR
```bash
docker tag rust-simple-rest-api:local acrrustapp.azurecr.io/rust-simple-rest-api:latest
```

Result: Image tagged as `acrrustapp.azurecr.io/rust-simple-rest-api:latest`

3) Logged in to ACR
```bash
az acr login --name acrrustapp
```

Result: Successfully authenticated to ACR

4) Pushed image to ACR
```bash
docker push acrrustapp.azurecr.io/rust-simple-rest-api:latest
```

Result: Image pushed to acrrustapp.azurecr.io

Dockerfile details
- Base image: debian:bullseye-slim (128MB)
- Runtime: Copies pre-built binary from host (`rust-simple-rest-api/target/release/rust-simple-rest-api`)
- Exposes port 3000
- Sets RUST_LOG=info environment variable
- Entry point: `/app/rust-simple-rest-api`

Image details
- Local tag: `rust-simple-rest-api:local`
- ACR tag: `acrrustapp.azurecr.io/rust-simple-rest-api:latest`
- Image ID: c2f0a3c58e59
- Size: 128MB

Next steps
- Task 3: Deploy API to Azure Container Instances (ACI)
- Will deploy using: `az container create --resource-group rg-rust-app --name api-container --image acrrustapp.azurecr.io/rust-simple-rest-api:latest --cpu 1 --memory 1 --port 3000 --environment-variables GEMINI_API_KEY=<key>`
