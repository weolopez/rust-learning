// Top-level imports. Each import brings types/functions into scope similar to `import` in Java
// or `require`/`import` in JS/Python, but Rust's `use` is resolved at compile time.
use axum::{
    // extract::Path and extract::State are extractors used by axum handlers to pull values
    // from the request or the application state. Think of them as annotations that make handler
    // parameters populate automatically (similar to frameworks like Spring or Express middleware).
    extract::{Path, State},
    // HTTP method and status code types
    http::{Method, StatusCode},
    // Trait to convert types into axum responses
    response::IntoResponse,
    // Routing helpers (get, post, put, delete)
    routing::get,
    // JSON body extractor and Router type
    Json,
    Router,
};
use rust_gemini_llm_client::generate_content;
use serde::{Deserialize, Serialize};
use std::{
    // HashMap to store items in-memory
    collections::HashMap,
    // Arc (atomic reference counted pointer) and RwLock (read-write lock) for shared mutable state
    sync::{Arc, RwLock},
};
use tower_http::cors::{Any, CorsLayer};

// Data model: a simple Item struct. `derive` automatically implements common traits.
// - Debug: allows printing with `{:?}` for debugging
// - Serialize/Deserialize: provided by serde to convert to/from JSON (like Jackson in Java)
// - Clone: allow cheap-ish duplication of the value when needed
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Item {
    id: u64,
    name: String,
    completed: bool,
}

// DTO for creating an item: incoming JSON will be deserialized into this struct
#[derive(Debug, Deserialize)]
struct CreateItem {
    name: String,
}

// DTO for updating an item. Options are `Option<T>` so they can be omitted in JSON.
#[derive(Debug, Deserialize)]
struct UpdateItem {
    name: Option<String>,
    completed: Option<bool>,
}

// App state type alias. This is an Arc (thread-safe ref-counted pointer) around
// an RwLock protecting a HashMap of items. Why this pattern?
// - Arc<T>: like Java's shared object references, but explicitly reference-counted and
//   thread-safe. Cloning an Arc increases the ref count; dropping an Arc decreases it.
// - RwLock<T>: allows multiple concurrent readers or one writer at a time. This
//   pattern avoids a global mutex if readers dominate.
// In Java you might use `ConcurrentHashMap` or synchronize access; here we compose
// Arc + RwLock for shared mutable access across async tasks.
type Db = Arc<RwLock<HashMap<u64, Item>>>;

// The tokio runtime entry point. `#[tokio::main]` sets up an async runtime so we can use async/await.
// In Java you'd have an ExecutorService; in JS/Python async is single-threaded event loop. Tokio
// is a multi-threaded async runtime (configurable) that schedules lightweight tasks.
#[tokio::main]
async fn main() {
    // Initialize tracing subscriber (logging). This is optional but useful for diagnostics.
    tracing_subscriber::fmt::init();

    // Initialize state. Db::default() constructs an Arc containing an RwLock with an empty HashMap.
    // Note: this performs heap allocations. The Arc is cheap to clone when we attach it to routes.
    let db = Db::default();

    // Configure CORS. Tower-http provides middleware; we allow common HTTP methods and any origin.
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(Any)
        .allow_headers(Any);

    // Build our application router and attach handlers. `.route` maps paths to handler functions.
    // `with_state(db)` clones the Arc and makes it available to handlers via the State extractor.
    let app = Router::new()
        .route("/prompt", axum::routing::post(handle_prompt))
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/:id",
            get(get_item).put(update_item).delete(delete_item),
        )
        .layer(cors)
        .with_state(db);

    // Bind a TCP listener. `.await` is used because bind is async. unwrap() here will panic
    // if binding fails (e.g., port in use). Prefer handling errors explicitly in production.
    // Use 0.0.0.0 to listen on all interfaces (required for Docker/container deployments)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    // Serve the application. This call is async and will run until the process exits.
    axum::serve(listener, app).await.unwrap();

    #[derive(Deserialize)]
    struct PromptRequest {
        prompt: String,
        // optional per-call API key
        api_key: Option<String>,
    }

    async fn handle_prompt(
        Json(body): Json<PromptRequest>,
    ) -> (StatusCode, Json<serde_json::Value>) {
        match generate_content(&body.prompt, body.api_key).await {
            Ok(result) => (
                StatusCode::OK,
                Json(serde_json::json!({ "result": result })),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({ "error": format!("{}", e) })),
            ),
        }
    }
}

// Handlers: each is an async function. Axum uses function signatures to determine how to
// extract parameters from requests (Path, State, Json). Returning `impl IntoResponse` lets
// us return types that axum converts into HTTP responses.

// List items: read-lock the DB, collect values, return JSON vector. Note the `.read().unwrap()`:
// - `.read()` acquires a read guard; it returns a Result because poisoning can occur if a writer panicked.
// - `unwrap()` will panic on error; in production you might handle the poisoning explicitly.
async fn list_items(State(db): State<Db>) -> Json<Vec<Item>> {
    // Acquire read lock. This blocks the current async task until the lock is available.
    // Because RwLock is from std (blocking), in real async servers you might prefer tokio::sync::RwLock
    // to avoid blocking the thread. This example keeps std::sync::RwLock for simplicity, but be aware.
    let items = db.read().unwrap();
    // Clone the items because we are returning owned data. `.cloned()` uses the Clone trait on Item.
    Json(items.values().cloned().collect())
}

// Create item: extract JSON body and state, obtain write lock, insert new item, return 201 Created
async fn create_item(State(db): State<Db>, Json(payload): Json<CreateItem>) -> impl IntoResponse {
    // Acquire write lock to mutate the HashMap
    let mut items = db.write().unwrap();
    // Compute a new ID: find max key and add 1. `unwrap_or(&0)` handles empty map.
    let id = items.keys().max().unwrap_or(&0) + 1;
    let item = Item {
        id,
        name: payload.name,
        completed: false,
    };
    // Insert and return a clone to the caller
    items.insert(id, item.clone());
    (StatusCode::CREATED, Json(item))
}

// Get item by ID. Path extractor converts the `:id` segment into a u64.
async fn get_item(Path(id): Path<u64>, State(db): State<Db>) -> impl IntoResponse {
    let items = db.read().unwrap();
    // Use `if let Some(...)` to handle the Option returned by HashMap::get.
    if let Some(item) = items.get(&id) {
        // Return Ok with JSON body (axum will convert this to a 200 response)
        Ok(Json(item.clone()))
    } else {
        // Return Err with a status code; axum will convert this into an HTTP response with that status
        Err(StatusCode::NOT_FOUND)
    }
}

// Update item partially. We get a write lock, mutate in-place, and return the updated item.
async fn update_item(
    Path(id): Path<u64>,
    State(db): State<Db>,
    Json(payload): Json<UpdateItem>,
) -> impl IntoResponse {
    let mut items = db.write().unwrap();
    if let Some(item) = items.get_mut(&id) {
        // Optional fields: only update when provided
        if let Some(name) = payload.name {
            item.name = name;
        }
        if let Some(completed) = payload.completed {
            item.completed = completed;
        }
        Ok(Json(item.clone()))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// Delete item. Return 204 No Content on success.
async fn delete_item(Path(id): Path<u64>, State(db): State<Db>) -> impl IntoResponse {
    let mut items = db.write().unwrap();
    if items.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

// Additional notes for Java/JS/Python developers learning Rust:
// - Memory management: Rust uses ownership and borrowing instead of a garbage collector.
//   Values have a single owner at a time; when the owner goes out of scope the value is dropped.
//   References (`&T`, `&mut T`) allow borrowing without transferring ownership. The compiler
//   enforces lifetimes so references cannot outlive the data they point to.
// - Concurrency: `Arc<T>` is an atomically reference-counted pointer to allow shared ownership
//   across threads (similar to `shared_ptr` with atomic ops). `RwLock<T>` serializes access
//   allowing multiple readers or one writer. Combining Arc + RwLock is a common pattern for
//   shared mutable state in Rust async servers. In Java you might use synchronized collections
//   or ConcurrentHashMap; in JS you rarely share memory across threads because Node is single-threaded.
// - Blocking vs async: `std::sync::RwLock` blocks the current thread when acquiring a lock. In an
//   async runtime like tokio, blocking the thread can starve other tasks. For production async
//   servers prefer `tokio::sync::RwLock` or other async-aware primitives.
// - Error handling: Rust uses Result/Option for recoverable cases. `unwrap()` panics on Err which
//   is like throwing an unchecked exception; prefer graceful handling or propagating errors with `?`.
// - Pattern matching: `match`, `if let`, and `while let` are powerful tools to destructure enums
//   like Result and Option; they replace many ad-hoc null checks or try/catch logic in other languages.
// - Traits: Rust uses trait-based polymorphism (think Java interfaces). `IntoResponse` is a trait that
//   allows many types to be converted into HTTP responses.
// - Clone: cloning in Rust is explicit. `Clone` may be cheap (copying a pointer) or expensive (deep clone),
//   depending on the type. This explicitness helps you reason about allocations.
// - Panics: panics unwind the stack by default (or abort if configured). Avoid panics for recoverable errors
//   in server code; return appropriate HTTP errors instead.
// - Logging: consider using the tracing crate for structured logging in async applications. It
//   integrates well with tokio and provides more context than println!.
// - Configuration: use environment variables or config files (with the config crate) for
//   configurable settings like bind address/port, database URLs, etc.
// - Testing: Rust has built-in support for unit and integration tests. Use `#[cfg(test)]`
//   to conditionally compile test code, and `cargo test` to run tests.
// - Documentation: Rust's `cargo doc` generates HTML documentation from comments. Use
//   `///` for doc comments, and markdown is supported in comments.
// - Community: Rust has a friendly and welcoming community. Don't hesitate to ask questions
//   on forums, Discord, or the Rust user subreddit. The community is generally very helpful.
//   Remember to follow the Rust Code of Conduct.

// If you want, I can follow up with suggested edits to use tokio::sync::RwLock to avoid blocking the
// async runtime, or show how to replace unwrap() with proper error handling and logging.
