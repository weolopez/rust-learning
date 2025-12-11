//! Chat component for displaying and managing a chat interface.
//!
//! This module provides a ChatView component that orchestrates
//! the MessagesArea, ChatInput, and GeminiService components.

use gpui::{
    prelude::*,
    div, IntoElement, ParentElement, Styled, Window,
    Entity,
};
use crate::theme::colors;
use super::chat_input::{ChatInput, ChatInputEvent};
use super::messages_area::{MessagesArea, MessageEvent};
use crate::services::gemini_service::{GeminiService, GeminiServiceEvent};

/// A chat view component that orchestrates messages, input, and AI service
pub struct ChatView {
    /// Messages area component
    messages_area: Entity<MessagesArea>,
    /// Chat input component
    chat_input: Entity<ChatInput>,
    /// Gemini service for AI responses
    gemini_service: Entity<GeminiService>,
}

impl ChatView {
    /// Create a new chat view
    pub fn new(cx: &mut Context<Self>) -> Self {
        let messages_area = cx.new(|cx| MessagesArea::new(cx));
        let chat_input = cx.new(|cx| ChatInput::new(cx));
        let gemini_service = cx.new(|cx| GeminiService::new(cx));
        
        // Subscribe to chat input events - forward to both messages area and gemini service
        let messages_area_clone = messages_area.clone();
        let gemini_service_clone = gemini_service.clone();
        cx.subscribe(&chat_input, move |_this, _emitter, event: &ChatInputEvent, cx| {
            match event {
                ChatInputEvent::SendMessage(text) => {
                    // Add user message to messages area
                    messages_area_clone.update(cx, |area, cx| {
                        area.add_message(super::messages_area::ChatMessage::user(text.clone()));
                        cx.notify();
                    });
                    
                    // Send to gemini service for processing
                    gemini_service_clone.update(cx, |service, cx| {
                        service.handle_chat_input(event, cx);
                    });
                }
            }
        }).detach();
        
        // Subscribe to gemini service events - forward assistant messages to messages area
        let messages_area_clone2 = messages_area.clone();
        cx.subscribe(&gemini_service, move |_this, _emitter, event: &GeminiServiceEvent, cx| {
            match event {
                // GeminiServiceEvent::AssistantMessage(text) => {
                    // messages_area_clone2.update(cx, |area, cx| {
                    //     area.add_message(super::messages_area::ChatMessage::assistant(text.clone()));
                    //     cx.notify();
                    // });
                // }
                // Use structured blocks to render code and rich content properly
                GeminiServiceEvent::AssistantMessageParsed(blocks) => {
                    messages_area_clone2.update(cx, |area, cx| {
                        area.add_message(super::messages_area::ChatMessage::assistant_with_blocks(blocks.clone()));
                        cx.notify();
                    });
                }
                GeminiServiceEvent::Error(error) => {
                    messages_area_clone2.update(cx, |area, cx| {
                        area.add_message(super::messages_area::ChatMessage::assistant(
                            format!("Error: {}", error)
                        ));
                        cx.notify();
                    });
                }
                GeminiServiceEvent::Processing => {
                    // Could show a loading indicator
                }
            }
        }).detach();
        
        Self {
            messages_area,
            chat_input,
            gemini_service,
        }
    }
}

impl Render for ChatView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("chat-view")
            .flex()
            .flex_col()
            .size_full()
            .bg(colors::background())
            // Messages area
            .child(self.messages_area.clone())
            // Chat input area
            .child(self.chat_input.clone())
    }
}
