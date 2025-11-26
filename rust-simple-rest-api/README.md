# Rust Simple REST API

A simple REST API server implemented in Rust using the `axum` framework. This project demonstrates how to build a basic CRUD (Create, Read, Update, Delete) API with in-memory storage.

## Features

- **RESTful Endpoints**: Standard HTTP methods for resource management.
- **JSON Serialization**: Automatic parsing and serialization of JSON data.
- **In-Memory Storage**: Uses a thread-safe `HashMap` to store data (reset on restart).
- **Error Handling**: Proper HTTP status codes for different scenarios.

## Prerequisites

- Rust 1.70 or higher
- Cargo (included with Rust)

## Installation

1. Navigate to the project directory:
   ```bash
   cd rust-simple-rest-api
   ```

2. Build the project:
   ```bash
   cargo build
   ```

## Usage

1. Run the server:
   ```bash
   cargo run
   ```
   The server will start listening on `127.0.0.1:3000`.

2. Test the endpoints using `curl` or any API client (like Postman).

### API Endpoints

| Method | Endpoint | Description | Request Body |
|--------|----------|-------------|--------------|
| GET | `/items` | List all items | None |
| POST | `/items` | Create a new item | `{"name": "Item Name"}` |
| GET | `/items/:id` | Get a specific item | None |
| PUT | `/items/:id` | Update an item | `{"name": "New Name", "completed": true}` |
| DELETE | `/items/:id` | Delete an item | None |
| POST | `/prompt` | Generate a response based on the input prompt | `{"prompt": "Your prompt here"}` |

### Examples

**Create an item:**
```bash
curl -X POST http://localhost:3000/items \
  -H "Content-Type: application/json" \
  -d '{"name": "Buy milk"}'
```

**List items:**
```bash
curl http://localhost:3000/items
```

**Update an item:**
```bash
curl -X PUT http://localhost:3000/items/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'
```

**Delete an item:**
```bash
curl -X DELETE http://localhost:3000/items/1
```
**Generate a response:**
```bash
curl -X POST http://localhost:3000/prompt \
   -H "Content-Type: application/json" \
   -d '{"prompt": "What is the weather today?"}'
```

### Environment (.env)

You can provide the Gemini API key via environment variable `GEMINI_API_KEY`. The project uses the `dotenv` crate to load a local `.env` file automatically when running with Cargo (convenience only).

Create a `.env` file in the project root (do not commit it):
```
GEMINI_API_KEY=sk_...
```

Or create a `.env.example` with the placeholder to share with teammates:
```
GEMINI_API_KEY=YOUR_API_KEY_HERE
```

## Project Structure

- `src/main.rs`: Contains the API server implementation, data models, and route handlers.
- `Cargo.toml`: Project configuration and dependencies.

## Dependencies

- `axum`: Web framework
- `tokio`: Async runtime
- `serde`: Serialization framework
- `serde_json`: JSON support
- `tracing`: Logging instrumentation