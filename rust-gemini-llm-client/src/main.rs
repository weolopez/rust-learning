// ===================================================================================
// RUST LEARNING GUIDE FOR JAVA/JS/PYTHON DEVELOPERS
// ===================================================================================
// This file demonstrates a simple async HTTP client.
// Key concepts to look for:
// 1. Ownership & Borrowing: How Rust manages memory without a Garbage Collector.
// 2. Option & Result: How Rust handles nulls and errors (no NullPointerException!).
// 3. Macros: Code generation tools ending in '!' (e.g., println!, vec!).
// ===================================================================================

// IMPORTS
// Similar to `import` in Java/Python or `import/require` in JS.
// "Crates" are like npm packages or Maven dependencies.
use dotenv::dotenv;
use futures_util::StreamExt; // Extension trait for streams (like RxJS or Java Streams)
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

// STRUCTS & ATTRIBUTES
// Structs are like Classes in Java/Python but hold DATA ONLY.
// Behavior is added separately via `impl` blocks (not shown here).
//
// `#[derive(...)]` is a macro attribute. It's similar to Java Annotations or Python Decorators.
// It automatically generates code for specific "Traits" (interfaces).
// Here, we auto-generate code to Serialize this struct to JSON.
#[derive(Serialize)]
struct GenerateContentRequest {
    // `Vec<T>` is a growable array, like ArrayList<T> in Java, List in Python, or Array in JS.
    // It owns its data (heap allocated).
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    // `String` is a heap-allocated, growable string (like Java's String but mutable).
    // It is distinct from `&str` (string slice), which is just a view/pointer to string data.
    role: String,
    parts: Vec<Part>, //Part is a user-defined type (typically a struct or enum) that represents a single element stored in the Vec<Part>—the data model for one "part" in your program.
}

#[derive(Serialize)]
struct Part {
    text: String,
}

// DESERIALIZATION STRUCTS
// `#[derive(Deserialize, Debug)]`:
// - Deserialize: Allows creating this struct from JSON.
// - Debug: Allows printing the struct using `{:?}` (like toString() for debugging).
#[derive(Deserialize, Debug)]
struct GenerateContentResponse {
    // OPTION TYPE
    // Rust doesn't have `null`. Instead, it uses `Option<T>`.
    // It can be `Some(value)` or `None`.
    // This forces you to handle the "null" case explicitly, preventing NullPointerExceptions.
    // Similar to Java's `Optional<T>`, but used everywhere.
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Option<ResponseContent>,
    // Renaming fields for JSON mapping (like @JsonProperty in Jackson).
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ResponseContent {
    parts: Option<Vec<ResponsePart>>,
}

#[derive(Deserialize, Debug)]
struct ResponsePart {
    text: Option<String>,
}

// CONSTANTS
// `&str` is a "String Slice". It points to UTF-8 bytes in memory.
// Here, it points to static memory embedded in the binary.
const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:streamGenerateContent";

// MAIN FUNCTION & ASYNC
// `#[tokio::main]` sets up the async runtime (event loop).
// Rust doesn't have a built-in runtime like Node.js or the JVM; you must pull one in.
use rust_gemini_llm_client::generate_content;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let prompt = "Explain how Rust's ownership model works in 3 sentences.";
    println!("Sending prompt: {}\n", prompt);

    match generate_content(prompt, None).await {
        Ok(resp) => {
            println!("Response:\n{}\n", resp);
        }
        Err(e) => {
            eprintln!("Error calling Gemini: {}", e);
        }
    }

    Ok(())
}
