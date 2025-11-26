# CI/CD and Deployment Scripts

This document describes the CI/CD pipelines and deployment scripts for the Rust full-stack application.

## Table of Contents

- [Overview](#overview)
- [Local Scripts](#local-scripts)
- [GitHub Actions CI/CD](#github-actions-cicd)
- [Prerequisites](#prerequisites)
- [Quick Start](#quick-start)
- [Script Reference](#script-reference)

## Overview

The deployment system supports both **local manual deployment** via shell scripts and **automated CI/CD** via GitHub Actions.

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    GitHub Repository                         │
├─────────────────────────────────────────────────────────────┤
│  Push to main ──► GitHub Actions ──► Build & Deploy         │
└──────────────────────────┬──────────────────────────────────┘
                           │
                           ▼
┌─────────────────────────────────────────────────────────────┐
│                    Azure Cloud                               │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐      │
│  │ ACR         │◄───│ API Image   │    │ Web Image   │      │
│  │ (Registry)  │    │ amd64       │    │ amd64       │      │
│  └─────────────┘    └──────┬──────┘    └──────┬──────┘      │
│                            │                   │             │
│                            ▼                   ▼             │
│                   ┌─────────────┐      ┌─────────────┐       │
│                   │ API ACI     │      │ Web ACI     │       │
│                   │ :3000       │◄─────│ :8080       │       │
│                   └─────────────┘      └─────────────┘       │
└─────────────────────────────────────────────────────────────┘
```

## Local Scripts

All scripts are located in the `scripts/` directory.

| Script | Purpose |
|--------|---------|
| `create-azure-resources.sh` | Create Resource Group and ACR |
| `build-and-push-web.sh` | Build and push web server image |
| `deploy-api-to-aci.sh` | Deploy API container to ACI |
| `deploy-web-to-aci.sh` | Deploy web container to ACI |
| `deploy-all.sh` | Full deployment (all steps) |
| `cleanup-azure.sh` | Delete all Azure resources |

### Usage

```bash
# Make scripts executable
chmod +x scripts/*.sh

# Full deployment (recommended)
GEMINI_API_KEY="your-key" ./scripts/deploy-all.sh

# Or step-by-step:
./scripts/create-azure-resources.sh
./rust-simple-rest-api/scripts/build-and-push-api.sh
./scripts/deploy-api-to-aci.sh "your-gemini-key"
./scripts/build-and-push-web.sh
./scripts/deploy-web-to-aci.sh

# Cleanup
./scripts/cleanup-azure.sh
```

## GitHub Actions CI/CD

### Workflows

#### 1. CI Pipeline (`.github/workflows/ci.yml`)

Triggered on: Push to `main`/`develop`, Pull Requests to `main`

- **Check & Lint**: Runs `cargo fmt` and `cargo clippy`
- **Build**: Builds all projects in parallel
- **Test**: Runs tests for projects with test suites

#### 2. Deploy Pipeline (`.github/workflows/deploy-azure.yml`)

Triggered on:
- Push to `main` (when API or Web code changes)
- Manual trigger via GitHub UI

Steps:
1. Build API image for `linux/amd64`
2. Push to ACR with commit SHA tag
3. Deploy API container to ACI
4. Update `config.js` with production API URL
5. Build Web image for `linux/amd64`
6. Push to ACR
7. Deploy Web container to ACI
8. Run smoke tests

### Required Secrets

Configure these in GitHub → Settings → Secrets and variables → Actions:

| Secret | Description |
|--------|-------------|
| `AZURE_CREDENTIALS` | Azure service principal JSON (see below) |
| `GEMINI_API_KEY` | Google Gemini API key |

#### Creating Azure Credentials

```bash
# Create service principal with Contributor role
az ad sp create-for-rbac \
  --name "github-actions-rust-app" \
  --role Contributor \
  --scopes /subscriptions/<subscription-id>/resourceGroups/rg-rust-app \
  --sdk-auth

# Copy the JSON output to AZURE_CREDENTIALS secret
```

### Manual Deployment

You can trigger deployment manually from the Actions tab:

1. Go to Actions → "Deploy to Azure Container Instances"
2. Click "Run workflow"
3. Select options (deploy API, deploy Web)
4. Click "Run workflow"

## Prerequisites

### Local Development

- **Azure CLI**: `brew install azure-cli` or [install guide](https://docs.microsoft.com/cli/azure/install-azure-cli)
- **Docker**: [Docker Desktop](https://www.docker.com/products/docker-desktop) or Colima
- **Rust**: Install via [rustup](https://rustup.rs/)

### Azure Login

```bash
# Login to Azure
az login

# Verify subscription
az account show

# Login to ACR
az acr login --name acrrustapp
```

## Quick Start

### First-Time Setup

```bash
# 1. Clone repository
git clone <repo-url>
cd rust-learning-projects

# 2. Login to Azure
az login

# 3. Deploy everything
GEMINI_API_KEY="your-key" ./scripts/deploy-all.sh
```

### Redeployment

After code changes:

```bash
# Rebuild and redeploy API only
./rust-simple-rest-api/scripts/build-and-push-api.sh
./scripts/deploy-api-to-aci.sh "$GEMINI_API_KEY"

# Rebuild and redeploy Web only
./scripts/build-and-push-web.sh
./scripts/deploy-web-to-aci.sh
```

## Script Reference

### create-azure-resources.sh

Creates the Azure Resource Group and Container Registry.

```bash
./scripts/create-azure-resources.sh [options]

Options:
  --rg <name>       Resource group name (default: rg-rust-app)
  --location <loc>  Azure region (default: eastus)
  --acr <name>      ACR name (default: acrrustapp)
  --sku <sku>       ACR SKU (default: Basic)
```

### deploy-api-to-aci.sh

Deploys the API container to Azure Container Instances.

```bash
./scripts/deploy-api-to-aci.sh <gemini-api-key>

# Or use environment variable
GEMINI_API_KEY="key" ./scripts/deploy-api-to-aci.sh
```

### deploy-web-to-aci.sh

Deploys the web server container to Azure Container Instances.

```bash
./scripts/deploy-web-to-aci.sh [options]

Options:
  --rg <name>         Resource group (default: rg-rust-app)
  --container <name>  Container name (default: web-container)
  --dns <label>       DNS label (default: rust-web-demo)
```

### deploy-all.sh

Runs the complete deployment pipeline.

```bash
GEMINI_API_KEY="your-key" ./scripts/deploy-all.sh
```

### cleanup-azure.sh

Deletes all Azure resources.

```bash
./scripts/cleanup-azure.sh [options]

Options:
  --rg <name>   Resource group (default: rg-rust-app)
  --force       Skip confirmation prompt
```

## Troubleshooting

### Common Issues

1. **Image architecture mismatch**
   - Ensure images are built with `--platform linux/amd64`
   - Mac M-series builds arm64 by default

2. **Container connection refused**
   - Check server binds to `0.0.0.0`, not `127.0.0.1`
   - Verify port is exposed in container create command

3. **ACR authentication failed**
   - Run `az acr login --name acrrustapp`
   - Ensure admin access is enabled: `az acr update --name acrrustapp --admin-enabled true`

### Viewing Logs

```bash
# API logs
az container logs --resource-group rg-rust-app --name api-container

# Web logs
az container logs --resource-group rg-rust-app --name web-container

# Follow logs
az container logs --resource-group rg-rust-app --name api-container --follow
```

### Container Status

```bash
az container show --resource-group rg-rust-app --name api-container \
  --query "{Status:containers[0].instanceView.currentState.state, FQDN:ipAddress.fqdn}" -o table
```

## Current Deployment URLs

| Service | URL |
|---------|-----|
| API | http://rust-api-demo.eastus.azurecontainer.io:3000 |
| Web | http://rust-web-demo.eastus.azurecontainer.io:8080 |

## Cost Management

Azure Container Instances are billed per second of vCPU and memory usage.

**Estimated costs** (Basic SKU, 1 vCPU, 1GB RAM each):
- Per container: ~$1.50/day running continuously
- ACR Basic: ~$5/month

**To minimize costs:**
```bash
# Stop containers when not needed
az container stop --resource-group rg-rust-app --name api-container
az container stop --resource-group rg-rust-app --name web-container

# Or delete entirely
./scripts/cleanup-azure.sh
```
