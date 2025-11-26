// API Configuration
// This file is loaded by index.html and configures the API endpoint for the frontend

// Production API endpoint (set by deployment)
const API_BASE_URL = "http://rust-api-demo.eastus.azurecontainer.io:3000";

// Global config object used by web components
window.APP_CONFIG = {
    apiUrl: `${API_BASE_URL}/items`,
    promptUrl: `${API_BASE_URL}/prompt`,
    baseUrl: API_BASE_URL
};

// Also expose API_URL for backward compatibility
const API_URL = API_BASE_URL;

console.log("API endpoint configured as:", window.APP_CONFIG);
