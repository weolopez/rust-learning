//! This module demonstrates a simple HTTP proxy server using Rust's Actix Web framework.
//!
//! As a Java developer learning Rust, note key differences:
//! - Rust uses `use` statements for importing, similar to Java's import.
//! - Structs are defined with `struct`, and traits like `Deserialize` and `Serialize` are derived using `#[derive]`.
//! - No classes, but structs can have methods implemented on them.
//! - Memory safety is enforced by the borrow checker, no garbage collection.

use actix_web::{web, App, HttpServer, HttpResponse, Error};
use reqwest::Client;
use serde::{Deserialize, Serialize};

/// Represents a request to the Gemini API.
/// In Rust, structs are like Java's classes but without methods by default.
/// The `#[derive(Deserialize, Serialize)]` attribute automatically implements JSON serialization/deserialization,
/// similar to Jackson in Java but compile-time generated.
#[derive(Deserialize, Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

/// Represents content in a Gemini request.
/// Vectors (`Vec<T>`) are growable arrays, similar to ArrayList in Java.
/// Ownership and borrowing replace Java's references.
#[derive(Deserialize, Serialize)]
struct Content {
    parts: Vec<Part>,
}

/// Represents a part of content, containing text.
/// Strings in Rust are UTF-8 encoded, owned types (String) vs borrowed (&str).
/// Similar to String in Java but with explicit ownership.
#[derive(Deserialize, Serialize)]
struct Part {
    text: String,
}
/// Asynchronous function that proxies requests to the Gemini API.
/// In Rust, async functions return futures, similar to Java's CompletableFuture.
/// The `?` operator propagates errors, like checked exceptions but with Result types.
/// No nulls: uses Option and Result for safety.
///
/// Parameters:
/// - `req_body`: Extracted JSON from request, automatically deserialized.
/// - `client`: Shared HTTP client, wrapped in web::Data for thread safety.
/// - `req`: The HTTP request for headers.
///
/// Returns: Result with HttpResponse on success, Error on failure.
async fn proxy_gemini(
    req_body: web::Json<GeminiRequest>,
    client: web::Data<Client>, // Keep client for making HTTP requests
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    // Get API key from header, using Option combinators.
    // `and_then` chains operations, `map` transforms, similar to Optional in Java.
    let api_key = req.headers().get("X-Gemini-API-Key").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
    if api_key.is_none() {
        // Early return with error response. `json!` is a macro for creating JSON.
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({"error": "Missing API key"})));
    }

    // Build URL with API key. `format!` macro interpolates strings, like String.format in Java.
    let url = format!("{}?key={}", "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent", api_key.unwrap());
    // Send POST request, await the future. `.map_err` converts errors.
    let response = client.post(&url).json(&req_body.into_inner()).send().await.map_err(|_| actix_web::error::ErrorInternalServerError("Request failed"))?;

    if response.status().is_success() {
        // Deserialize JSON response. `await` suspends the function.
        let body: serde_json::Value = response.json().await.map_err(|_| actix_web::error::ErrorInternalServerError("Parse failed"))?;
        Ok(HttpResponse::Ok().json(body))
    } else {
        // Return error response with status and body.
        Ok(HttpResponse::build(response.status()).body(response.text().await.unwrap_or_default()))
    }
}

/// Creates an Actix Web application with the proxy route.
/// Functions can return `impl Trait` for opaque types, hiding complex generics.
/// Similar to Java's generics but with impl for existential types.
/// `web::Data` shares state across requests, like Spring's beans but compile-time safe.
///
/// Parameters:
/// - `client`: HTTP client to share.
///
/// Returns: Configured Actix App.
fn create_app(client: Client) -> App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, Error = actix_web::Error, InitError = ()>> {
    // Build app with shared data and route. Closures are used for handlers.
    App::new().app_data(web::Data::new(client)).route("/proxy", web::post().to(proxy_gemini))
}

/// The main entry point of the application.
/// `#[actix_web::main]` macro sets up the async runtime, similar to Java's main but with async support.
/// Functions return `Result` for error handling, `?` propagates errors.
/// No exceptions, explicit error handling.
///
/// Returns: std::io::Result indicating success or failure.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create server with factory closure, bind to address, run. `?` propagates bind errors.
    HttpServer::new(|| create_app(Client::new())).bind("0.0.0.0:8089")?.run().await
}