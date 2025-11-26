#!/usr/bin/env bash
# Non-interactive ACI deployment

set -euo pipefail

RG="rg-rust-app"
IMAGE="acrrustapp.azurecr.io/rust-simple-rest-api:latest"
CONTAINER="api-container"
GEMINI_KEY="AIzaSyAU_RAK3xAWgH3vtITLZpTGMZpZvH8JTt4"

echo "Getting ACR credentials..."
ACR_USER=$(az acr credential show --name acrrustapp --query username -o tsv)
ACR_PASS=$(az acr credential show --name acrrustapp --query "passwords[0].value" -o tsv)

echo "Creating container..."
az container create \
  --resource-group "$RG" \
  --name "$CONTAINER" \
  --image "$IMAGE" \
  --os-type Linux \
  --cpu 1 \
  --memory 1 \
  --ports 3000 \
  --environment-variables GEMINI_API_KEY="$GEMINI_KEY" RUST_LOG="info" \
  --registry-username "$ACR_USER" \
  --registry-password "$ACR_PASS" \
  --no-wait

echo "Container creation initiated (no-wait). Checking status..."
sleep 15
az container show --resource-group "$RG" --name "$CONTAINER" --query "{Name:name, Status:containers[0].instanceView.currentState.state, FQDN:ipAddress.fqdn}" -o table || echo "Container still initializing..."
