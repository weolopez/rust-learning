# Azure Deployment Tasks - Detailed Explanation

This document outlines the detailed steps for deploying the Rust web application and API to Azure Container Instances, with explanations for each task's purpose.

## 1. 🔄 IN PROGRESS:  Create Azure Resource Group and Container Registry (CLI)
**Why:** Establishing the foundational Azure infrastructure. The resource group provides logical grouping and access control for all Azure resources, while the container registry (ACR) is required to store and distribute Docker images in Azure's ecosystem.

**Technical Details:** 
- Resource Group: `rg-rust-app` in `eastus` region
- Container Registry: `acrrustapp` with Basic SKU (sufficient for small scale)
- All resources will be deployed within this resource group for easy management and cleanup

> Quick script: `scripts/create-azure-resources.sh` provides an idempotent, ready-to-run Azure CLI workflow to create the resource group and ACR. Run after `az login`.

## 2. 🔄 IN PROGRESS: Build and push API image to ACR
**Why:** Containerizing the Rust API server to make it deployable on Azure. The API must be packaged with all dependencies and accessible via Docker to run consistently across environments.

**Technical Details:**
- Build Docker image for `rust-simple-rest-api`
- Use multi-stage build to minimize image size (builder stage for compilation, runtime stage for execution)
- Push to ACR so Azure services can pull the image
- This enables the API to run in a containerized environment with proper networking

## 3. ⏳ PENDING: Deploy API container to ACI and retrieve public IP
**Why:** Making the API server accessible from the internet and from the web application. ACI provides container orchestration without managing VMs, and we need the public IP to configure the frontend.

**Technical Details:**
- Deploy `rust-simple-rest-api` as an Azure Container Instance
- Expose port 3000 publicly
- Set `GEMINI_API_KEY` as environment variable for API access
- Retrieve the container's public FQDN (e.g., `api.region.azurecontainer.io`)
- This creates the backend service that the web app will communicate with

## 4. ⏳ PENDING: Update frontend config with production API URL
**Why:** The web application currently uses localhost for API calls, but in production it needs to call the deployed API container. We need to update the configuration file that the frontend loads to point to the real Azure endpoints.

**Technical Details:**
- Update `rust-static-web-server/static/config.js` to use the API's Azure FQDN instead of `localhost:3000`
- This ensures the web app can make cross-origin requests to the API
- The config.js approach allows runtime configuration without rebuilding the entire static site

## 5. ⏳ PENDING: Build and push Web image to ACR
**Why:** Containerizing the web server application so it can be deployed on Azure. Like the API, it needs to be packaged with all static files and served consistently.

**Technical Details:**
- Build Docker image for `rust-static-web-server` with updated config.js
- Include all static files (HTML, JS, CSS)
- Push to ACR for Azure deployment
- The web server serves the frontend and handles static file requests

## 6. ⏳ PENDING: Deploy Web container to ACI
**Why:** Making the web application accessible to users via the internet. This creates the user-facing component of the application.

**Technical Details:**
- Deploy `rust-static-web-server` as an Azure Container Instance
- Expose port 8080 publicly
- No environment variables needed (static server)
- Users will access this container's public FQDN to use the application

## 7. ⏳ PENDING: Verify deployment
**Why:** Ensuring everything works end-to-end in the production environment. Testing is critical to confirm interoperability between the web and API containers.

**Technical Details:**
- Access the web app URL in a browser
- Test the items management functionality (CRUD operations via API)
- Test the AI chat feature (calls to API with Gemini integration)
- Verify no CORS issues and proper cross-container communication
- Confirm the application behaves identically to local development

## Key Technical Considerations

### Networking & Security
- **CORS:** API allows all origins since both containers are in the same Azure region
- **Environment Variables:** API key passed securely to container at runtime
- **Public IPs:** Both containers get public access for inter-container communication

### Cost Optimization
- **ACI Billing:** Pay only for compute time when containers are running
- **Basic ACR SKU:** Sufficient storage and bandwidth for development/demo
- **Resource Cleanup:** All resources are in one resource group for easy deletion

### Container Architecture
- **Multi-stage Builds:** Reduce image size and attack surface
- **Static Configuration:** Config.js allows environment-specific settings without code changes
- **Independent Deployments:** API and web can be updated separately

This deployment strategy ensures scalability, security, and ease of management for the Rust full-stack application.