//! Chat panel UI component
//! 
//! Provides a chat interface with message display and command handling.

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

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
}

impl ChatPanel {
    pub fn new() -> Self {
        Self {
            messages: vec![ChatMessage::system(
                "Welcome to Physics Chat! Type /help for commands.",
                GRAY,
            )],
            input_text: String::new(),
            username: String::from("User"),
            visible: true,
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

        let chat_width = 320.0;
        let chat_height = 400.0;
        let chat_x = screen_width() - chat_width - 10.0;

        widgets::Window::new(hash!(), vec2(chat_x, 10.0), vec2(chat_width, chat_height))
            .label("Chat")
            .ui(&mut root_ui(), |ui| {
                // Username input
                ui.label(None, "Your name:");
                ui.input_text(hash!(), "", &mut self.username);
                ui.separator();

                // Chat messages area
                widgets::Group::new(hash!(), vec2(chat_width - 20.0, chat_height - 150.0))
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

    /// Check if a screen position is within the chat panel area
    pub fn contains_point(&self, x: f32, y: f32) -> bool {
        if !self.visible {
            return false;
        }
        x > screen_width() - 340.0 && y < 420.0
    }
}

impl Default for ChatPanel {
    fn default() -> Self {
        Self::new()
    }
}