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
                // ChatMessage::assistant("Hello! How can I help you today?"),
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
    /// This function demonstrates Rust's pattern matching with enums, ownership/borrowing, and async programming
    pub fn handle_action(&mut self, action: &MessageAction, cx: &mut Context<Self>) {
        // 'match' is Rust's powerful pattern matching construct, similar to switch but type-safe and exhaustive
        // Unlike Java's switch, match must cover all possible cases or have a default arm
        match action {
            // Enum variant with data: CopyText contains a &str (string slice)
            // In Rust, &str is a borrowed string view, not owned like Java's String
            MessageAction::CopyText(text) => {
                // Convert &str to owned String using to_string()
                // ClipboardItem::new_string expects an owned String
                cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
            }
            // Struct-like enum variant with named fields
            // { message_id, is_positive } destructures the fields directly
            MessageAction::RateMessage { message_id, is_positive } => {
                // 'if let' is syntactic sugar for pattern matching on Option<T>
                // Option<T> is Rust's way of handling nullable values, like Optional<T> in Java but built-in
                // iter_mut() returns a mutable iterator over the Vec (vector, like ArrayList)
                // find() takes a closure (lambda function) that returns bool
                // Closures capture variables from their environment, similar to Java lambdas
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    // Dereference *is_positive because it's a &bool reference
                    // Some() wraps the value in Option, like Optional.of() in Java
                    msg.feedback = Some(*is_positive);
                    // Notify the UI framework to re-render this component
                    cx.notify();
                }
            }
            // Another enum variant with named fields
            MessageAction::ExecuteCode { message_id, code: _ } => {
                // Set status to running - update UI immediately
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    // Mutable iteration over the vector of content blocks
                    // &mut msg.blocks gives mutable reference to the vector
                    for block in &mut msg.blocks {
                        // Pattern matching on enum variants within the loop
                        // ContentBlock::Code destructures to get execution_status field
                        // .. ignores other fields in the struct
                        if let ContentBlock::Code { execution_status, .. } = block {
                            // Dereference *execution_status to assign to the mutable reference
                            // ExecutionStatus is an enum, similar to Java enums
                            *execution_status = ExecutionStatus::Running;
                        }
                    }
                }
                cx.notify();

                // Clone message_id because it will be moved into the async closure
                // In Rust, ownership prevents using borrowed values in async blocks that may outlive the current scope
                // This is different from Java where everything is reference-based
                let message_id = message_id.clone();

                // Spawn an asynchronous task using GPUI's async runtime
                // 'async move' creates an async closure that moves captured variables into it
                // Similar to Java's CompletableFuture or threads, but integrated with async/await
                let _ = cx.spawn(async move |this: gpui::WeakEntity<MessagesArea>, cx| {
                    // Simulate execution delay using tokio async runtime
                    // .await suspends the current task without blocking the thread
                    // Unlike Java's Thread.sleep() which blocks the thread
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

                    // Update the component from within the async task
                    // 'this' is a WeakEntity to avoid reference cycles (like weak references in Java)
                    // update() provides mutable access to the component
                    let _ = this.update(cx, |area, cx| {
                        // Find the message again in the updated component state
                        if let Some(msg) = area.messages.iter_mut().find(|m| m.id == message_id) {
                            // Update all code blocks to success status
                            for block in &mut msg.blocks {
                                if let ContentBlock::Code { execution_status, .. } = block {
                                    // Create a success result with simulated output
                                    // .into() converts &str to String (owned string)
                                    *execution_status = ExecutionStatus::Success("Output: 42".into());
                                }
                            }
                            // Notify UI to update with the results
                            cx.notify();
                        }
                    });
                });
            }
            // Simple enum variant with single field
            MessageAction::Regenerate(message_id) => {
                // Find and update the message to show thinking state
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    msg.is_thinking = true;
                    // clear() removes all elements from the vector
                    msg.blocks.clear();
                    cx.notify();
                }
            }
            // Enum variant with multiple fields
            MessageAction::EditMessage { message_id, new_content } => {
                // Create a new branch instead of overwriting
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    msg.total_branches += 1;
                    msg.branch_index = msg.total_branches;
                    // vec![] creates a new vector, like Arrays.asList() in Java
                    // clone() creates a copy of the string
                    // .into() converts to the expected type
                    msg.blocks = vec![ContentBlock::Text(new_content.clone().into())];
                    cx.notify();
                }
            }
            MessageAction::NavigateBranch { message_id, direction } => {
                if let Some(msg) = self.messages.iter_mut().find(|m| &m.id == message_id) {
                    // Type casting and method chaining
                    // as i32 converts u32 to i32 for arithmetic
                    // max() and min() are methods on i32, similar to Math.max/min in Java
                    let new_index = (msg.branch_index as i32 + direction).max(1).min(msg.total_branches as i32);
                    msg.branch_index = new_index as u32; // Cast back to u32
                    cx.notify();
                }
            }
            MessageAction::ReadAloud(_message_id) => {
                // TTS implementation would go here
                // _ prefix indicates intentionally unused variable (no warning)
            }
            MessageAction::Share(_message_id) => {
                // Share implementation would go here
            }
        }
    }

    /// Send a user message and fetch an AI response
    /// Demonstrates async programming, ownership transfer, and UI state management in Rust
    pub fn send_message_and_get_ai_response(&mut self, user_message: String, cx: &mut Context<Self>) {
        // Add the user message to the UI immediately
        // ChatMessage::user() is an associated function (static method in Java terms)
        // user_message.clone() creates a copy because String is owned and we need it later
        // In Rust, moving a value prevents further use; cloning creates a duplicate
        self.add_message(ChatMessage::user(user_message.clone()));

        // Create and add a "thinking" indicator message
        // ChatMessage::thinking() returns a new ChatMessage instance
        // let binds an immutable variable by default (like final in Java)
        let thinking_msg = ChatMessage::thinking();
        // Clone the ID because we'll need it in the async closure
        // thinking_msg.id is a String (owned), clone() duplicates it
        let thinking_id = thinking_msg.id.clone();
        // Move thinking_msg into add_message (transfers ownership)
        self.add_message(thinking_msg);
        // Notify the UI framework to re-render with the new messages
        cx.notify();

        // Spawn an asynchronous task to simulate AI response
        // cx.spawn() starts a background task that doesn't block the UI thread
        // async move creates an async closure that takes ownership of captured variables
        // 'move' transfers ownership of user_message and thinking_id into the closure
        let _ = cx.spawn(async move |this: gpui::WeakEntity<MessagesArea>, cx| {
            // Simulate network delay or AI processing time
            // tokio::time::sleep() is non-blocking; suspends this task without blocking threads
            // std::time::Duration represents time spans, similar to Java's Duration
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;

            // Create AI response using format! macro for string interpolation
            // format! is like String.format() in Java but returns owned String
            // {} is a placeholder, automatically replaced with user_message
            let ai_response = format!("AI Response to: {}", user_message);

            // Update the UI component from within the async task
            // this.update() provides safe mutable access to the component
            // The closure |area, cx| receives mutable reference to MessagesArea and context
            let _ = this.update(cx, |area, cx| {
                // Remove the thinking message by filtering the vector
                // retain() keeps only messages where the closure returns true
                // Similar to Java streams: messages.stream().filter(m -> !m.id.equals(thinking_id))
                // But retain() modifies the vector in-place for efficiency
                area.messages.retain(|m| m.id != thinking_id);
                // Add the actual AI response message
                // ChatMessage::assistant() creates an assistant message
                area.add_message(ChatMessage::assistant(ai_response));
                // Notify UI to update the display
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