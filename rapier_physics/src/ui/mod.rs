//! UI module
//!
//! Contains all user interface components including chat, controls, and styling.

mod chat;
mod controls;
mod skin;

pub use chat::{ChatMessage, ChatPanel, ChatCommand};
pub use controls::{ControlsPanel, ControlsResult};
pub use skin::create_custom_skin;