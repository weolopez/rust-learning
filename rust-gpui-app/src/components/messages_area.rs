//! Messages area component for displaying chat messages.
//!
//! This component displays a scrollable list of chat messages
//! and handles message events to update the display.

use gpui::{
    prelude::*,
    div, IntoElement, ParentElement, SharedString, Styled, Window,
    px, rgb, EventEmitter,
};
use crate::theme::colors;

/// Events emitted by the messages area
#[derive(Clone, Debug)]
pub enum MessagesAreaEvent {
    /// Request to add a user message
    AddUserMessage(String),
    /// Request to add an assistant message
    AddAssistantMessage(String),
}

/// Events that the messages area can receive
#[derive(Clone, Debug)]
pub enum MessageEvent {
    /// Add a user message
    UserMessage(String),
    /// Add an assistant message
    AssistantMessage(String),
}

/// A single chat message
#[derive(Clone)]
pub struct ChatMessage {
    /// The message content
    pub content: SharedString,
    /// Whether this message is from the user
    pub is_user: bool,
}

impl ChatMessage {
    /// Create a new user message
    pub fn user(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            is_user: true,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            is_user: false,
        }
    }
}

/// A messages area component that displays chat messages
pub struct MessagesArea {
    /// The list of messages
    messages: Vec<ChatMessage>,
}

impl MessagesArea {
    /// Create a new messages area
    pub fn new(cx: &mut Context<Self>) -> Self {
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
    fn handle_message_event(&mut self, event: &MessageEvent, cx: &mut Context<Self>) {
        match event {
            MessageEvent::UserMessage(content) => {
                self.add_message(ChatMessage::user(content.clone()));
            }
            MessageEvent::AssistantMessage(content) => {
                self.add_message(ChatMessage::assistant(content.clone()));
            }
        }
        cx.notify();
    }

    /// Send a user message and fetch an AI response
    pub fn send_message_and_get_ai_response(&mut self, user_message: String, cx: &mut Context<Self>) {
        // Add the user message to the display
        self.add_message(ChatMessage::user(user_message.clone()));
        cx.notify();

        // Simulate an asynchronous task to fetch the AI response
        cx.spawn(async move |this: gpui::WeakEntity<MessagesArea>, cx| {
            // Simulate a delay for the AI response (replace with actual API call)
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            let ai_response = format!("AI Response to: {}", user_message);

            // Update the MessagesArea with the AI response
            this.update(cx, |area, _| {
                area.add_message(ChatMessage::assistant(ai_response));
            });
        });
    }
}

impl EventEmitter<MessagesAreaEvent> for MessagesArea {}

impl Render for MessagesArea {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let messages: Vec<_> = self.messages.iter().cloned().collect();
        let has_messages = !messages.is_empty();

        div()
            .id("messages-area")
            .flex()
            .flex_col()
            .flex_grow()
            .overflow_y_scroll()
            .p_4()
            .gap_2()
            .children(messages.into_iter().enumerate().map(|(i, msg)| {
                div()
                    .id(i)
                    .p_3()
                    .rounded_lg()
                    .max_w(px(500.0))
                    .when(msg.is_user, |d| {
                        d.ml_auto()
                            .bg(colors::primary())
                            .text_color(rgb(0xffffff))
                    })
                    .when(!msg.is_user, |d| {
                        d.mr_auto()
                            .bg(colors::surface())
                            .text_color(colors::text())
                    })
                    .child(msg.content.clone())
            }))
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