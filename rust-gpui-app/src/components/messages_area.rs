//! Messages area component for displaying chat messages.
//!
//! This component displays a scrollable list of chat messages
//! and handles message events to update the display.

use gpui::{
    prelude::*,
    div, IntoElement, ParentElement, Styled, Window,
    EventEmitter, ClipboardItem,
};
use crate::theme::colors;

// Re-export message item types
pub use super::message_item::{
    ChatMessage, ContentBlock, ExecutionStatus, MessageAction,
};

// --- Events ---

/// Events emitted by the messages area
#[derive(Clone, Debug)]
pub enum MessagesAreaEvent {
    /// Request to add a user message
    AddUserMessage(String),
    /// Request to add an assistant message
    AddAssistantMessage(String),
    /// Action triggered from message UI
    MessageAction(MessageAction),
}

/// Events that the messages area can receive
#[derive(Clone, Debug)]
pub enum MessageEvent {
    /// Add a user message
    UserMessage(String),
    /// Add an assistant message
    AssistantMessage(String),
    /// Update streaming state
    StreamingUpdate { message_id: String, content: String },
    /// Complete streaming
    StreamingComplete(String),
}

// --- Messages Area Component ---

/// A messages area component that displays chat messages
pub struct MessagesArea {
    /// The list of messages
    messages: Vec<ChatMessage>,
}

impl MessagesArea {
    /// Create a new messages area
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            messages: vec![
                ChatMessage::assistant("Hello! How can I help you today?"),
            ],
        }
    }

    /// Add a message to the display
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    /// Handle message events
    #[allow(dead_code)]
    fn handle_message_event(&mut self, event: &MessageEvent, cx: &mut Context<Self>) {
        match event {
            MessageEvent::UserMessage(content) => {
                self.add_message(ChatMessage::user(content.clone()));
            }
            MessageEvent::AssistantMessage(content) => {
                self.add_message(ChatMessage::assistant(content.clone()));
            }
            MessageEvent::StreamingUpdate { message_id, content } => {
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    if let Some(ContentBlock::Text(text)) = msg.blocks.last_mut() {
                        *text = content.clone().into();
                    }
                    msg.is_streaming = true;
                }
            }
            MessageEvent::StreamingComplete(message_id) => {
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    msg.is_streaming = false;
                }
            }
        }
        cx.notify();
    }

    /// Handle message actions
    pub fn handle_action(&mut self, action: &MessageAction, cx: &mut Context<Self>) {
        match action {
            MessageAction::CopyText(text) => {
                cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
            }
            MessageAction::RateMessage { message_id, is_positive } => {
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    msg.feedback = Some(*is_positive);
                    cx.notify();
                }
            }
            MessageAction::ExecuteCode { message_id, code: _ } => {
                // Set status to running
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    for block in &mut msg.blocks {
                        if let ContentBlock::Code { execution_status, .. } = block {
                            *execution_status = ExecutionStatus::Running;
                        }
                    }
                }
                cx.notify();

                // Spawn async task to execute code
                let message_id = message_id.clone();
                let _ = cx.spawn(async move |this: gpui::WeakEntity<MessagesArea>, cx| {
                    // Simulate execution delay
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                    let _ = this.update(cx, |area, cx| {
                        if let Some(msg) = area.messages.iter_mut().find(|m| m.id == message_id) {
                            for block in &mut msg.blocks {
                                if let ContentBlock::Code { execution_status, .. } = block {
                                    *execution_status = ExecutionStatus::Success("Output: 42".into());
                                }
                            }
                            cx.notify();
                        }
                    });
                });
            }
            MessageAction::Regenerate(message_id) => {
                // Find and update the message to show thinking state
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    msg.is_thinking = true;
                    msg.blocks.clear();
                    cx.notify();
                }
            }
            MessageAction::EditMessage { message_id, new_content } => {
                // Create a new branch instead of overwriting
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    msg.total_branches += 1;
                    msg.branch_index = msg.total_branches;
                    msg.blocks = vec![ContentBlock::Text(new_content.clone().into())];
                    cx.notify();
                }
            }
            MessageAction::NavigateBranch { message_id, direction } => {
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    let new_index = (msg.branch_index as i32 + direction).max(1).min(msg.total_branches as i32);
                    msg.branch_index = new_index as u32;
                    cx.notify();
                }
            }
            MessageAction::ReadAloud(_message_id) => {
                // TTS implementation would go here
            }
            MessageAction::Share(_message_id) => {
                // Share implementation would go here
            }
        }
    }

    /// Send a user message and fetch an AI response
    pub fn send_message_and_get_ai_response(&mut self, user_message: String, cx: &mut Context<Self>) {
        // Add the user message
        self.add_message(ChatMessage::user(user_message.clone()));

        // Add a thinking message
        let thinking_msg = ChatMessage::thinking();
        let thinking_id = thinking_msg.id.clone();
        self.add_message(thinking_msg);
        cx.notify();

        // Spawn async task for AI response
        let _ = cx.spawn(async move |this: gpui::WeakEntity<MessagesArea>, cx| {
            // Simulate delay
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            let ai_response = format!("AI Response to: {}", user_message);

            let _ = this.update(cx, |area, cx| {
                // Remove thinking message and add real response
                area.messages.retain(|m| m.id != thinking_id);
                area.add_message(ChatMessage::assistant(ai_response));
                cx.notify();
            });
        });
    }
}

impl EventEmitter<MessagesAreaEvent> for MessagesArea {}

impl Render for MessagesArea {
    fn render(&mut self, window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let messages: Vec<_> = self.messages.iter().cloned().collect();
        let has_messages = !messages.is_empty();

        div()
            .id("messages-area")
            .flex()
            .flex_col()
            .flex_grow()
            .overflow_y_scroll()
            .p_4()
            .gap_4()
            // Render messages using ChatMessage's render method
            .children(messages.into_iter().map(|msg| {
                msg.render_message(window)
            }))
            // Empty state
            .when(!has_messages, |d| {
                d.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h_full()
                        .text_color(colors::text_muted())
                        .child("No messages yet. Start a conversation!")
                )
            })
    }
}