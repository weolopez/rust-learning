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
- **`rust-docker-server`**: The simple REST API server packaged as a deployable Docker image.
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

### rust-docker-server
**Purpose**: Containerize a Rust application
**Key Concepts**: Docker basics, multi-stage builds, containerization best practices
**Dependencies**: Docker, based on `rust-simple-rest-api`
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
5. **Containerize with `rust-docker-server`** - Learn deployment basics
6. **Deploy to cloud with `rust-azure-app-service`** - Experience cloud deployment

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

## Roadmap

- [ ] Add more advanced CLI projects (argument parsing libraries, interactive CLIs)
- [ ] Implement authentication and authorization in API projects
- [ ] Add database integration examples
- [ ] Create WebSocket server project
- [ ] Add AWS Lambda deployment example
- [ ] Include performance benchmarking projects
- [ ] Add async/await examples in web projects
- [ ] Create microservices architecture example

---

Happy learning Rust! 🚀