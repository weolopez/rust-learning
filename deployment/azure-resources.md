# Azure resource verification and details

Date: 2025-11-26

Summary
- Resource group `rg-rust-app`: NOT FOUND
- Container Registry `acrrustapp`: NOT FOUND

Status after creation attempt
- Resource group `rg-rust-app`: FOUND
	- id: /subscriptions/869d8dbb-908a-4e51-ac3e-fa38d2d2cedd/resourceGroups/rg-rust-app
	- location: eastus
- Container Registry `acrrustapp`: FOUND
	- login server: acrrustapp.azurecr.io
	- sku: Basic
	- resource group: rg-rust-app
	- creation date: 2025-11-26T14:10:27Z

Commands used to verify
```bash
az group show --name rg-rust-app -o json
az acr show --name acrrustapp -o json
```

Observed output
- Resource group check returned: (ResourceGroupNotFound) Resource group 'rg-rust-app' could not be found.
- ACR check returned: The resource with name 'acrrustapp' and type 'Microsoft.ContainerRegistry/registries' could not be found in subscription.

Next steps / How to create
1. Ensure you are logged in to Azure and have the desired subscription selected:
```bash
az login
az account set --subscription "<your-subscription-id-or-name>"
```

2. Create the resource group and ACR using the included idempotent script:
```bash
./scripts/create-azure-resources.sh --rg rg-rust-app --location eastus --acr acrrustapp --sku Basic
```

3. Verify the resources were created:
```bash
az group show --name rg-rust-app -o json
az acr show --name acrrustapp -o json
az acr show --name acrrustapp --query loginServer -o tsv
```

4. After creation, note these values for deployment:
- Resource group name: `rg-rust-app`
- ACR name: `acrrustapp`
- ACR login server: output of `az acr show --name acrrustapp --query loginServer -o tsv`

Security note
- Do not commit secrets or API keys. Use Azure Key Vault or pass secrets as environment variables when deploying containers.
