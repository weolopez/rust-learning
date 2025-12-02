//! Chat component for displaying and managing a chat interface.
//!
//! This module provides a ChatView component that displays a list of messages
//! and handles user interactions using the ChatInput component.

use gpui::{
    prelude::*,
    div, IntoElement, ParentElement, SharedString, Styled, Window,
    Entity,
    px, rgb,
};
use crate::theme::colors;
use super::chat_input::{ChatInput, ChatInputEvent};

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

/// A chat view component that displays messages and handles input
pub struct ChatView {
    /// The list of messages
    messages: Vec<ChatMessage>,
    /// Chat input component entity
    chat_input: Entity<ChatInput>,
}

impl ChatView {
    /// Create a new chat view
    pub fn new(cx: &mut Context<Self>) -> Self {
        let chat_input = cx.new(|cx| ChatInput::new(cx));
        
        // Subscribe to chat input events
        cx.subscribe(&chat_input, |this, _emitter, event: &ChatInputEvent, cx| {
            match event {
                ChatInputEvent::SendMessage(text) => {
                    this.handle_send_message(text.clone(), cx);
                }
            }
        }).detach();
        
        Self {
            messages: vec![
                ChatMessage::assistant("Hello! How can I help you today?"),
            ],
            chat_input,
        }
    }

    /// Add a message to the chat
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    /// Handle a send message event from the chat input
    fn handle_send_message(&mut self, text: String, cx: &mut Context<Self>) {
        if !text.trim().is_empty() {
            self.add_message(ChatMessage::user(text.clone()));
            // TODO: Send to AI and get response
            self.add_message(ChatMessage::assistant(format!("You said: {}", text)));
        }
        cx.notify();
    }
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let messages: Vec<_> = self.messages.iter().cloned().collect();
        let has_messages = !messages.is_empty();
        
        div()
            .id("chat-view")
            .flex()
            .flex_col()
            .size_full()
            .bg(colors::background())
            // Messages area
            .child(
                div()
                    .id("messages-container")
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
            )
            // Chat input area (handles its own send button)
            .child(self.chat_input.clone())
    }
}
