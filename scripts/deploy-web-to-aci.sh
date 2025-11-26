#!/usr/bin/env bash
# Deploy rust-static-web-server to Azure Container Instances
set -euo pipefail

RG_NAME="rg-rust-app"
ACR_NAME="acrrustapp"
CONTAINER_NAME="web-container"
DNS_LABEL="rust-web-demo"
IMAGE_NAME="acrrustapp.azurecr.io/rust-static-web-server:latest"
PORT=8080
CPU="1"
MEMORY="1"

usage() {
  cat <<EOF
Usage: $0 [options]

Options:
  --rg <name>         Resource group name (default: $RG_NAME)
  --container <name>  Container name (default: $CONTAINER_NAME)
  --dns <label>       DNS name label (default: $DNS_LABEL)
  -h, --help          Show this help message
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --rg) RG_NAME="$2"; shift 2 ;;
    --container) CONTAINER_NAME="$2"; shift 2 ;;
    --dns) DNS_LABEL="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown arg: $1"; usage; exit 1 ;;
  esac
done

echo "Deploying $IMAGE_NAME to ACI..."
echo "  Resource Group: $RG_NAME"
echo "  Container Name: $CONTAINER_NAME"
echo "  DNS Label: $DNS_LABEL"
echo "  Port: $PORT"

# Get ACR credentials
ACR_LOGIN_SERVER=$(az acr show --name "$ACR_NAME" --query loginServer -o tsv)
ACR_USERNAME=$(az acr credential show --name "$ACR_NAME" --query username -o tsv)
ACR_PASSWORD=$(az acr credential show --name "$ACR_NAME" --query "passwords[0].value" -o tsv)

echo "Creating container instance..."
az container create \
  --resource-group "$RG_NAME" \
  --name "$CONTAINER_NAME" \
  --image "$IMAGE_NAME" \
  --dns-name-label "$DNS_LABEL" \
  --ports "$PORT" \
  --os-type Linux \
  --cpu "$CPU" \
  --memory "$MEMORY" \
  --environment-variables RUST_LOG="info" \
  --registry-login-server "$ACR_LOGIN_SERVER" \
  --registry-username "$ACR_USERNAME" \
  --registry-password "$ACR_PASSWORD" \
  --restart-policy OnFailure

echo ""
echo "Retrieving container details..."
az container show \
  --resource-group "$RG_NAME" \
  --name "$CONTAINER_NAME" \
  --query "{FQDN:ipAddress.fqdn, IP:ipAddress.ip, Port:containers[0].ports[0].port, Status:containers[0].instanceView.currentState.state}" \
  -o table

FQDN=$(az container show --resource-group "$RG_NAME" --name "$CONTAINER_NAME" --query ipAddress.fqdn -o tsv)
echo ""
echo "Web app URL: http://${FQDN}:${PORT}"
echo ""
echo "To check logs:"
echo "  az container logs --resource-group $RG_NAME --name $CONTAINER_NAME"
echo ""
echo "To delete container:"
echo "  az container delete --resource-group $RG_NAME --name $CONTAINER_NAME --yes"
