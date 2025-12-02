//! Gemini API client for interacting with Google's Gemini models.
//!
//! This module provides functionality to send messages to and receive responses
//! from Google's Gemini AI models, with support for both regular and streaming responses.

use crate::state::{ChatMessage, MessageRole};
use gpui::SharedString;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Gemini API request structure
#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
    generation_config: Option<GenerationConfig>,
    safety_settings: Option<Vec<SafetySetting>>,
}

/// Content for Gemini API
#[derive(Serialize)]
struct GeminiContent {
    role: String,
    parts: Vec<GeminiPart>,
}

/// Part of a Gemini message
#[derive(Serialize)]
struct GeminiPart {
    text: String,
}

/// Generation configuration
#[derive(Serialize)]
struct GenerationConfig {
    temperature: Option<f32>,
    top_k: Option<i32>,
    top_p: Option<f32>,
    max_output_tokens: Option<i32>,
}

/// Safety settings for content filtering
#[derive(Serialize)]
struct SafetySetting {
    category: String,
    threshold: String,
}

/// Gemini API response structure
#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

/// Candidate response from Gemini
#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContentResponse,
}

/// Content in Gemini response
#[derive(Deserialize)]
struct GeminiContentResponse {
    parts: Vec<GeminiPartResponse>,
}

/// Part in Gemini response
#[derive(Deserialize)]
struct GeminiPartResponse {
    text: String,
}

/// Gemini API error response
#[derive(Deserialize)]
struct GeminiError {
    error: GeminiErrorDetails,
}

#[derive(Deserialize)]
struct GeminiErrorDetails {
    code: i32,
    message: String,
}

/// Client for interacting with Gemini API
pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiClient {
    /// Create a new Gemini client
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            model: "gemini-pro".to_string(), // Default model
        }
    }

    /// Set the model to use
    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    /// Send a message and get a response
    pub async fn send_message(
        &self,
        messages: &[ChatMessage],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model, self.api_key
        );

        let contents = messages
            .iter()
            .map(|msg| GeminiContent {
                role: match msg.role {
                    MessageRole::User => "user".to_string(),
                    MessageRole::Assistant => "model".to_string(),
                },
                parts: vec![GeminiPart {
                    text: msg.content.to_string(),
                }],
            })
            .collect();

        let request = GeminiRequest {
            contents,
            generation_config: Some(GenerationConfig {
                temperature: Some(0.7),
                top_k: Some(40),
                top_p: Some(0.95),
                max_output_tokens: Some(1024),
            }),
            safety_settings: None,
        };

        let response = self.client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            if let Ok(error) = serde_json::from_str::<GeminiError>(&error_text) {
                return Err(format!("Gemini API error: {}", error.error.message).into());
            } else {
                return Err(format!("HTTP error {}: {}", status, error_text).into());
            }
        }

        let gemini_response: GeminiResponse = response.json().await?;
        let text = gemini_response
            .candidates
            .first()
            .and_then(|candidate| candidate.content.parts.first())
            .map(|part| part.text.clone())
            .unwrap_or_else(|| "No response generated".to_string());

        Ok(text)
    }

    /// Send a message with streaming response (placeholder for future implementation)
    pub async fn stream_message(
        &self,
        messages: &[ChatMessage],
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // For now, just call the regular send_message
        // Streaming implementation would require processing Server-Sent Events
        self.send_message(messages).await
    }
}

/// Convenience function to send a message using default client
pub async fn send_message(
    api_key: &str,
    messages: &[ChatMessage],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = GeminiClient::new(api_key.to_string());
    client.send_message(messages).await
}

/// Convenience function for streaming messages
pub async fn stream_message(
    api_key: &str,
    messages: &[ChatMessage],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let client = GeminiClient::new(api_key.to_string());
    client.stream_message(messages).await
}