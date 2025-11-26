#!/usr/bin/env bash
# Clean up all Azure resources for the Rust app deployment
set -euo pipefail

RG_NAME="rg-rust-app"
FORCE=false

usage() {
  cat <<EOF
Usage: $0 [options]

Options:
  --rg <name>   Resource group name (default: $RG_NAME)
  --force       Skip confirmation prompt
  -h, --help    Show this help message

This script deletes the entire resource group and all resources within it.
EOF
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --rg) RG_NAME="$2"; shift 2 ;;
    --force) FORCE=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown arg: $1"; usage; exit 1 ;;
  esac
done

echo "This will delete the resource group '$RG_NAME' and ALL resources within it:"
echo ""

# List resources in the group
az resource list --resource-group "$RG_NAME" --query "[].{Name:name, Type:type}" -o table 2>/dev/null || echo "  (Resource group may not exist)"

echo ""

if [ "$FORCE" = false ]; then
  read -p "Are you sure you want to delete all these resources? (yes/no): " CONFIRM
  if [ "$CONFIRM" != "yes" ]; then
    echo "Aborted."
    exit 0
  fi
fi

echo "Deleting resource group '$RG_NAME'..."
az group delete --name "$RG_NAME" --yes --no-wait

echo ""
echo "Deletion initiated. Resources will be removed in the background."
echo "Check status: az group show --name $RG_NAME"
