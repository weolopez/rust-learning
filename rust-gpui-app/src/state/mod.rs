//! State management module for the application.
//!
//! This module provides centralized state management through the AppState struct.
//! It handles global application state that needs to be shared across views.

use chrono::{DateTime, Utc};
use gpui::SharedString;
use crate::theme::Theme;

/// Role of a chat message
#[derive(Clone, Debug, PartialEq)]
pub enum MessageRole {
    User,
    Assistant,
}

/// Individual chat message
#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub id: u64,
    pub role: MessageRole,
    pub content: SharedString,
    pub timestamp: DateTime<Utc>,
}

/// Chat-specific state
#[derive(Clone, Debug)]
pub struct ChatState {
    pub messages: Vec<ChatMessage>,
    pub is_loading: bool,
    pub error: Option<String>,
    pub api_key: Option<String>,
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
            is_loading: false,
            error: None,
            api_key: None,
        }
    }
}

/// Global application state.
///
/// This struct holds all shared state that needs to be accessible
/// across different views and components in the application.
///
/// # Example
///
/// ```
/// use crate::state::AppState;
///
/// let state = AppState::new("My App");
/// println!("App title: {}", state.title);
/// ```
#[derive(Clone)]
pub struct AppState {
    /// Application title or greeting text
    pub title: SharedString,
    /// Current view/route name (for future navigation)
    pub current_view: SharedString,
    /// Whether the app is in dark mode
    pub dark_mode: bool,
    /// Chat state for LLM interactions
    pub chat: ChatState,
    /// Application theme
    pub theme: Theme,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            title: "World".into(),
            current_view: "home".into(),
            dark_mode: true,
            chat: ChatState::new(),
            theme: Theme::default(),
        }
    }
}

impl AppState {
    /// Creates a new AppState with the given title.
    ///
    /// # Arguments
    /// * `title` - The title or greeting text for the app
    ///
    /// # Example
    /// ```
    /// let state = AppState::new("User");
    /// ```
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            ..Default::default()
        }
    }

    /// Updates the current view.
    ///
    /// # Arguments
    /// * `view` - The name of the view to navigate to
    pub fn navigate_to(&mut self, view: impl Into<SharedString>) {
        self.current_view = view.into();
    }

    /// Toggles dark mode.
    pub fn toggle_dark_mode(&mut self) {
        self.dark_mode = !self.dark_mode;
    }

    /// Sets the application title.
    ///
    /// # Arguments
    /// * `title` - The new title text
    pub fn set_title(&mut self, title: impl Into<SharedString>) {
        self.title = title.into();
    }
}