#!/usr/bin/env bash
# Build and push rust-static-web-server image to ACR
set -euo pipefail

ACR_NAME="acrrustapp"
TAG="latest"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --acr-name) ACR_NAME="$2"; shift 2 ;;
    --tag) TAG="$2"; shift 2 ;;
    -h|--help) echo "Usage: $0 [--acr-name <name>] [--tag <tag>]"; exit 0 ;;
    *) echo "Unknown arg: $1"; exit 1 ;;
  esac
done

if ! command -v az >/dev/null 2>&1; then
  echo "az CLI not found. Install Azure CLI and login (az login)." >&2
  exit 1
fi

echo "Logging into ACR: $ACR_NAME"
az acr login --name "$ACR_NAME"

ACR_LOGIN_SERVER=$(az acr show --name "$ACR_NAME" --query loginServer -o tsv)
IMAGE_NAME="$ACR_LOGIN_SERVER/rust-static-web-server:$TAG"

echo "Building Docker image: $IMAGE_NAME (platform: linux/amd64)"
docker build --platform linux/amd64 -f rust-static-web-server/Dockerfile -t "$IMAGE_NAME" .

echo "Pushing $IMAGE_NAME"
docker push "$IMAGE_NAME"

echo "Pushed: $IMAGE_NAME"
