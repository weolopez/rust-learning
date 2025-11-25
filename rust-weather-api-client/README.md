# Rust Weather API Client

A simple command-line weather client written in Rust. This project demonstrates how to make asynchronous HTTP requests, parse JSON responses, and manage environment variables in a Rust application.

It is designed with comments specifically tailored for Java developers to help map concepts between the two languages.

## Features

- Fetches current weather data for a hardcoded city (London) using the OpenWeatherMap API.
- Deserializes JSON responses into strongly-typed Rust structs.
- Handles errors gracefully.
- Uses environment variables for API key management.

## Prerequisites

- **Rust**: Ensure you have Rust installed (version 1.70+ recommended).
- **OpenWeatherMap API Key**: Sign up at [openweathermap.org](https://openweathermap.org/api) to get a free API key.

## Setup

1. **Clone the repository** (if you haven't already).
2. **Navigate to the project directory**:
   ```bash
   cd rust-weather-api-client
   ```
3. **Configure Environment Variables**:
   - Copy the example environment file:
     ```bash
     cp .env.example .env
     ```
   - Open `.env` and replace `your_api_key_here` with your actual OpenWeatherMap API key.

## Running the Project

Run the application using Cargo:

```bash
cargo run
```

You should see output similar to:

```text
Fetching weather for London...
Full Weather Data: WeatherResponse { coord: Coord { lon: -0.1257, lat: 51.5085 }, ... }
---------------------------------
Weather in London: broken clouds
Temperature: 15.32°C
Humidity: 72%
Wind Speed: 4.12 m/s
```

## Key Concepts for Java Developers

- **Cargo.toml**: Similar to `pom.xml` (Maven) or `build.gradle` (Gradle). It manages dependencies and build settings.
- **`mod models`**: Declares a module. In Rust, file structure often maps to module structure, similar to packages in Java.
- **Structs**: The primary way to define data structures, analogous to classes with public fields.
- **`#[derive(Deserialize)]`**: An attribute macro that automatically generates code for JSON deserialization, similar to using Jackson annotations like `@JsonDeserialize`.
- **`Result<T, E>`**: Rust's way of handling errors without exceptions. It's an enum that can be either `Ok(value)` or `Err(error)`.
- **`async/await`**: Rust's asynchronous programming model. `tokio` is the runtime that executes these async tasks, similar to how a framework like Spring WebFlux or a library like Netty handles non-blocking I/O.
- **`?` Operator**: A syntactic sugar for error propagation. It unwraps `Ok` values or returns `Err` values early, reducing the boilerplate of `try-catch` blocks.

## Dependencies

- **[reqwest](https://crates.io/crates/reqwest)**: An ergonomic, async HTTP client.
- **[tokio](https://crates.io/crates/tokio)**: An asynchronous runtime for Rust.
- **[serde](https://crates.io/crates/serde)**: A framework for serializing and deserializing Rust data structures.
- **[dotenv](https://crates.io/crates/dotenv)**: Loads environment variables from a `.env` file.