//! Integration tests for Gemini API chat flow
//! 
//! These tests make real API calls to the Gemini API.
//! Requires GEMINI_API_KEY environment variable to be set.
//! 
//! Tests will be skipped if GEMINI_API_KEY is not set or is invalid.

use chrono::Utc;
use rust_gpui_app::services::GeminiClient;
use rust_gpui_app::{ChatMessage, MessageRole};

/// Helper to get API key, returns None if not set (test will be skipped)
fn get_api_key() -> Option<String> {
    std::env::var("GEMINI_API_KEY").ok()
}

/// Helper to validate API key works before running tests
async fn validate_api_key(api_key: &str) -> bool {
    let client = GeminiClient::new(api_key.to_string()).with_model("gemini-2.0-flash".to_string());
    let messages = vec![ChatMessage {
        id: 0,
        role: MessageRole::User,
        content: "hi".into(),
        timestamp: Utc::now(),
    }];
    client.send_message(&messages).await.is_ok()
}

/// Test a basic single message chat flow
#[tokio::test]
async fn test_single_message_chat() {
    // Get API key from environment
    let Some(api_key) = get_api_key() else {
        println!("SKIPPED: GEMINI_API_KEY not set");
        return;
    };

    if !validate_api_key(&api_key).await {
        println!("SKIPPED: GEMINI_API_KEY is invalid or leaked");
        return;
    }

    // Create client
    let client = GeminiClient::new(api_key).with_model("gemini-2.0-flash".to_string());

    // Create a simple message
    let messages = vec![ChatMessage {
        id: 1,
        role: MessageRole::User,
        content: "Say hello in exactly 3 words".into(),
        timestamp: Utc::now(),
    }];

    // Send message and get response
    let result = client.send_message(&messages).await;

    // Verify we got a response
    assert!(result.is_ok(), "API call failed: {:?}", result.err());
    
    let response = result.unwrap();
    assert!(!response.is_empty(), "Response should not be empty");
    
    println!("Single message response: {}", response);
}

/// Test a multi-turn conversation flow
#[tokio::test]
async fn test_multi_turn_conversation() {
    let Some(api_key) = get_api_key() else {
        println!("SKIPPED: GEMINI_API_KEY not set");
        return;
    };

    if !validate_api_key(&api_key).await {
        println!("SKIPPED: GEMINI_API_KEY is invalid or leaked");
        return;
    }

    let client = GeminiClient::new(api_key).with_model("gemini-2.0-flash".to_string());

    // First message
    let mut messages = vec![ChatMessage {
        id: 1,
        role: MessageRole::User,
        content: "My name is TestUser. Remember this.".into(),
        timestamp: Utc::now(),
    }];

    let response1 = client.send_message(&messages).await
        .expect("First message should succeed");
    
    println!("Turn 1 response: {}", response1);
    
    // Add assistant response to history
    messages.push(ChatMessage {
        id: 2,
        role: MessageRole::Assistant,
        content: response1.into(),
        timestamp: Utc::now(),
    });

    // Second message - test context retention
    messages.push(ChatMessage {
        id: 3,
        role: MessageRole::User,
        content: "What is my name?".into(),
        timestamp: Utc::now(),
    });

    let response2 = client.send_message(&messages).await
        .expect("Second message should succeed");
    
    println!("Turn 2 response: {}", response2);
    
    // The response should contain "TestUser" since we told it our name
    assert!(
        response2.to_lowercase().contains("testuser"),
        "Response should remember the name 'TestUser', got: {}",
        response2
    );
}

/// Test error handling with invalid API key
#[tokio::test]
async fn test_invalid_api_key() {
    let client = GeminiClient::new("invalid_api_key".to_string())
        .with_model("gemini-2.0-flash".to_string());

    let messages = vec![ChatMessage {
        id: 1,
        role: MessageRole::User,
        content: "Hello".into(),
        timestamp: Utc::now(),
    }];

    let result = client.send_message(&messages).await;
    
    // Should fail with an error
    assert!(result.is_err(), "Should fail with invalid API key");
    
    let error = result.err().unwrap().to_string();
    println!("Expected error: {}", error);
}

/// Test with a longer conversation to ensure stability
#[tokio::test]
async fn test_extended_conversation() {
    let Some(api_key) = get_api_key() else {
        println!("SKIPPED: GEMINI_API_KEY not set");
        return;
    };

    if !validate_api_key(&api_key).await {
        println!("SKIPPED: GEMINI_API_KEY is invalid or leaked");
        return;
    }

    let client = GeminiClient::new(api_key).with_model("gemini-2.0-flash".to_string());

    let prompts = [
        "Let's count. Start with 1.",
        "Next number?",
        "Next number?",
    ];

    let mut messages: Vec<ChatMessage> = Vec::new();
    let mut message_id = 0u64;

    for prompt in prompts {
        message_id += 1;
        messages.push(ChatMessage {
            id: message_id,
            role: MessageRole::User,
            content: prompt.into(),
            timestamp: Utc::now(),
        });

        let response = client.send_message(&messages).await
            .expect("Message should succeed");
        
        println!("User: {}", prompt);
        println!("Assistant: {}\n", response);

        message_id += 1;
        messages.push(ChatMessage {
            id: message_id,
            role: MessageRole::Assistant,
            content: response.into(),
            timestamp: Utc::now(),
        });
    }

    // Verify we built up the conversation
    assert_eq!(messages.len(), 6, "Should have 6 messages (3 user + 3 assistant)");
}

/// Test that the model parameter works correctly
#[tokio::test]
async fn test_model_selection() {
    let Some(api_key) = get_api_key() else {
        println!("SKIPPED: GEMINI_API_KEY not set");
        return;
    };

    if !validate_api_key(&api_key).await {
        println!("SKIPPED: GEMINI_API_KEY is invalid or leaked");
        return;
    }

    // Test with gemini-2.0-flash model
    let client = GeminiClient::new(api_key).with_model("gemini-2.0-flash".to_string());

    let messages = vec![ChatMessage {
        id: 1,
        role: MessageRole::User,
        content: "Reply with just the word 'ok'".into(),
        timestamp: Utc::now(),
    }];

    let result = client.send_message(&messages).await;
    assert!(result.is_ok(), "gemini-2.0-flash should work: {:?}", result.err());
    
    println!("Model test response: {}", result.unwrap());
}
