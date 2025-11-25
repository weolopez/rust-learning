# Rust Static Web Server

A basic static web server implemented in Rust using the `tiny_http` crate. This project demonstrates how to handle HTTP requests, serve static files, and manage basic routing in Rust.

## Features

- Serves static files from the `static/` directory
- Handles 404 Not Found errors
- Basic security against directory traversal
- Runs on port 8080

## Prerequisites

- Rust and Cargo (installed via [rustup](https://rustup.rs/))

## Getting Started

1. Navigate to the project directory:
   ```bash
   cd rust-static-web-server
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the server:
   ```bash
   cargo run
   ```

4. Open your browser and visit:
   [http://localhost:8080](http://localhost:8080)

## Project Structure

- `src/main.rs`: Contains the server implementation logic.
- `static/`: Directory containing static files (HTML, CSS, JS, images) to be served.
- `Cargo.toml`: Project configuration and dependencies.

## Dependencies

- [tiny_http](https://crates.io/crates/tiny_http): A low-level HTTP server library for Rust.