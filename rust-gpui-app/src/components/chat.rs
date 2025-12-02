//! Chat component for interacting with LLM models.
//!
//! This component provides a chat interface for LLM conversations
//! with text input and message display.

use gpui::{
    prelude::*,
    div, IntoElement, ParentElement, SharedString, Styled, Window,
    px, rgb, Entity, MouseButton,
};
use crate::state::{ChatMessage, MessageRole};
use crate::theme::colors;
use super::text_input::TextInput;
use chrono::Utc;

/// A chat component with text input
pub struct ChatView {
    /// Chat messages
    messages: Vec<ChatMessage>,
    /// Text input entity
    text_input: Entity<TextInput>,
    /// Next message ID
    next_id: u64,
}

impl ChatView {
    /// Create a new chat view
    pub fn new(cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(|cx| TextInput::new(cx, "Type a message..."));
        Self {
            messages: Vec::new(),
            text_input,
            next_id: 1,
        }
    }

    /// Send the current message
    fn send_message(&mut self, cx: &mut Context<Self>) {
        let text = self.text_input.read(cx).text().to_string();
        if !text.trim().is_empty() {
            // Add user message
            self.messages.push(ChatMessage {
                id: self.next_id,
                role: MessageRole::User,
                content: text.into(),
                timestamp: Utc::now(),
            });
            self.next_id += 1;
            
            // Clear input
            self.text_input.update(cx, |input, _cx| {
                input.clear();
            });
        }
        cx.notify();
    }
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let bg_color = colors::background();
        let text_color = colors::text();
        
        div()
            .flex()
            .flex_col()
            .h_full()
            .bg(bg_color)
            .child(
                // Messages area
                div()
                    .id("messages-area")
                    .flex_grow()
                    .p_4()
                    .children(
                        self.messages.iter().map(|msg| {
                            let is_user = msg.role == MessageRole::User;
                            div()
                                .mb_4()
                                .flex()
                                .flex_col()
                                .when(is_user, |d| d.items_end())
                                .child(
                                    div()
                                        .px_3()
                                        .py_2()
                                        .rounded_lg()
                                        .bg(if is_user { colors::primary() } else { colors::surface() })
                                        .text_color(text_color)
                                        .max_w(px(600.0))
                                        .child(msg.content.clone())
                                )
                        })
                    )
                    .when(self.messages.is_empty(), |d| {
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
            .child(
                // Input area
                div()
                    .flex()
                    .items_end()
                    .gap_2()
                    .p_4()
                    .border_t_1()
                    .border_color(colors::border())
                    .child(self.text_input.clone())
                    .child(
                        div()
                            .id("send-button")
                            .px_4()
                            .py_2()
                            .bg(colors::primary())
                            .rounded_lg()
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                this.send_message(cx);
                            }))
                            .child("Send")
                    )
            )
    }
}
