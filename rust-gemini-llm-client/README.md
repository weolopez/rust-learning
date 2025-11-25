# Rust Gemini LLM Client

A Rust client for interacting with the Google Gemini LLM API. This project demonstrates how to perform API authentication, send requests, handle streaming responses, and parse JSON data in Rust.

## Features

- Authenticates with the Gemini API using an API key
- Sends a prompt to the `gemini-pro` model
- Handles streaming responses for real-time output
- Parses JSON responses using `serde` and `serde_json`

## Prerequisites

- Rust and Cargo (installed via [rustup](https://rustup.rs/))
- A Google Cloud Project with the Gemini API enabled
- An API key for the Gemini API

## Getting Started

1. Navigate to the project directory:
   ```bash
   cd rust-gemini-llm-client
   ```

2. Create a `.env` file from the example:
   ```bash
   cp .env.example .env
   ```

3. Open `.env` and replace `your_api_key_here` with your actual Gemini API key.

4. Build the project:
   ```bash
   cargo build
   ```

5. Run the client:
   ```bash
   cargo run
   ```

## Project Structure

- `src/main.rs`: Contains the client implementation logic.
- `Cargo.toml`: Project configuration and dependencies.
- `.env`: Stores the API key (not committed to version control).

## Dependencies

- [reqwest](https://crates.io/crates/reqwest): An HTTP client for Rust.
- [tokio](https://crates.io/crates/tokio): An asynchronous runtime for Rust.
- [serde](https://crates.io/crates/serde): A framework for serializing and deserializing Rust data structures.
- [serde_json](https://crates.io/crates/serde_json): A JSON serialization/deserialization library for Rust.
- [dotenv](https://crates.io/crates/dotenv): A library for loading environment variables from a `.env` file.