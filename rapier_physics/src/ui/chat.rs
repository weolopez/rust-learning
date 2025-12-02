//! Chat panel UI component
//!
//! Provides a chat interface with message display and command handling.

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use super::{Bounds, HasBounds};

/// A single chat message
#[derive(Clone)]
pub struct ChatMessage {
    pub sender: String,
    pub text: String,
    pub color: Color,
}

impl ChatMessage {
    pub fn new(sender: impl Into<String>, text: impl Into<String>, color: Color) -> Self {
        Self {
            sender: sender.into(),
            text: text.into(),
            color,
        }
    }

    pub fn system(text: impl Into<String>, color: Color) -> Self {
        Self::new("System", text, color)
    }
}

/// Chat command types
#[derive(Debug, Clone, PartialEq)]
pub enum ChatCommand {
    AddBall,
    Clear,
    Help,
    Count,
    None,
}

/// Result of processing chat input
pub struct ChatInputResult {
    pub command: ChatCommand,
    pub message_sent: bool,
}

/// Chat panel state and rendering
pub struct ChatPanel {
    pub messages: Vec<ChatMessage>,
    pub input_text: String,
    pub username: String,
    pub visible: bool,
    /// The current position of the window (macroquad mutates this when dragged!)
    pub window_pos: Vec2,
    /// Flag to track if window has been initialized with screen-relative position
    initialized: bool,
}

impl ChatPanel {
    /// Panel size constants
    pub const WIDTH: f32 = 320.0;
    pub const HEIGHT: f32 = 400.0;
    /// Extra margin for window borders
    pub const MARGIN: f32 = 10.0;

    pub fn new() -> Self {
        Self {
            messages: vec![ChatMessage::system(
                "Welcome to Physics Chat! Type /help for commands.",
                GRAY,
            )],
            input_text: String::new(),
            username: String::from("User"),
            visible: true,
            // Initial position will be set on first render based on screen size
            window_pos: vec2(0.0, 10.0),
            initialized: false,
        }
    }

    /// Initialize window position based on screen size (called on first render)
    fn init_position(&mut self) {
        if !self.initialized {
            self.window_pos.x = screen_width() - Self::WIDTH - Self::MARGIN;
            self.initialized = true;
        }
    }

    /// Toggle chat visibility
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }

    /// Add a message to the chat
    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    /// Add a system message
    pub fn add_system_message(&mut self, text: impl Into<String>, color: Color) {
        self.messages.push(ChatMessage::system(text, color));
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear();
        self.add_system_message("Chat cleared!", GRAY);
    }

    /// Parse a command from input text
    fn parse_command(input: &str) -> ChatCommand {
        match input.trim().to_lowercase().as_str() {
            "/ball" | "/add" => ChatCommand::AddBall,
            "/clear" => ChatCommand::Clear,
            "/help" => ChatCommand::Help,
            "/count" => ChatCommand::Count,
            _ => ChatCommand::None,
        }
    }

    /// Render the chat panel and process input
    /// Returns the command to execute (if any)
    pub fn render(&mut self, ball_count: usize) -> ChatInputResult {
        let mut result = ChatInputResult {
            command: ChatCommand::None,
            message_sent: false,
        };

        if !self.visible {
            return result;
        }

        // Initialize position on first render (needs screen size)
        self.init_position();

        // IMPORTANT: Pass window_pos directly - macroquad will mutate it when dragged!
        widgets::Window::new(hash!(), self.window_pos, vec2(Self::WIDTH, Self::HEIGHT))
            .label("Chat")
            .movable(true)
            .ui(&mut root_ui(), |ui| {
                // Username input
                ui.label(None, "Your name:");
                ui.input_text(hash!(), "", &mut self.username);
                ui.separator();

                // Chat messages area
                widgets::Group::new(hash!(), vec2(Self::WIDTH - 20.0, Self::HEIGHT - 150.0))
                    .ui(ui, |ui| {
                        // Show last 15 messages
                        let start = if self.messages.len() > 15 {
                            self.messages.len() - 15
                        } else {
                            0
                        };
                        for msg in &self.messages[start..] {
                            let formatted = format!("{}: {}", msg.sender, msg.text);
                            ui.label(None, &formatted);
                        }
                    });

                ui.separator();

                // Input area
                ui.label(None, "Message:");
                ui.input_text(hash!(), "", &mut self.input_text);

                if ui.button(None, "Send") {
                    if !self.input_text.trim().is_empty() {
                        let sender_name = if self.username.trim().is_empty() {
                            "User".to_string()
                        } else {
                            self.username.clone()
                        };

                        // Add user message
                        self.messages.push(ChatMessage::new(
                            sender_name,
                            self.input_text.clone(),
                            SKYBLUE,
                        ));

                        // Parse and handle command
                        let command = Self::parse_command(&self.input_text);
                        match command {
                            ChatCommand::AddBall => {
                                self.add_system_message("Adding a ball!", LIME);
                            }
                            ChatCommand::Clear => {
                                self.clear();
                            }
                            ChatCommand::Help => {
                                self.add_system_message(
                                    "Commands: /ball, /clear, /help, /count",
                                    YELLOW,
                                );
                            }
                            ChatCommand::Count => {
                                self.add_system_message(
                                    format!("There are {} balls in the scene.", ball_count),
                                    YELLOW,
                                );
                            }
                            ChatCommand::None => {}
                        }

                        result.command = command;
                        result.message_sent = true;
                        self.input_text.clear();
                    }
                }
            });

        result
    }

}

/// Implementing HasBounds gives us contains_point() for free via the default implementation.
///
/// The only thing we need to provide is the `bounds()` method - the trait handles
/// the rest! This is a great example of Rust's "composition over inheritance" approach.
///
/// Note: window_pos is automatically updated by macroquad when the user drags the window,
/// so our bounds will always reflect the current position!
impl HasBounds for ChatPanel {
    fn bounds(&self) -> Bounds {
        // If not visible, return zero-size bounds so contains_point returns false
        if !self.visible {
            return Bounds::default();
        }
        // Build bounds from the current window position (which macroquad updates when dragged)
        Bounds::new(
            self.window_pos.x,
            self.window_pos.y,
            Self::WIDTH + Self::MARGIN,
            Self::HEIGHT + Self::MARGIN,
        )
    }
}

impl Default for ChatPanel {
    fn default() -> Self {
        Self::new()
    }
}