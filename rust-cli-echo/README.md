# Rust CLI Echo

A simple command-line interface (CLI) program written in Rust that echoes user input back to the terminal. This project demonstrates basic Rust CLI development, including argument parsing and standard output handling.

## Features

- Accepts command-line arguments
- Echoes the provided message to stdout
- Provides usage help when no arguments are given
- Lightweight and fast Rust implementation

## Prerequisites

# rust-cli-echo

A multi-purpose CLI tool that can execute Azure CLI commands or forward prompts to the `rust-gemini-llm-client` library.

## Usage

The CLI supports two modes:

### 1. Azure CLI Mode
Execute Azure CLI commands directly:

```bash
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- az <command_args...>
```

Examples:
```bash
# Create resource group
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- az group create --name rg-rust-app --location eastus

# List resource groups
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- az group list --output table

# Create container registry
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- az acr create --resource-group rg-rust-app --name acrrustapp --sku Basic
```

### 2. Gemini AI Mode
Forward prompts to Gemini AI:

```bash
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- [-k API_KEY] "Your prompt here"
```

Examples:
```bash
# With environment variable
export GEMINI_API_KEY="sk_..."
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- "Explain Rust ownership in 2 sentences"

# With inline key
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- -k sk_... "Explain Rust ownership in 2 sentences"
```

If you run without a key, the program will try to load `GEMINI_API_KEY` from the environment (and `.env` via `dotenv`). If no key is found, it prints an error and exits.

Development

Build:
```bash
cargo build --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml
```

Run (debug):
```bash
cargo run --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml -- "Your prompt here"
```

Build (release):
```bash
cargo build --manifest-path /Users/weo/Development/rust/hello/rust-cli-echo/Cargo.toml --release
```

Run the compiled binary (debug):
```bash
# binary lives at target/debug/rust-cli-echo
GEMINI_API_KEY="sk_..." ./rust-cli-echo/target/debug/rust-cli-echo "Tell me a joke"
```

Run the compiled binary (release):
```bash
# binary lives at target/release/rust-cli-echo
GEMINI_API_KEY="sk_..." ./rust-cli-echo/target/release/rust-cli-echo "Tell me a joke"
```

Project layout

```
rust-cli-echo/
├─ Cargo.toml        # depends on ../rust-gemini-llm-client
└─ src/
   └─ main.rs        # simple CLI that calls generate_content(prompt, api_key_opt)
```

Security note

- Do not commit API keys. Add `.env` to `.gitignore` and use environment variables or secret managers for CI.
