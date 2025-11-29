//! Rapier Physics Demo with Chat UI
//!
//! A physics simulation using Rapier2D with an interactive chat interface.

use macroquad::prelude::*;
use rapier_physics::app::App;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rapier Physics Demo with Chat".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Create application
    let mut app = App::new();
    
    // Initialize UI skin
    app.init_ui();

    // Main game loop
    loop {
        // Update physics
        app.update_physics();

        // Render scene
        app.render();

        // Render UI and handle UI interactions
        app.render_ui();

        // Handle input
        app.handle_keyboard_input();
        app.handle_mouse_input();

        // Wait for next frame
        next_frame().await
    }
}