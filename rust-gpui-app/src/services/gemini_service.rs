//! Gemini service for handling chat input events and generating AI responses.
//!
//! This service subscribes to ChatInputEvent from the chat input component
//! and emits assistant messages after querying the Gemini API.
use dotenv::dotenv;

use gpui::{prelude::*, EventEmitter, SharedString};
use crate::components::chat_input::ChatInputEvent;
use crate::services::gemini::GeminiClient;
use crate::state::{ChatMessage, MessageRole};
use chrono::Utc;

/// Events emitted by the Gemini service
#[derive(Clone, Debug)]
pub enum GeminiServiceEvent {
    /// An assistant message was generated
    AssistantMessage(String),
    /// An error occurred while generating a response
    Error(String),
    /// Processing started
    Processing,
}

/// Gemini service for processing chat messages
pub struct GeminiService {
    /// Whether we're currently processing
    is_processing: bool,
    /// Whether the API is configured
    is_configured: bool,
    /// The API key for Gemini
    api_key: Option<String>,
    /// Conversation history for context
    conversation_history: Vec<ChatMessage>,
    /// Message ID counter
    next_message_id: u64,
}

impl GeminiService {
    /// Create a new Gemini service
    pub fn new(_cx: &mut Context<Self>) -> Self {
        dotenv().ok();
        // Try to get API key from environment
        let api_key = std::env::var("GEMINI_API_KEY").ok();
        
        Self {
            is_processing: false,
            is_configured: api_key.is_some(),
            api_key,
            conversation_history: Vec::new(),
            next_message_id: 0,
        }
    }

    /// Handle a chat input event
    pub fn handle_chat_input(&mut self, event: &ChatInputEvent, cx: &mut Context<Self>) {
        match event {
            ChatInputEvent::SendMessage(text) => {
                self.process_message(text.clone(), cx);
            }
        }
    }

    /// Process a user message and generate a response
    fn process_message(&mut self, text: String, cx: &mut Context<Self>) {
        // Check if already processing
        if self.is_processing {
            return;
        }

        // Emit processing event
        self.is_processing = true;
        cx.emit(GeminiServiceEvent::Processing);
        cx.notify();

        // Check if API key is configured
        let Some(api_key) = self.api_key.clone() else {
            self.is_processing = false;
            cx.emit(GeminiServiceEvent::Error(
                "No Gemini API key configured. Set GEMINI_API_KEY environment variable.".to_string()
            ));
            cx.notify();
            return;
        };

        // Add user message to conversation history
        let user_message = ChatMessage {
            id: self.next_message_id,
            role: MessageRole::User,
            content: SharedString::from(text.clone()),
            timestamp: Utc::now(),
        };
        self.next_message_id += 1;
        self.conversation_history.push(user_message);

        // Clone what we need for the async task
        let messages = self.conversation_history.clone();

        // Spawn async task to call Gemini API
        cx.spawn(async move |this: gpui::WeakEntity<GeminiService>, cx| {
            // reqwest requires a Tokio runtime, so we spawn a blocking task with its own runtime
            let result = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
                .and_then(|rt| {
                    rt.block_on(async {
                        let client = GeminiClient::new(api_key).with_model("gemini-2.0-flash".to_string());
                        client.send_message(&messages).await
                    })
                });

            this.update(cx, |service, inner_cx| {
                service.is_processing = false;

                match result {
                    Ok(response_text) => {
                        // Add assistant message to conversation history
                        let assistant_message = ChatMessage {
                            id: service.next_message_id,
                            role: MessageRole::Assistant,
                            content: SharedString::from(response_text.clone()),
                            timestamp: Utc::now(),
                        };
                        service.next_message_id += 1;
                        service.conversation_history.push(assistant_message);

                        inner_cx.emit(GeminiServiceEvent::AssistantMessage(response_text));
                    }
                    Err(e) => {
                        inner_cx.emit(GeminiServiceEvent::Error(e.to_string()));
                    }
                }
                inner_cx.notify();
            }).ok();
        })
        .detach();
    }

    /// Check if the service is processing
    pub fn is_processing(&self) -> bool {
        self.is_processing
    }

    /// Check if the service has a configured API key
    pub fn is_configured(&self) -> bool {
        self.is_configured
    }

    /// Clear conversation history
    pub fn clear_history(&mut self) {
        self.conversation_history.clear();
    }
}

impl EventEmitter<GeminiServiceEvent> for GeminiService {}