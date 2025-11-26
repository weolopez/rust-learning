// 'dotenv' and 'reqwest' are external crates (libraries) defined in Cargo.toml
use dotenv::dotenv;
use reqwest::Client;
// 'std' is the Rust Standard Library
use std::env;
use std::error::Error;

// Macro that initializes the Tokio async runtime (like Node.js libuv event loop)
// Transforms the async main into a synchronous one that starts the reactor
#[tokio::main]
// Returns a Result. '()' is Unit type (void/None).
// Box<dyn Error> means "Any error on the heap" (Dynamic Dispatch, like Java Interface)
async fn main() -> Result<(), Box<dyn Error>> {
    // Load .env file, ignore error if missing (.ok() converts Result to Option)
    dotenv().ok();

    // .expect() panics (crashes) if the value is missing.
    // Similar to Optional.orElseThrow() in Java.
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");

    // Creates an HTTP client. Uses Arc internally for cheap cloning.
    let client = Client::new();

    // format! macro creates a new String on the heap (like f-strings in Python)
    let url = format!("https://generativelanguage.googleapis.com/v1beta/models?key={}", api_key);
    
    // .await: Suspends function execution until future completes (Lazy execution)
    // ?: Error propagation. If Err, return immediately. If Ok, unwrap value.
    let response = client.get(&url).send().await?;
    
    println!("Status: {}", response.status());

    // Reads the full body into a String (heap allocation)
    let text = response.text().await?;
    println!("Models: {}", text);
    
    // Return Success with Unit type.
    // Memory cleanup (RAII) happens here for api_key, client, url, etc.
    Ok(())
}