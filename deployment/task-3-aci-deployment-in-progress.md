# Task 3: Deploy API to ACI - In Progress

Date: 2025-11-26

Status: IN PROGRESS (commands executed, waiting for container to be ready)

Commands executed

1) Enabled admin credentials on ACR
```bash
az acr update -n acrrustapp --admin-enabled true
```

2) Deployment script executed with non-interactive flag
```bash
az container create \
  --resource-group rg-rust-app \
  --name api-container \
  --image acrrustapp.azurecr.io/rust-simple-rest-api:latest \
  --os-type Linux \
  --cpu 1 \
  --memory 1 \
  --ports 3000 \
  --environment-variables GEMINI_API_KEY="<key>" RUST_LOG="info" \
  --registry-username acrrustapp \
  --registry-password "<password>" \
  --no-wait
```

Container details (to be updated once deployment completes)
- Resource group: rg-rust-app
- Container name: api-container
- Image: acrrustapp.azurecr.io/rust-simple-rest-api:latest
- CPU: 1 core
- Memory: 1 GB
- Port: 3000
- Environment: GEMINI_API_KEY set, RUST_LOG=info

Next steps to verify deployment
```bash
# Check container status
az container show --resource-group rg-rust-app --name api-container --query "{Status:containers[0].instanceView.currentState.state, FQDN:ipAddress.fqdn}" -o table

# Check logs if running
az container logs --resource-group rg-rust-app --name api-container

# Test API endpoint (once FQDN is available)
curl http://<FQDN>:3000/items
```

Once container is running
- FQDN will be available: `api-container.eastus.azurecontainer.io`
- Update this document with the actual FQDN
- Proceed to Task 4: Update frontend config.js
