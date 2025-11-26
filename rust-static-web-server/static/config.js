// API Configuration
// This file is loaded by index.html and configures the API endpoint for the frontend

// Production API endpoint (set by deployment)
// Will be updated during deployment to point to the actual ACI FQDN
const API_URL = "http://rust-api-demo.eastus.azurecontainer.io:3000";

// Fallback for development
// const API_URL = "http://localhost:3000";

// Production example (filled in during deployment):
// const API_URL = "http://api-container.eastus.azurecontainer.io:3000";

console.log("API endpoint configured as:", API_URL);
