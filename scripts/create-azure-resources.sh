#!/usr/bin/env bash
# Creates an Azure Resource Group and an Azure Container Registry (ACR)
# Safe, idempotent: checks for existing resources before creating them.

set -euo pipefail

RG_NAME="rg-rust-app"
LOCATION="eastus"
ACR_NAME="acrrustapp"
ACR_SKU="Basic"

usage() {
  cat <<EOF
Usage: $0 [--rg <name>] [--location <loc>] [--acr <name>] [--sku <sku>]

Defaults:
  --rg      ${RG_NAME}
  --location ${LOCATION}
  --acr     ${ACR_NAME}
  --sku     ${ACR_SKU}

This script requires the Azure CLI (az) and that you're logged in (az login).
It will create the resource group and ACR if they do not already exist.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --rg) RG_NAME="$2"; shift 2 ;;
    --location) LOCATION="$2"; shift 2 ;;
    --acr) ACR_NAME="$2"; shift 2 ;;
    --sku) ACR_SKU="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown arg: $1"; usage; exit 1 ;;
  esac
done

echo "Checking Azure login..."
if ! az account show > /dev/null 2>&1; then
  echo "You are not logged in. Run 'az login' first." >&2
  exit 1
fi

echo "Ensure resource group: $RG_NAME in $LOCATION"
if az group show --name "$RG_NAME" > /dev/null 2>&1; then
  echo "Resource group '$RG_NAME' already exists."
else
  az group create --name "$RG_NAME" --location "$LOCATION"
  echo "Created resource group '$RG_NAME'."
fi

echo "Ensure ACR: $ACR_NAME (sku: $ACR_SKU)"
if az acr show --name "$ACR_NAME" > /dev/null 2>&1; then
  echo "ACR '$ACR_NAME' already exists."
else
  az acr create --resource-group "$RG_NAME" --name "$ACR_NAME" --sku "$ACR_SKU"
  echo "Created ACR '$ACR_NAME'."
fi

echo "Done. ACR login server: $(az acr show --name "$ACR_NAME" --query loginServer -o tsv)"
