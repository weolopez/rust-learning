# Task 4: Update frontend config.js with production API URL

Date: 2025-11-26
Status: READY (awaiting API FQDN from Task 3)

Purpose
Update the web frontend's API configuration to call the deployed API container instead of localhost during development.

Current config (development)
File: `rust-static-web-server/static/config.js`
- Likely has: `API_URL = "http://localhost:3000"`

What needs to change
Replace localhost with the production API FQDN from Task 3 (e.g., `api-container.eastus.azurecontainer.io`)

Steps

1) Wait for Task 3: API container FQDN (e.g., `api-container.eastus.azurecontainer.io`)

2) Read current config.js to see its format
```bash
cat rust-static-web-server/static/config.js
```

3) Update config.js with production URL
```bash
# Replace localhost:3000 with production FQDN:3000
sed -i '' "s|localhost:3000|api-container.eastus.azurecontainer.io:3000|g" rust-static-web-server/static/config.js
```

Or manually edit the file to use the FQDN

4) Verify the change
```bash
cat rust-static-web-server/static/config.js
```

Expected output
- Should show: `http://api-container.eastus.azurecontainer.io:3000`
- Or similar production URL format

Next step
- Task 5: Build and push web server Docker image (which will include the updated config.js)

Note
Config.js is served as a static file, so the frontend will load it at runtime and use the production API URL without requiring a rebuild of the web server binary.
