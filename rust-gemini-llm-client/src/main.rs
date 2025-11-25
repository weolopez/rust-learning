use dotenv::dotenv;
use futures_util::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;

#[derive(Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize, Debug)]
struct GenerateContentResponse {
    candidates: Option<Vec<Candidate>>,
}

#[derive(Deserialize, Debug)]
struct Candidate {
    content: Option<ResponseContent>,
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

const GEMINI_API_URL: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:streamGenerateContent";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    println!("DEBUG: Loaded environment variables");

    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY must be set");
    println!("DEBUG: API key loaded (length: {})", api_key.len());
    let client = Client::new();
    println!("DEBUG: HTTP client created");

    let prompt = "Explain how Rust's ownership model works in 3 sentences.";
    println!("Sending prompt: {}\n", prompt);

    let request_body = GenerateContentRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: prompt.to_string(),
            }],
        }],
    };

    let url = format!("{}?key={}", GEMINI_API_URL, api_key);
    println!("DEBUG: Request URL: {}", url);

    println!("DEBUG: Sending request...");
    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await?;

    println!("DEBUG: Response Status: {}", response.status());

    if !response.status().is_success() {
        println!("DEBUG: Request failed. Response body:");
        let text = response.text().await?;
        println!("{}", text);
        return Ok(());
    }

    let mut stream = response.bytes_stream();
    println!("DEBUG: Stream obtained, processing chunks...");

    println!("Response:");
    while let Some(item) = stream.next().await {
        let chunk = item?;
        // The API returns a stream of JSON objects, but they might be chunked arbitrarily.
        // For simplicity in this learning project, we'll try to parse complete chunks.
        // In a production app, you'd want a more robust JSON stream parser.
        
        // Note: The actual Gemini stream format is an array of JSON objects like [{}, {}, ...]
        // Parsing the raw stream correctly requires handling the array structure.
        // However, for this simple example, we will print the raw text chunks to demonstrate streaming.
        
        if let Ok(text) = String::from_utf8(chunk.to_vec()) {
             // Clean up the response a bit for display (removing array brackets/commas if present)
            let clean_text = text.trim_start_matches('[').trim_end_matches(']').trim_end_matches(',');
            
            if let Ok(response) = serde_json::from_str::<GenerateContentResponse>(clean_text) {
                if let Some(candidates) = response.candidates {
                    for candidate in candidates {
                        if let Some(content) = candidate.content {
                            if let Some(parts) = content.parts {
                                for part in parts {
                                    if let Some(text) = part.text {
                                        print!("{}", text);
                                    }
                                }
                            }
                        }
                        if let Some(reason) = candidate.finish_reason {
                            if reason != "STOP" {
                                println!("\nFinish reason: {}", reason);
                            }
                        }
                    }
                }
            } else {
                println!("DEBUG: Failed to parse chunk as GenerateContentResponse: {}", clean_text);
            }
        } else {
            println!("DEBUG: Failed to decode chunk as UTF-8");
        }
    }
    println!("\n\nDone!");

    Ok(())
}
