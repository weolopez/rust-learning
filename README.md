# Rust Learning Projects

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A collection of standalone Rust projects designed for learning different aspects of Rust development. Each project focuses on a specific use case and demonstrates beginner-level concepts in Rust programming.

## Table of Contents

- [Rust Learning Projects](#rust-learning-projects)
  - [Table of Contents](#table-of-contents)
  - [Project Structure](#project-structure)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
  - [Project Details](#project-details)
    - [rust-cli-echo](#rust-cli-echo)
    - [rust-static-web-server](#rust-static-web-server)
    - [rust-simple-rest-api](#rust-simple-rest-api)
    - [rust-weather-api-client](#rust-weather-api-client)
    - [rust-gemini-llm-client](#rust-gemini-llm-client)
    - [rust-docker-server](#rust-docker-server)
    - [rust-azure-app-service](#rust-azure-app-service)
  - [Learning Path](#learning-path)
  - [Getting Started](#getting-started)
  - [Contributing](#contributing)
    - [Code Style](#code-style)
    - [Testing](#testing)
  - [License](#license)
  - [Acknowledgments](#acknowledgments)
  - [Contact](#contact)
  - [Roadmap](#roadmap)

## Project Structure

- **`rust-cli-echo`**: A simple command-line interface (CLI) program that echoes user input.
- **`rust-static-web-server`**: A basic static web server that serves HTML files.
- **`rust-simple-rest-api`**: A simple REST API server with endpoints for basic CRUD operations on a data model (e.g., a list of items).
- **`rust-weather-api-client`**: An API integrator that fetches weather data from a public API (e.g., OpenWeatherMap).
- **`rust-gemini-llm-client`**: A Rust client for interacting with the Gemini LLM API (auth, request/response, streaming, and JSON parsing).
- **`rust-docker-server`**: The simple REST API server packaged as a deployable Docker image.
- **`rust-gpui-app`**: A GUI application using the GPUI framework for learning desktop UI development.
- **`rust-azure-app-service`**: The simple REST API server configured for deployment to Azure App Service.

Each project is standalone and includes:
- Complete source code
- `Cargo.toml` for dependencies and build configuration
- `README.md` with build, run instructions, and documentation
- Examples and usage guides

## Prerequisites

- **Rust**: Version 1.70 or higher. Install via [rustup](https://rustup.rs/)
- **Cargo**: Included with Rust installation for dependency management and building
- **For deployment projects**:
  - Docker (for `rust-docker-server`)
  - Azure CLI (for `rust-azure-app-service`)
- **For GUI projects**:
  - Xcode with command line tools (for `rust-gpui-app` on macOS)

All projects are designed to work on macOS, Linux, and Windows with minimal system dependencies.

## Installation

1. Clone this repository:
   ```bash
   git clone <repository-url>
   cd rust-learning-projects
   ```

2. Install Rust if not already installed:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

3. Verify installation:
   ```bash
   rustc --version
   cargo --version
   ```

Each project can be built and run independently from its subdirectory.

## Project Details

### rust-cli-echo
**Purpose**: Learn basic CLI development in Rust
**Key Concepts**: Argument parsing, standard output, error handling
**Dependencies**: None (uses only std library)
**Complexity**: Beginner

### rust-static-web-server
**Purpose**: Understand basic web server implementation
**Key Concepts**: HTTP handling, file serving, TCP connections
**Dependencies**: None (basic std library usage)
**Complexity**: Beginner

### rust-simple-rest-api
**Purpose**: Build a RESTful API server
**Key Concepts**: HTTP methods, JSON serialization, routing, CRUD operations
**Dependencies**: A web framework like `axum` or `actix-web`
**Complexity**: Beginner to Intermediate

### rust-weather-api-client
**Purpose**: Integrate with external APIs
**Key Concepts**: HTTP client requests, API key management, error handling, JSON parsing
**Dependencies**: HTTP client library like `reqwest`
**Complexity**: Beginner to Intermediate

### rust-gemini-llm-client
**Purpose**: Interact with Large Language Models (LLMs)
**Key Concepts**: API authentication, streaming responses, JSON handling, async/await
**Dependencies**: `reqwest`, `tokio`, `serde`, `dotenv`
**Complexity**: Intermediate

### rust-docker-server
**Purpose**: Containerize a Rust application
**Key Concepts**: Docker basics, multi-stage builds, containerization best practices
**Dependencies**: Docker, based on `rust-simple-rest-api`
**Complexity**: Intermediate

### rust-gpui-app
**Purpose**: Learn desktop GUI development with Rust
**Key Concepts**: GPUI framework, UI components, rendering, event handling
**Dependencies**: GPUI crate, requires Xcode and Metal on macOS
**Complexity**: Intermediate

### rust-azure-app-service
**Purpose**: Deploy Rust applications to cloud platforms
**Key Concepts**: Azure App Service, deployment configuration, environment variables
**Dependencies**: Azure CLI, based on `rust-simple-rest-api`
**Complexity**: Intermediate

## Learning Path

Recommended order for learning:

1. **Start with `rust-cli-echo`** - Master basic Rust syntax and CLI fundamentals
2. **Then `rust-static-web-server`** - Learn networking and HTTP basics
3. **Move to `rust-simple-rest-api`** - Understand web frameworks and API design
4. **Try `rust-weather-api-client`** - Practice external API integration
5. **Build `rust-gemini-llm-client`** - Master async streams and complex API interactions
6. **Containerize with `rust-docker-server`** - Learn deployment basics
7. **Explore GUI with `rust-gpui-app`** - Learn desktop UI development
8. **Deploy to cloud with `rust-azure-app-service`** - Experience cloud deployment

Each project builds on concepts from the previous ones, creating a progressive learning curve.

## Getting Started

Navigate to any project directory and follow the instructions in its README.md:

```bash
cd rust-cli-echo
cargo build
cargo run -- hello world
```

## Contributing

We welcome contributions! Please follow these guidelines:

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/amazing-improvement`
3. **Make your changes** following Rust best practices
4. **Run tests and formatting**:
   ```bash
   cargo fmt
   cargo clippy
   cargo test
   ```
5. **Commit your changes**: `git commit -m 'Add amazing improvement'`
6. **Push to the branch**: `git push origin feature/amazing-improvement`
7. **Open a Pull Request**

### Code Style
- Follow the [Rust Style Guide](https://doc.rust-lang.org/1.0.0/style/README.html)
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings

### Testing
- Add unit tests for new functionality
- Ensure all tests pass before submitting PRs

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) - A systems programming language that runs fast, prevents segfaults, and guarantees thread safety
- Inspired by various Rust learning resources and tutorials
- Thanks to the Rust community for excellent documentation and tools

## Contact

- **Issues**: Report bugs or request features via [GitHub Issues](https://github.com/username/rust-learning-projects/issues)
- **Discussions**: Join community discussions for questions and help
- **Email**: For private inquiries or collaborations

## Azure Deployment

This project includes full CI/CD support for deploying to Azure Container Instances.

### Quick Deploy

```bash
# Login to Azure
az login

# Deploy everything
GEMINI_API_KEY="your-key" ./scripts/deploy-all.sh
```

### CI/CD Documentation

See [docs/CICD.md](docs/CICD.md) for complete documentation on:
- Local deployment scripts
- GitHub Actions workflows
- Azure infrastructure setup
- Troubleshooting guide

### Deployment Scripts

| Script | Purpose |
|--------|---------|
| `scripts/deploy-all.sh` | Full deployment pipeline |
| `scripts/create-azure-resources.sh` | Create Azure RG + ACR |
| `scripts/deploy-api-to-aci.sh` | Deploy API container |
| `scripts/deploy-web-to-aci.sh` | Deploy Web container |
| `scripts/cleanup-azure.sh` | Delete all resources |

### Current Deployment

| Service | URL |
|---------|-----|
| API | http://rust-api-demo.eastus.azurecontainer.io:3000 |
| Web | http://rust-web-demo.eastus.azurecontainer.io:8080 |

## Roadmap

- [ ] Add more advanced CLI projects (argument parsing libraries, interactive CLIs)
- [ ] Implement authentication and authorization in API projects
- [ ] Add database integration examples
- [ ] Create WebSocket server project
- [ ] Add AWS Lambda deployment example
- [ ] Include performance benchmarking projects
- [ ] Add async/await examples in web projects
- [ ] Create microservices architecture example
- [x] Azure Container Instances deployment
- [x] CI/CD with GitHub Actions

---

Happy learning Rust! 🚀