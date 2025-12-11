//! Components module - reusable UI building blocks.
//!
//! This module exports all reusable components that can be composed
//! together to build views. Components are designed to be:
//! - Self-contained
//! - Reusable across different views
//! - Consistently styled using the theme
//!
//! # Available Components
//!
//! - [`button`] - Interactive button elements
//! - [`card`] - Container components with styling
//! - [`color_swatch`] - Color display elements
//!
//! # Example
//!
//! ```
//! use crate::components::{button, card, color_swatch};
//! use gpui::red;
//!
//! let my_card = card()
//!     .child(button("Click me"))
//!     .child(color_swatch(red()));
//! ```

pub mod button;
pub mod card;
pub mod chat;
pub mod chat_input;
pub mod color_swatch;
pub mod message_item;
pub mod messages_area;
pub mod text_input;

// Re-export component functions for convenient access
pub use button::{button, button_outline, button_secondary, button_small};
pub use card::{card, card_centered, card_full, card_sized};
pub use chat::ChatView;
pub use chat_input::{ChatInput, ChatInputEvent};
pub use color_swatch::{color_swatch, color_swatch_row, color_swatch_sized};
pub use message_item::{ChatMessage, ContentBlock, ExecutionStatus, MessageAction};
pub use messages_area::{MessagesArea, MessagesAreaEvent, MessageEvent};
pub use text_input::TextInput;