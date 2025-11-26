use dotenv::dotenv;
// 'futures_util' provides utilities for async streams (like RxJS or Python Async Generators)
use futures_util::StreamExt;
use reqwest::Client;
// 'serde' is the standard serialization framework (like Jackson in Java or json in Python)
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

// #[derive(...)]: Automatically implements traits (interfaces) for the struct.
// Serialize: Allows this struct to be converted to JSON.
#[derive(Serialize)]
struct GenerateContentRequest {
    // Vec<T>: A growable array (like ArrayList in Java or List in Python).
    // Allocated on the heap.
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    // String: A heap-allocated, growable UTF-8 string.
    // Not to be confused with &str (string slice), which is a view into a string.
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

// Deserialize: Allows creating this struct from JSON.
// Debug: Allows printing the struct with {:?} for debugging.
#[derive(Deserialize, Debug)]
struct GenerateContentResponse {
    // Option<T>: Represents a value that might be missing (null safety).
    // Rust has no 'null'. You must handle the None case explicitly.
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Option<ResponseContent>,
    // Renames the JSON field "finishReason" to the Rust field "finish_reason"
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

// 'const': Compile-time constant. Inlined wherever used.
const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-flash-latest:streamGenerateContent";

/// Generate content from Gemini. If `api_key_opt` is None the function will
/// attempt to read `GEMINI_API_KEY` from environment (.env is loaded automatically).
/// Returns the concatenated text response.
// 'pub': Makes this function accessible from other modules/crates.
// 'Send + Sync': Thread-safety markers.
//   - Send: Can be moved to another thread.
//   - Sync: Can be shared between threads.
//   Required here because async runtimes (Tokio) are multi-threaded.
pub async fn generate_content(
    prompt: &str, // Borrowed string slice (efficient, no copy)
    api_key_opt: Option<String>, // Takes ownership of an optional String
) -> Result<String, Box<dyn Error + Send + Sync>> {
    dotenv().ok();

    // 'match': Pattern matching (like switch on steroids).
    // Forces you to handle all cases (Some and None).
    let api_key = match api_key_opt {
        Some(k) => k, // If provided, use it
        None => env::var("GEMINI_API_KEY")?, // If None, look up env var. ? propagates error.
    };

    let client = Client::new();

    // Struct initialization syntax.
    // 'vec![]' macro creates a Vec on the heap.
    let request_body = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(), // .to_string() allocates memory on heap
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let url = format!("{}?key={}", GEMINI_API_URL, api_key);

    // .json(): Serializes the struct to JSON automatically using Serde.
    let response = client.post(&url).json(&request_body).send().await?;

    if !response.status().is_success() {
        let status = response.status();
        let text = response.text().await.unwrap_or_default();
        // .into(): Converts the String error into Box<dyn Error> automatically
        return Err(format!("Request failed: {} - {}", status, text).into());
    }

    // Streaming response handling (Memory efficient for large responses)
    let mut stream = response.bytes_stream();
    let mut output = String::new();

    // 'while let': Loops as long as the pattern matches (stream yields Some(item))
    while let Some(item) = stream.next().await {
        let chunk = item?; // Unwrap the chunk or propagate error
        
        // Convert bytes to UTF-8 string
        if let Ok(text) = String::from_utf8(chunk.to_vec()) {
            // Manual JSON cleanup (The API returns a stream of JSON arrays, which is tricky)
            // In a production app, you'd use a proper streaming JSON parser.
            let clean_text = text
                .trim()
                .trim_start_matches('[')
                .trim_start_matches(',')
                .trim_end_matches(']')
                .trim_end_matches(',')
                .trim()
                .to_string();

            if clean_text.is_empty() {
                continue;
            }

            // Attempt to parse the cleaned chunk
            // ::<Type> syntax specifies what to parse into (Generics)
            if let Ok(parsed) = serde_json::from_str::<GenerateContentResponse>(&clean_text) {
                // Nested 'if let' to safely access deeply nested Option types
                // This avoids NullPointerExceptions by design.
                if let Some(candidates) = parsed.candidates {
                    for candidate in candidates {
                        if let Some(content) = candidate.content {
                            if let Some(parts) = content.parts {
                                for part in parts {
                                    if let Some(text) = part.text {
                                        output.push_str(&text); // Append to output buffer
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(output)
}
