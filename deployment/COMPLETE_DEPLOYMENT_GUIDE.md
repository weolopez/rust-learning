# Complete Azure Deployment Guide - Ready to Execute

**Date**: 2025-11-26  
**Status**: Docker images built and pushed to ACR. Ready for manual ACI deployment.

---

## Quick Reference: ACR Images Ready

Both Docker images have been successfully built and pushed to Azure Container Registry:

```
acrrustapp.azurecr.io/rust-simple-rest-api:latest     (128 MB) - API server, port 3000
acrrustapp.azurecr.io/rust-static-web-server:latest   (122 MB) - Web UI, port 8080
```

---

## Step-by-Step Deployment

### 1️⃣ Get ACR Credentials (Required for both deployments)

```bash
# Store credentials in variables for reuse
ACR_USER=$(az acr credential show --name acrrustapp --query username -o tsv)
ACR_PASS=$(az acr credential show --name acrrustapp --query "passwords[0].value" -o tsv)

echo "ACR User: $ACR_USER"
echo "ACR Pass: $ACR_PASS"
```

---

### 2️⃣ Deploy API Container to ACI

```bash
az container create \
  --resource-group rg-rust-app \
  --name api-container \
  --image acrrustapp.azurecr.io/rust-simple-rest-api:latest \
  --os-type Linux \
  --cpu 1 \
  --memory 1 \
  --port 3000 \
  --environment-variables GEMINI_API_KEY="AIzaSyAU_RAK3xAWgH3vtITLZpTGMZpZvH8JTt4" RUST_LOG="info" \
  --registry-username "$ACR_USER" \
  --registry-password "$ACR_PASS"
```

**Expected output**: JSON response with provisioning state

**Check status**:
```bash
az container show --resource-group rg-rust-app --name api-container \
  --query "{Status:containers[0].instanceView.currentState.state, FQDN:ipAddress.fqdn, IP:ipAddress.ip}"
```

**Get API FQDN** (wait ~2-3 minutes, then run):
```bash
API_FQDN=$(az container show --resource-group rg-rust-app --name api-container --query ipAddress.fqdn -o tsv)
echo "API FQDN: $API_FQDN"
# Example output: api-container.eastus.azurecontainer.io
```

**Test API** (once running):
```bash
curl http://$API_FQDN:3000/items
```

---

### 3️⃣ Update Frontend Config with Production API URL

```bash
# Replace localhost with production FQDN
API_FQDN="api-container.eastus.azurecontainer.io"  # Use actual FQDN from step 2

# Update config.js
sed -i '' "s|localhost:3000|$API_FQDN:3000|g" rust-static-web-server/static/config.js

# Verify the change
cat rust-static-web-server/static/config.js | grep API_URL
```

**Expected output**: `const API_URL = "http://api-container.eastus.azurecontainer.io:3000";`

---

### 4️⃣ Rebuild Web Server Docker Image (with updated config.js)

```bash
# Rebuild the image (will include the updated config.js)
docker build -f rust-static-web-server/Dockerfile -t rust-static-web-server:updated .

# Tag for ACR
docker tag rust-static-web-server:updated acrrustapp.azurecr.io/rust-static-web-server:latest

# Push to ACR
docker push acrrustapp.azurecr.io/rust-static-web-server:latest
```

---

### 5️⃣ Deploy Web Server Container to ACI

```bash
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

**Get Web FQDN**:
```bash
WEB_FQDN=$(az container show --resource-group rg-rust-app --name web-container --query ipAddress.fqdn -o tsv)
echo "Web FQDN: $WEB_FQDN"
# Example output: web-container.eastus.azurecontainer.io
```

---

### 6️⃣ Verify End-to-End Deployment

```bash
# Check both containers are running
az container list --resource-group rg-rust-app \
  --query "[].{Name:name, Status:containers[0].instanceView.currentState.state, FQDN:ipAddress.fqdn}" \
  -o table

# Test API directly
curl http://$API_FQDN:3000/items

# Test creating an item via API
curl -X POST http://$API_FQDN:3000/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Item"}'

# Access web app
curl http://$WEB_FQDN:8080

# Check logs
az container logs --resource-group rg-rust-app --name api-container
az container logs --resource-group rg-rust-app --name web-container
```

---

## Complete One-Shot Script

Save as `deploy.sh` and run: `bash deploy.sh`

```bash
#!/bin/bash
set -e

echo "=== Azure Rust Full-Stack Deployment ==="

RG="rg-rust-app"
ACR_NAME="acrrustapp"

echo "Getting ACR credentials..."
ACR_USER=$(az acr credential show --name "$ACR_NAME" --query username -o tsv)
ACR_PASS=$(az acr credential show --name "$ACR_NAME" --query "passwords[0].value" -o tsv)

echo "Deploying API container..."
az container create \
  --resource-group "$RG" \
  --name api-container \
  --image "$ACR_NAME.azurecr.io/rust-simple-rest-api:latest" \
  --os-type Linux \
  --cpu 1 --memory 1 --port 3000 \
  --environment-variables GEMINI_API_KEY="AIzaSyAU_RAK3xAWgH3vtITLZpTGMZpZvH8JTt4" RUST_LOG="info" \
  --registry-username "$ACR_USER" \
  --registry-password "$ACR_PASS" \
  --no-wait

echo "Waiting 30 seconds for API to initialize..."
sleep 30

echo "Getting API FQDN..."
API_FQDN=$(az container show --resource-group "$RG" --name api-container --query ipAddress.fqdn -o tsv 2>/dev/null || echo "pending")
echo "API FQDN: $API_FQDN"

if [ "$API_FQDN" != "pending" ]; then
  echo "Updating web config with API FQDN..."
  sed -i '' "s|localhost:3000|$API_FQDN:3000|g" rust-static-web-server/static/config.js
  
  echo "Rebuilding web image..."
  docker build -f rust-static-web-server/Dockerfile -t rust-static-web-server:updated . > /dev/null 2>&1
  docker tag rust-static-web-server:updated "$ACR_NAME.azurecr.io/rust-static-web-server:latest"
  echo "Pushing web image..."
  docker push "$ACR_NAME.azurecr.io/rust-static-web-server:latest" > /dev/null 2>&1
  
  echo "Deploying web container..."
  az container create \
    --resource-group "$RG" \
    --name web-container \
    --image "$ACR_NAME.azurecr.io/rust-static-web-server:latest" \
    --os-type Linux \
    --cpu 1 --memory 1 --port 8080 \
    --environment-variables RUST_LOG="info" \
    --registry-username "$ACR_USER" \
    --registry-password "$ACR_PASS" \
    --no-wait
  
  echo ""
  echo "=== Deployment Complete ==="
  echo "API endpoint: http://$API_FQDN:3000"
  echo ""
  echo "Wait 2-3 minutes for web container to initialize..."
  sleep 30
  WEB_FQDN=$(az container show --resource-group "$RG" --name web-container --query ipAddress.fqdn -o tsv 2>/dev/null || echo "pending")
  echo "Web endpoint: http://$WEB_FQDN:8080"
else
  echo "API FQDN not yet available. Wait a few more minutes and try again."
fi
```

---

## Cleanup When Done

```bash
# Delete containers
az container delete --resource-group rg-rust-app --name api-container --yes
az container delete --resource-group rg-rust-app --name web-container --yes

# Optional: Delete entire resource group (keeps images in ACR)
az group delete --name rg-rust-app --yes

# Optional: Delete ACR entirely (removes images too)
az acr delete --name acrrustapp --yes
```

---

## Troubleshooting

| Issue | Solution |
|-------|----------|
| `az container create` hangs | Use `--no-wait` flag or run as background process |
| Container won't start | Check logs: `az container logs --resource-group rg-rust-app --name api-container` |
| FQDN not assigned | Wait 2-3 minutes, containers take time to initialize |
| API not reachable | Verify security group rules allow port 3000; check container status |
| Web app shows blank | Check browser console for errors; verify config.js has correct API_URL |
| AI chat doesn't work | Verify GEMINI_API_KEY is correctly set; check API logs |

---

## Estimated Costs

- **Per container**: ~$0.013 per hour (1 CPU, 1 GB memory)
- **Both running**: ~$0.025–$0.03 per hour
- **Storage (ACR)**: ~$5/month for Basic tier (includes free storage)
- **Example**: 24 hours running = ~$0.60

---

## What's Deployed

| Component | Image | Port | Purpose |
|-----------|-------|------|---------|
| API | rust-simple-rest-api:latest | 3000 | REST API with Gemini integration |
| Web | rust-static-web-server:latest | 8080 | Static web UI (HTML/JS/CSS) |

---

## Next Steps After Deployment

1. Open web app: `http://<web-fqdn>:8080`
2. Create items via the web UI
3. Test AI chat feature
4. Monitor container logs for issues
5. Clean up resources when done

---

**Ready to deploy! Run the commands above or use the one-shot script.**
