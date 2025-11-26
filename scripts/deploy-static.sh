#!/usr/bin/env bash
# Quick script to rebuild and redeploy the static web server to ACI
# Usage: ./scripts/deploy-static.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(dirname "$SCRIPT_DIR")"

# Configuration
RG_NAME="rg-rust-app"
ACR_NAME="acrrustapp"
CONTAINER_NAME="web-container"
DNS_LABEL="rust-web-demo"
PORT=8080

cd "$REPO_ROOT"

echo "============================================"
echo "  Deploying Static Web Server to ACI"
echo "============================================"
echo ""

# Step 1: Login to ACR
echo "[1/4] Logging into ACR..."
az acr login --name "$ACR_NAME"

ACR_LOGIN_SERVER=$(az acr show --name "$ACR_NAME" --query loginServer -o tsv)
IMAGE_NAME="$ACR_LOGIN_SERVER/rust-static-web-server:latest"

# Step 2: Build image
echo ""
echo "[2/4] Building Docker image (linux/amd64)..."
docker build --platform linux/amd64 \
  -f rust-static-web-server/Dockerfile \
  -t "$IMAGE_NAME" \
  .

# Step 3: Push to ACR
echo ""
echo "[3/4] Pushing image to ACR..."
docker push "$IMAGE_NAME"

# Step 4: Redeploy container
echo ""
echo "[4/4] Redeploying container to ACI..."

# Delete existing container
echo "Deleting existing container..."
az container delete \
  --resource-group "$RG_NAME" \
  --name "$CONTAINER_NAME" \
  --yes 2>/dev/null || true

# Get ACR credentials
ACR_USERNAME=$(az acr credential show --name "$ACR_NAME" --query username -o tsv)
ACR_PASSWORD=$(az acr credential show --name "$ACR_NAME" --query "passwords[0].value" -o tsv)

# Create new container
echo "Creating new container..."
az container create \
  --resource-group "$RG_NAME" \
  --name "$CONTAINER_NAME" \
  --image "$IMAGE_NAME" \
  --dns-name-label "$DNS_LABEL" \
  --ports "$PORT" \
  --os-type Linux \
  --cpu 1 \
  --memory 1 \
  --environment-variables RUST_LOG="info" \
  --registry-login-server "$ACR_LOGIN_SERVER" \
  --registry-username "$ACR_USERNAME" \
  --registry-password "$ACR_PASSWORD" \
  --restart-policy OnFailure

# Get final status
FQDN=$(az container show --resource-group "$RG_NAME" --name "$CONTAINER_NAME" --query ipAddress.fqdn -o tsv)

echo ""
echo "============================================"
echo "  Deployment Complete!"
echo "============================================"
echo ""
echo "  Web URL: http://${FQDN}:${PORT}"
echo ""
echo "  View logs: az container logs --resource-group $RG_NAME --name $CONTAINER_NAME"
echo ""
