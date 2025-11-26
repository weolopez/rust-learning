# Task 6: Deploy web server to Azure Container Instances

Date: 2025-11-26
Status: READY (after Task 5 image push)

Purpose
Deploy the rust-static-web-server Docker image to Azure Container Instances on port 8080, making the web app accessible to users on the internet.

Prerequisites
- Task 5 complete: Web server image pushed to ACR
- Image available: `acrrustapp.azurecr.io/rust-static-web-server:latest`

Deployment command

```bash
# Get ACR credentials
ACR_USER=$(az acr credential show --name acrrustapp --query username -o tsv)
ACR_PASS=$(az acr credential show --name acrrustapp --query "passwords[0].value" -o tsv)

# Deploy web container
az container create \
  --resource-group rg-rust-app \
  --name web-container \
  --image acrrustapp.azurecr.io/rust-static-web-server:latest \
  --os-type Linux \
  --cpu 1 \
  --memory 1 \
  --port 8080 \
  --environment-variables RUST_LOG="info" \
  --registry-username "$ACR_USER" \
  --registry-password "$ACR_PASS"
```

Alternative (one-liner):
```bash
az container create \
  --resource-group rg-rust-app \
  --name web-container \
  --image acrrustapp.azurecr.io/rust-static-web-server:latest \
  --os-type Linux \
  --cpu 1 --memory 1 --port 8080 \
  --registry-username acrrustapp \
  --registry-password "$(az acr credential show --name acrrustapp --query 'passwords[0].value' -o tsv)"
```

After deployment

1) Get web server FQDN
```bash
az container show --resource-group rg-rust-app --name web-container \
  --query ipAddress.fqdn -o tsv
```

Expected: `web-container.eastus.azurecontainer.io`

2) Access web app
```bash
# In browser or curl:
curl http://web-container.eastus.azurecontainer.io:8080
```

3) Check logs if needed
```bash
az container logs --resource-group rg-rust-app --name web-container
```

Deployment details
- Container name: web-container
- Image: acrrustapp.azurecr.io/rust-static-web-server:latest
- Port: 8080 (publicly exposed)
- CPU: 1 core
- Memory: 1 GB
- No environment variables required (static server)

Next step
- Task 7: Verify end-to-end deployment
  - Access web app FQDN:8080
  - Test CRUD operations via API
  - Test AI chat (calls to API with Gemini integration)
  - Verify cross-container communication

Cost
- Both containers running: ~$0.02–$0.15 per hour
- Delete when done:
  ```bash
  az container delete --resource-group rg-rust-app --name web-container --yes
  az container delete --resource-group rg-rust-app --name api-container --yes
  ```
