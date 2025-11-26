# Task 7: Verify end-to-end deployment

Date: 2025-11-26
Status: READY (after Tasks 3 & 6: both containers deployed)

Purpose
Ensure the full stack works: web app accessible, API responding, communication between web and API working, AI chat functional.

Prerequisites
- Task 3 complete: API container deployed (FQDN: `api-container.eastus.azurecontainer.io`)
- Task 6 complete: Web container deployed (FQDN: `web-container.eastus.azurecontainer.io`)

Verification steps

1) **Check both containers are running**
```bash
az container list --resource-group rg-rust-app --query "[].{Name:name, Status:containers[0].instanceView.currentState.state, FQDN:ipAddress.fqdn}" -o table
```

Expected: Both api-container and web-container showing "Running"

2) **Test API directly**
```bash
# Get items (should return empty list or existing items)
curl http://api-container.eastus.azurecontainer.io:3000/items

# Create an item
curl -X POST http://api-container.eastus.azurecontainer.io:3000/items \
  -H "Content-Type: application/json" \
  -d '{"name":"Test Item"}'

# Get items again
curl http://api-container.eastus.azurecontainer.io:3000/items
```

Expected: API responds with JSON, CRUD operations work

3) **Access web app**
```bash
# Open in browser or curl
curl http://web-container.eastus.azurecontainer.io:8080
```

Expected: HTML response (index.html served)

4) **Test web app → API communication**
- Open browser: `http://web-container.eastus.azurecontainer.io:8080`
- Use the web UI to add items
- The web app should call the API at the configured FQDN
- Items should appear in the list

5) **Test AI chat feature**
- In the web app, use the "AI Chat" feature
- Should call API endpoint `/prompt` with your message
- API should call Gemini LLM and return response
- Verify the response appears in the chat

6) **Check API logs**
```bash
az container logs --resource-group rg-rust-app --name api-container
```

Expected: Logs show requests from the web container or direct curl

7) **Check web server logs**
```bash
az container logs --resource-group rg-rust-app --name web-container
```

Expected: Logs show HTTP requests

Testing checklist
- [ ] API container running
- [ ] Web container running
- [ ] Direct API calls work (curl to API endpoint)
- [ ] Web app loads in browser
- [ ] Web UI can create items (calls API successfully)
- [ ] Web UI displays items from API
- [ ] AI chat sends request to API
- [ ] AI chat receives Gemini LLM response
- [ ] CORS working (no cross-origin errors in browser console)
- [ ] No connection timeouts between containers

Troubleshooting

**API not responding:**
```bash
az container logs --resource-group rg-rust-app --name api-container
az container show --resource-group rg-rust-app --name api-container --query containers[0].instanceView.events -o json
```

**Web app can't reach API:**
- Verify web container has correct API FQDN in config.js
- Check browser console for CORS errors
- Test API directly from your local machine (curl)
- Verify both containers are in same resource group (region = eastus)

**AI chat not working:**
- Check that GEMINI_API_KEY was set correctly in API container
- Check API logs for Gemini API errors
- Test `/prompt` endpoint directly:
  ```bash
  curl -X POST http://api-container.eastus.azurecontainer.io:3000/prompt \
    -H "Content-Type: application/json" \
    -d '{"prompt":"Hello"}'
  ```

**Performance issues:**
- Current allocation: 1 CPU, 1 GB memory per container
- If slow, consider increasing resources (costs will increase)
- Monitor container health in Azure Portal

Cleanup when done
```bash
# Delete both containers
az container delete --resource-group rg-rust-app --name api-container --yes
az container delete --resource-group rg-rust-app --name web-container --yes

# Keep RG and ACR for future deployments, or delete entirely:
az group delete --name rg-rust-app --yes
```

Success!
If all checks pass, your Rust full-stack application is successfully deployed to Azure with:
- REST API responding to requests
- Web UI accessible and functional
- Inter-container communication working
- AI chat integration operational
