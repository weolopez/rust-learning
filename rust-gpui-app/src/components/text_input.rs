//! Text input component for capturing user text.
//!
//! A basic text input field that handles keyboard input and displays text.

use gpui::{
    prelude::*,
    div, IntoElement, ParentElement, SharedString, Styled, Window,
    FocusHandle, Focusable, KeyDownEvent, MouseButton,
    px,
};
use crate::theme::colors;

/// A simple text input component
pub struct TextInput {
    /// The current text content
    text: String,
    /// Placeholder text shown when empty
    placeholder: SharedString,
    /// Focus handle for keyboard events
    focus_handle: FocusHandle,
}

impl TextInput {
    /// Create a new text input with optional placeholder
    pub fn new(cx: &mut Context<Self>, placeholder: impl Into<SharedString>) -> Self {
        Self {
            text: String::new(),
            placeholder: placeholder.into(),
            focus_handle: cx.focus_handle(),
        }
    }

    /// Get the current text
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Set the text content
    pub fn set_text(&mut self, text: impl Into<String>) {
        self.text = text.into();
    }

    /// Clear the text
    pub fn clear(&mut self) {
        self.text.clear();
    }

    /// Check if the input is empty
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Handle a key down event
    fn handle_key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        let key = &event.keystroke.key;
        
        if key == "backspace" {
            self.text.pop();
            cx.notify();
        } else if key == "space" {
            self.text.push(' ');
            cx.notify();
        } else if key == "enter" {
            // Enter is handled by parent
            cx.notify();
        } else if let Some(ref ch) = event.keystroke.key_char {
            // Use key_char for actual character input
            self.text.push_str(ch);
            cx.notify();
        } else if key.len() == 1 {
            // Single character key without key_char
            let ch = if event.keystroke.modifiers.shift {
                key.to_uppercase()
            } else {
                key.to_lowercase()
            };
            self.text.push_str(&ch);
            cx.notify();
        }
    }
}

impl Focusable for TextInput {
    fn focus_handle(&self, _cx: &gpui::App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for TextInput {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focus_handle = self.focus_handle.clone();
        let is_focused = self.focus_handle.is_focused(window);
        let display_text: SharedString = if self.text.is_empty() {
            self.placeholder.clone()
        } else {
            self.text.clone().into()
        };
        let text_color = if self.text.is_empty() {
            colors::text_muted()
        } else {
            colors::text()
        };

        div()
            .id("text-input")
            .track_focus(&focus_handle)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                this.handle_key_down(event, cx);
            }))
            .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _event, window, _cx| {
                window.focus(&focus_handle);
            }))
            .flex_grow()
            .p_2()
            .bg(colors::surface())
            .rounded_lg()
            .border_1()
            .border_color(if is_focused { colors::primary() } else { colors::border() })
            .text_color(text_color)
            .cursor_text()
            .child(
                div()
                    .flex()
                    .items_center()
                    .h_6()
                    .child(display_text)
                    .when(is_focused, |d| {
                        d.child(
                            div()
                                .w(px(2.0))
                                .h_4()
                                .bg(colors::text())
                                .ml_px()
                        )
                    })
            )
    }
}