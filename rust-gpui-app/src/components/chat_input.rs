//! Chat input component with text field and send button.
//!
//! This component handles text input and emits events when messages are sent.

use gpui::{
    prelude::*,
    div, IntoElement, ParentElement, Styled, Window,
    Entity, MouseButton, EventEmitter, FocusHandle, Focusable, KeyDownEvent,
    px, rgb,
};
use crate::theme::colors;
use super::text_input::TextInput;

/// Events emitted by the chat input component
#[derive(Clone, Debug)]
pub enum ChatInputEvent {
    /// User submitted a message
    SendMessage(String),
}

/// A chat input component with text field and send button
pub struct ChatInput {
    /// Text input entity
    text_input: Entity<TextInput>,
    /// Focus handle
    focus_handle: FocusHandle,
}

impl ChatInput {
    /// Create a new chat input
    pub fn new(cx: &mut Context<Self>) -> Self {
        let text_input = cx.new(|cx| TextInput::new(cx, "Type a message..."));
        Self {
            text_input,
            focus_handle: cx.focus_handle(),
        }
    }

    /// Submit the current message
    fn submit(&mut self, cx: &mut Context<Self>) {
        let text = self.text_input.read(cx).text().to_string();
        if !text.trim().is_empty() {
            // Emit the send message event
            cx.emit(ChatInputEvent::SendMessage(text));
            
            // Clear input
            self.text_input.update(cx, |input, _cx| {
                input.clear();
            });
        }
        cx.notify();
    }

    /// Handle key down for Enter key submission
    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        if event.keystroke.key == "enter" && !event.keystroke.modifiers.shift {
            self.submit(cx);
        }
    }
}

impl EventEmitter<ChatInputEvent> for ChatInput {}

impl Focusable for ChatInput {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for ChatInput {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        
        div()
            .id("chat-input-container")
            .track_focus(&focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                this.handle_key_down(event, cx);
            }))
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
                    .hover(|style| style.bg(colors::secondary()))
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                        this.submit(cx);
                    }))
                    .child("Send")
            )
    }
}