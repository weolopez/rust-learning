//! Controls panel UI component
//!
//! Provides the main control interface for the physics simulation.

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use super::{Bounds, HasBounds};

/// Result of control panel interactions
pub struct ControlsResult {
    pub add_ball_clicked: bool,
    pub toggle_chat_clicked: bool,
}

/// Controls panel for the physics simulation
pub struct ControlsPanel {
    /// The current position of the window (macroquad mutates this when dragged!)
    pub window_pos: Vec2,
}

impl ControlsPanel {
    /// Panel position and size constants
    pub const X: f32 = 10.0;
    pub const Y: f32 = 10.0;
    pub const WIDTH: f32 = 200.0;
    pub const HEIGHT: f32 = 180.0;
    /// Extra margin for window borders
    pub const MARGIN: f32 = 10.0;

    /// Create a new controls panel with default position
    pub fn new() -> Self {
        Self {
            // Initial position - macroquad will update this when the window is dragged
            window_pos: vec2(Self::X, Self::Y),
        }
    }

    /// Render the controls panel
    /// Returns actions triggered by button clicks
    ///
    /// Note: The window_pos field is automatically updated by macroquad when dragged!
    pub fn render(&mut self, ball_count: usize, chat_visible: bool) -> ControlsResult {
        let mut result = ControlsResult {
            add_ball_clicked: false,
            toggle_chat_clicked: false,
        };

        // DEBUG: Log position before render
        println!("DEBUG: ControlsPanel before render: window_pos=({}, {})",
                 self.window_pos.x, self.window_pos.y);

        // IMPORTANT: Pass window_pos directly - macroquad will mutate it when dragged!
        widgets::Window::new(hash!(), self.window_pos, vec2(Self::WIDTH, Self::HEIGHT))
            .label("Controls")
            .movable(true)
            .ui(&mut root_ui(), |ui| {
                ui.label(None, &format!("FPS: {:.0}", get_fps()));
                ui.label(None, &format!("Bodies: {}", ball_count));
                ui.label(None, &format!("Screen: {:.0}x{:.0}", screen_width(), screen_height()));
                ui.separator();

                if ui.button(None, "Add Ball") {
                    result.add_ball_clicked = true;
                }

                let toggle_label = if chat_visible {
                    "Hide Chat (T)"
                } else {
                    "Show Chat (T)"
                };
                if ui.button(None, toggle_label) {
                    result.toggle_chat_clicked = true;
                }

                ui.separator();
                ui.label(None, "SPACE: Add ball");
                ui.label(None, "Click: Add at cursor");
                ui.label(None, "R: Reset");
            });

        result
    }
}

impl Default for ControlsPanel {
    fn default() -> Self {
        Self::new()
    }
}

/// Implementing HasBounds gives us contains_point() for free via the default implementation.
///
/// Note: window_pos is automatically updated by macroquad when the user drags the window,
/// so our bounds will always reflect the current position!
impl HasBounds for ControlsPanel {
    fn bounds(&self) -> Bounds {
        // Build bounds from the current window position (which macroquad updates when dragged)
        let bounds = Bounds::new(
            self.window_pos.x,
            self.window_pos.y,
            Self::WIDTH + Self::MARGIN,
            Self::HEIGHT + Self::MARGIN,
        );
        // DEBUG: Log the bounds being returned
        println!(
            "DEBUG ControlsPanel::bounds() -> x={}, y={}, width={}, height={}",
            bounds.x, bounds.y, bounds.width, bounds.height
        );
        bounds
    }
}
