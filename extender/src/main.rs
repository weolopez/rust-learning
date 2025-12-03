use actix_web::{web, App, HttpServer, HttpResponse, Error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct GeminiRequest {
    pub contents: Vec<Content>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Content {
    pub parts: Vec<Part>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub struct Part {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProxyResponse {
    pub candidates: Vec<Candidate>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Candidate {
    pub content: Content,
}

/// Configuration for the proxy service
#[derive(Clone)]
pub struct ProxyConfig {
    pub gemini_base_url: String,
    pub api_key: String,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            gemini_base_url: "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent".to_string(),
            api_key: env::var("GEMINI_API_KEY").unwrap_or_default(),
        }
    }
}

pub async fn proxy_gemini(
    req_body: web::Json<GeminiRequest>,
    client: web::Data<Client>,
    config: web::Data<ProxyConfig>,
) -> Result<HttpResponse, Error> {
    let url = format!("{}?key={}", config.gemini_base_url, config.api_key);

    let response = client
        .post(&url)
        .json(&req_body.into_inner())
        .send()
        .await
        .map_err(|e| {
            eprintln!("Request error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to forward request")
        })?;

    if response.status().is_success() {
        let gemini_response: serde_json::Value = response.json().await.map_err(|e| {
            eprintln!("JSON parse error: {}", e);
            actix_web::error::ErrorInternalServerError("Failed to parse response")
        })?;
        Ok(HttpResponse::Ok().json(gemini_response))
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_default();
        eprintln!("Gemini API error: {} - {}", status, error_text);
        Ok(HttpResponse::build(status).body(error_text))
    }
}

pub fn create_app(client: Client, config: ProxyConfig) -> App<impl actix_web::dev::ServiceFactory<
    actix_web::dev::ServiceRequest,
    Config = (),
    Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
    Error = actix_web::Error,
    InitError = (),
>> {
    App::new()
        .app_data(web::Data::new(client))
        .app_data(web::Data::new(config))
        .route("/proxy", web::post().to(proxy_gemini))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let client = Client::new();
    let config = ProxyConfig::default();

    HttpServer::new(move || create_app(client.clone(), config.clone()))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, http::StatusCode};
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::{method, path};

    #[actix_rt::test]
    async fn test_gemini_request_serialization() {
        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: "Hello, Gemini!".to_string(),
                }],
            }],
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Hello, Gemini!"));
        assert!(json.contains("contents"));
        assert!(json.contains("parts"));
        assert!(json.contains("text"));
    }

    #[actix_rt::test]
    async fn test_gemini_request_deserialization() {
        let json = r#"{"contents":[{"parts":[{"text":"Test message"}]}]}"#;
        let request: GeminiRequest = serde_json::from_str(json).unwrap();

        assert_eq!(request.contents.len(), 1);
        assert_eq!(request.contents[0].parts.len(), 1);
        assert_eq!(request.contents[0].parts[0].text, "Test message");
    }

    #[actix_rt::test]
    async fn test_proxy_response_serialization() {
        let response = ProxyResponse {
            candidates: vec![Candidate {
                content: Content {
                    parts: vec![Part {
                        text: "Response from Gemini".to_string(),
                    }],
                },
            }],
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Response from Gemini"));
        assert!(json.contains("candidates"));
    }

    #[actix_rt::test]
    async fn test_proxy_endpoint_success() {
        // Start a mock server to simulate Gemini API
        let mock_server = MockServer::start().await;

        // Set up the mock response
        let mock_response = serde_json::json!({
            "candidates": [{
                "content": {
                    "parts": [{"text": "Hello! How can I help you today?"}]
                }
            }]
        });

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&mock_response))
            .mount(&mock_server)
            .await;

        // Create test app with mock server URL
        let client = Client::new();
        let config = ProxyConfig {
            gemini_base_url: mock_server.uri(),
            api_key: "test_api_key".to_string(),
        };

        let app = test::init_service(create_app(client, config)).await;

        // Create test request
        let req_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: "Hello!".to_string(),
                }],
            }],
        };

        let req = test::TestRequest::post()
            .uri("/proxy")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        let body: serde_json::Value = test::read_body_json(resp).await;
        assert!(body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap()
            .contains("Hello"));
    }

    #[actix_rt::test]
    async fn test_proxy_endpoint_api_error() {
        // Start a mock server to simulate Gemini API error
        let mock_server = MockServer::start().await;

        Mock::given(method("POST"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(400).set_body_string("Bad Request: Invalid API key"))
            .mount(&mock_server)
            .await;

        let client = Client::new();
        let config = ProxyConfig {
            gemini_base_url: mock_server.uri(),
            api_key: "invalid_key".to_string(),
        };

        let app = test::init_service(create_app(client, config)).await;

        let req_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: "Hello!".to_string(),
                }],
            }],
        };

        let req = test::TestRequest::post()
            .uri("/proxy")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_rt::test]
    async fn test_proxy_endpoint_invalid_json() {
        let client = Client::new();
        let config = ProxyConfig {
            gemini_base_url: "http://localhost:9999".to_string(),
            api_key: "test_key".to_string(),
        };

        let app = test::init_service(create_app(client, config)).await;

        let req = test::TestRequest::post()
            .uri("/proxy")
            .set_payload("invalid json")
            .insert_header(("content-type", "application/json"))
            .to_request();

        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    /// Integration test that calls the real Gemini API.
    /// This test is ignored by default and requires GEMINI_API_KEY to be set.
    /// Run with: cargo test test_real_gemini_api -- --ignored
    #[actix_rt::test]
    #[ignore]
    async fn test_real_gemini_api() {
        dotenvy::dotenv().ok();
        
        let api_key = env::var("GEMINI_API_KEY")
            .expect("GEMINI_API_KEY must be set to run this test");
        
        assert!(!api_key.is_empty(), "GEMINI_API_KEY cannot be empty");

        let client = Client::new();
        let config = ProxyConfig::default();

        let app = test::init_service(create_app(client, config)).await;

        // Create a simple request to the Gemini API
        let req_body = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: "Say 'Hello, test!' and nothing else.".to_string(),
                }],
            }],
        };

        let req = test::TestRequest::post()
            .uri("/proxy")
            .set_json(&req_body)
            .to_request();

        let resp = test::call_service(&app, req).await;

        println!("Response status: {:?}", resp.status());
        
        assert_eq!(resp.status(), StatusCode::OK, "Expected OK response from Gemini API");

        let body: serde_json::Value = test::read_body_json(resp).await;
        println!("Response body: {}", serde_json::to_string_pretty(&body).unwrap());

        // Verify the response has the expected structure
        assert!(body.get("candidates").is_some(), "Response should have 'candidates' field");
        
        let candidates = body["candidates"].as_array().expect("candidates should be an array");
        assert!(!candidates.is_empty(), "candidates should not be empty");
        
        // Check that we got some text back
        let text = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .expect("Response should contain text");
        
        println!("Gemini response text: {}", text);
        assert!(!text.is_empty(), "Response text should not be empty");
    }

    /// Test that directly calls the Gemini API without going through the proxy
    /// to verify the API key and connection work.
    /// Run with: cargo test test_direct_gemini_api_call -- --ignored
    #[actix_rt::test]
    #[ignore]
    async fn test_direct_gemini_api_call() {
        dotenvy::dotenv().ok();
        
        let api_key = env::var("GEMINI_API_KEY")
            .expect("GEMINI_API_KEY must be set to run this test");
        
        let client = Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent?key={}",
            api_key
        );

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part {
                    text: "What is 2 + 2? Answer with just the number.".to_string(),
                }],
            }],
        };

        let response = client
            .post(&url)
            .json(&request)
            .send()
            .await
            .expect("Failed to send request to Gemini API");

        println!("Direct API response status: {:?}", response.status());
        
        assert!(response.status().is_success(), "Direct API call should succeed");

        let body: serde_json::Value = response
            .json()
            .await
            .expect("Failed to parse Gemini API response");

        println!("Direct API response: {}", serde_json::to_string_pretty(&body).unwrap());

        // Verify we got a valid response
        assert!(body.get("candidates").is_some(), "Response should have candidates");
        
        let text = body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .expect("Response should contain text");
        
        println!("Gemini says 2+2 = {}", text.trim());
        assert!(text.contains("4"), "Gemini should correctly answer 2+2=4");
    }
}
