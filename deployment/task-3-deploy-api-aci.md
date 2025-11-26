# Task 3: Deploy API to Azure Container Instances (ACI)

Date: 2025-11-26

Status: READY TO DEPLOY (awaiting GEMINI_API_KEY)

Purpose
Deploy the rust-simple-rest-api Docker image to Azure Container Instances so it's accessible from the internet and can be called by the web frontend.

Prerequisites
- Azure resource group `rg-rust-app` created ✅
- Azure Container Registry `acrrustapp` created ✅
- Docker image pushed to ACR: `acrrustapp.azurecr.io/rust-simple-rest-api:latest` ✅
- GEMINI_API_KEY (required for API to function)

Deployment script
Located: `scripts/deploy-api-to-aci.sh`

Usage (once you have GEMINI_API_KEY)
```bash
./scripts/deploy-api-to-aci.sh <YOUR_GEMINI_API_KEY>
```

Or set the environment variable:
```bash
export GEMINI_API_KEY="<YOUR_GEMINI_API_KEY>"
./scripts/deploy-api-to-aci.sh
```

What the script does
1. Retrieves ACR credentials (login server, username, password)
2. Creates an Azure Container Instance with:
   - Resource group: `rg-rust-app`
   - Container name: `api-container`
   - Image: `acrrustapp.azurecr.io/rust-simple-rest-api:latest`
   - CPU: 1 core
   - Memory: 1 GB
   - Port: 3000 (exposed)
   - Environment variables:
     - `GEMINI_API_KEY`: your provided API key
     - `RUST_LOG`: info
   - Registry credentials: automatically pulled from ACR
3. Retrieves and displays the container's FQDN (fully qualified domain name)

Output after deployment
- FQDN: e.g., `api-container.eastus.azurecontainer.io`
- IP address: public IP of the container
- API endpoint: `http://<FQDN>:3000`

Next steps after deployment
1. Save the FQDN output — you'll need it for task 4 (update frontend config)
2. Verify the API is running:
   ```bash
   curl http://<FQDN>:3000/items
   ```
3. Proceed to task 4: Update frontend config.js with the production API URL

Cost note
- ACI billing: charged per second the container is running
- Estimated cost for small test: ~$0.01–$0.10 per hour
- Delete container when done:
  ```bash
  az container delete --resource-group rg-rust-app --name api-container --yes
  ```

Troubleshooting
- Check logs: `az container logs --resource-group rg-rust-app --name api-container`
- Check status: `az container show --resource-group rg-rust-app --name api-container --query containers[0].instanceView.currentState`
- Delete and retry: `az container delete --resource-group rg-rust-app --name api-container --yes` then run deploy script again
