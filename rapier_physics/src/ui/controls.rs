//! Controls panel UI component
//! 
//! Provides the main control interface for the physics simulation.

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

/// Result of control panel interactions
pub struct ControlsResult {
    pub add_ball_clicked: bool,
    pub toggle_chat_clicked: bool,
}

/// Controls panel for the physics simulation
pub struct ControlsPanel;

impl ControlsPanel {
    /// Render the controls panel
    /// Returns actions triggered by button clicks
    pub fn render(ball_count: usize, chat_visible: bool) -> ControlsResult {
        let mut result = ControlsResult {
            add_ball_clicked: false,
            toggle_chat_clicked: false,
        };

        widgets::Window::new(hash!(), vec2(10.0, 10.0), vec2(200.0, 180.0))
            .label("Controls")
            .ui(&mut root_ui(), |ui| {
                ui.label(None, &format!("FPS: {:.0}", get_fps()));
                ui.label(None, &format!("Bodies: {}", ball_count));
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

    /// Check if a screen position is within the controls panel area
    pub fn contains_point(x: f32, y: f32) -> bool {
        x < 220.0 && y < 200.0
    }
}