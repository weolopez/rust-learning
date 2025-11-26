#!/usr/bin/env bash
set -euo pipefail

# Usage: build-and-push-api.sh --acr-name <acrName> [--tag <tag>]
ACR_NAME="acrrustapp"
TAG="latest"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --acr-name) ACR_NAME="$2"; shift 2 ;;
    --tag) TAG="$2"; shift 2 ;;
    -h|--help) echo "Usage: $0 --acr-name <name> [--tag <tag>]"; exit 0 ;;
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
IMAGE_NAME="$ACR_LOGIN_SERVER/rust-simple-rest-api:$TAG"

echo "Building Docker image: $IMAGE_NAME (build context: repo root)"
docker build -f rust-simple-rest-api/Dockerfile -t "$IMAGE_NAME" .

echo "Pushing $IMAGE_NAME"
docker push "$IMAGE_NAME"

echo "Pushed: $IMAGE_NAME"
