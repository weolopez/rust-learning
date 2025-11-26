# Manual ACI Deployment Steps

If the automated `az container create` command hangs or times out, use these alternative manual steps:

## Step 1: Get ACR Credentials
```bash
ACR_USER=$(az acr credential show --name acrrustapp --query username -o tsv)
ACR_PASS=$(az acr credential show --name acrrustapp --query "passwords[0].value" -o tsv)
echo "ACR User: $ACR_USER"
echo "ACR Pass: $ACR_PASS"
```

## Step 2: Try Simpler Deployment (without registry credentials first)
```bash
az container create \
  --resource-group rg-rust-app \
  --name api-container \
  --image acrrustapp.azurecr.io/rust-simple-rest-api:latest \
  --os-type Linux \
  --cpu 1 \
  --memory 1 \
  --port 3000 \
  --environment-variables GEMINI_API_KEY="AIzaSyAU_RAK3xAWgH3vtITLZpTGMZpZvH8JTt4"
```

If that fails with auth error, add credentials:
```bash
--registry-username "$ACR_USER" \
--registry-password "$ACR_PASS"
```

## Step 3: Check Status (wait 2-5 minutes then run)
```bash
az container show --resource-group rg-rust-app --name api-container \
  --query "{Status:containers[0].instanceView.currentState.state, FQDN:ipAddress.fqdn, IP:ipAddress.ip}" \
  -o table
```

## Step 4: Once Running, Get Full FQDN
```bash
az container show --resource-group rg-rust-app --name api-container \
  --query ipAddress.fqdn -o tsv
```

Expected output: `api-container.eastus.azurecontainer.io`

## Step 5: Test API
```bash
curl http://api-container.eastus.azurecontainer.io:3000/items
```

## Troubleshooting

If deployment fails:
```bash
# Check resource group
az group show --name rg-rust-app

# Check subscription limits
az account show

# Check ACR credentials are enabled
az acr show --name acrrustapp --query adminUserEnabled

# Check if container exists
az container list --resource-group rg-rust-app

# Delete and retry
az container delete --resource-group rg-rust-app --name api-container --yes
```

Once you have the FQDN, save it for Task 4.
