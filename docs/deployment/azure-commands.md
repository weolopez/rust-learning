# Azure CLI commands executed (chronological)

Date: 2025-11-26

Purpose: Record exact commands and observed outputs so future runs are reproducible.

1) Initial verification (resources did not exist)

```bash
az group show --name rg-rust-app -o json
# Observed: (ResourceGroupNotFound) Resource group 'rg-rust-app' could not be found.

az acr show --name acrrustapp -o json
# Observed: The resource with name 'acrrustapp' and type 'Microsoft.ContainerRegistry/registries' could not be found in subscription
```

2) Create resources using the idempotent script

```bash
./scripts/create-azure-resources.sh --rg rg-rust-app --location eastus --acr acrrustapp --sku Basic
# Script behavior: checks az login, creates RG if missing, creates ACR if missing.
```

3) Post-creation verification

```bash
az group show --name rg-rust-app -o json
# Observed: JSON for the resource group with provisioningState: Succeeded

az acr list --resource-group rg-rust-app -o table
# Observed example row:
# NAME        RESOURCE GROUP    LOCATION    SKU    LOGIN SERVER           CREATION DATE         ADMIN ENABLED
# acrrustapp  rg-rust-app       eastus      Basic  acrrustapp.azurecr.io  2025-11-26T14:10:27Z  False

az acr show --name acrrustapp --query loginServer -o tsv
# Observed output: acrrustapp.azurecr.io
```

Helpful follow-ups

- Set subscription explicitly before running creates (if you have multiple subscriptions):

```bash
az account set --subscription "<your-subscription-id-or-name>"
```

- To push images to ACR:

```bash
az acr login --name acrrustapp
docker tag rust-simple-rest-api:local acrrustapp.azurecr.io/rust-simple-rest-api:latest
docker push acrrustapp.azurecr.io/rust-simple-rest-api:latest
```

Security note
- Keep secrets out of scripts and repos. Use Key Vault or pass secrets as environment variables during deployment.
