#!/usr/bin/env bash
# Deploy rust-simple-rest-api to Azure Container Instances

set -euo pipefail

RG_NAME="rg-rust-app"
ACR_NAME="acrrustapp"
CONTAINER_NAME="api-container"
IMAGE_NAME="acrrustapp.azurecr.io/rust-simple-rest-api:latest"
PORT=3000
CPU="1"
MEMORY="1"
LOCATION="eastus"

# Optional: accept GEMINI_API_KEY as argument or env var
GEMINI_API_KEY="${1:-${GEMINI_API_KEY:-}}"

if [ -z "$GEMINI_API_KEY" ]; then
  echo "Error: GEMINI_API_KEY not provided" >&2
  echo "Usage: $0 <gemini-api-key>" >&2
  echo "Or set GEMINI_API_KEY environment variable" >&2
  exit 1
fi

echo "Deploying $IMAGE_NAME to ACI..."
echo "  Resource Group: $RG_NAME"
echo "  Container Name: $CONTAINER_NAME"
echo "  Port: $PORT"
echo "  CPU: $CPU, Memory: ${MEMORY}GB"

# Get ACR credentials for ACI pull
ACR_LOGIN_SERVER=$(az acr show --name "$ACR_NAME" --query loginServer -o tsv)
ACR_USERNAME=$(az acr credential show --name "$ACR_NAME" --query username -o tsv)
ACR_PASSWORD=$(az acr credential show --name "$ACR_NAME" --query "passwords[0].value" -o tsv)

echo "Creating container instance..."
az container create \
  --resource-group "$RG_NAME" \
  --name "$CONTAINER_NAME" \
  --image "$IMAGE_NAME" \
  --cpu "$CPU" \
  --memory "$MEMORY" \
  --port "$PORT" \
  --protocol TCP \
  --environment-variables GEMINI_API_KEY="$GEMINI_API_KEY" RUST_LOG="info" \
  --registry-login-server "$ACR_LOGIN_SERVER" \
  --registry-username "$ACR_USERNAME" \
  --registry-password "$ACR_PASSWORD" \
  --restart-policy OnFailure

echo "Retrieving container details..."
az container show \
  --resource-group "$RG_NAME" \
  --name "$CONTAINER_NAME" \
  --query "{FQDN:ipAddress.fqdn, IP:ipAddress.ip, Port:containers[0].ports[0].port, Status:containers[0].instanceView.currentState.state}" \
  -o table

echo ""
echo "API endpoint: http://$(az container show --resource-group "$RG_NAME" --name "$CONTAINER_NAME" --query ipAddress.fqdn -o tsv):${PORT}"
echo ""
echo "To check logs:"
echo "  az container logs --resource-group $RG_NAME --name $CONTAINER_NAME"
echo ""
echo "To delete container:"
echo "  az container delete --resource-group $RG_NAME --name $CONTAINER_NAME --yes"
