//! Custom UI skin/styling
//!
//! Provides custom visual styling for the macroquad UI components.

use macroquad::prelude::*;
use macroquad::ui::{root_ui, Skin};

/// Create a custom skin for better looking UI
pub fn create_custom_skin() -> Skin {
    let label_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(WHITE)
        .build();

    let button_style = root_ui()
        .style_builder()
        .font_size(16)
        .text_color(WHITE)
        .color(Color::new(0.3, 0.3, 0.5, 1.0))
        .color_hovered(Color::new(0.4, 0.4, 0.6, 1.0))
        .color_clicked(Color::new(0.2, 0.2, 0.4, 1.0))
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .font_size(16)
        .color(WHITE)  // background when not focused
        .color_clicked(WHITE)  // background when focused
        .text_color(BLACK)
        .build();

    let window_style = root_ui()
        .style_builder()
        .color(Color::new(0.1, 0.1, 0.15, 0.95))
        .build();

    let group_style = root_ui()
        .style_builder()
        .color(Color::new(0.15, 0.15, 0.2, 0.9))
        .build();

    Skin {
        label_style,
        button_style,
        editbox_style,
        window_style,
        group_style,
        ..root_ui().default_skin()
    }
}
