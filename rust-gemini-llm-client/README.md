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
   # rust-gemini-llm-client

   A small Rust library and example CLI that calls Google Gemini Generative Language APIs (`streamGenerateContent`).

   Features
   - Async library function `generate_content(prompt: &str, api_key_opt: Option<String>) -> Result<String, Box<dyn std::error::Error + Send + Sync>>`
   - Example binary in this crate demonstrating usage
   - `rust-cli-echo` example app in the workspace that depends on this crate

   Quick start

   1) Provide your API key (recommended: environment variable)

   ```bash
   # macOS / Linux
   export GEMINI_API_KEY="sk_..."
   ```

   2) Use the library from another Rust crate (example)

   ```rust
   // Cargo.toml: add path dependency to this crate
   // rust-gemini-llm-client = { path = "../rust-gemini-llm-client" }

   use rust_gemini_llm_client::generate_content;

   #[tokio::main]
   async fn main() {
      let prompt = "Explain Rust ownership in 2 sentences.";
      match generate_content(prompt, None).await {
         Ok(resp) => println!("Response:\n{}", resp),
         Err(e) => eprintln!("Error: {}", e),
      }
   }
   ```

   3) Run the example CLI (`rust-cli-echo`) in this workspace

   ```bash
   # with env var
   GEMINI_API_KEY="sk_..." cargo run -p rust-cli-echo -- "Hello from CLI"

   # or pass key inline
   cargo run --manifest-path rust-cli-echo/Cargo.toml -- -k sk_... "Hello from CLI"
   ```

   Notes
   - The library currently concatenates the text parts received from the streaming endpoint and returns a single `String`.
   - If you need real-time token/chunk processing, I can add a streaming callback API.
   - Uses `reqwest` + `tokio`; pass an API key per-call or rely on `GEMINI_API_KEY`.

   License
   - Copy or adapt as needed for your project.
