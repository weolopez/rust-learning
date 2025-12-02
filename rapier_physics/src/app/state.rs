//! Application state
//! 
//! Contains the main App struct that holds all application state.

use macroquad::prelude::*;
use macroquad::ui::root_ui;

use crate::constants::screen_to_world;
use crate::physics::{PhysicsWorld, BallManager};
use crate::rendering::SceneRenderer;
use crate::ui::{ChatPanel, ChatCommand, ControlsPanel, HasBounds, create_custom_skin};

/// Main application state
pub struct App {
    /// Physics simulation
    pub physics: PhysicsWorld,
    /// Ball entity manager
    pub balls: BallManager,
    /// Chat UI panel
    pub chat: ChatPanel,
    /// Controls UI panel
    pub controls: ControlsPanel,
}

impl App {
    /// Create a new application instance
    pub fn new() -> Self {
        let mut physics = PhysicsWorld::new();
        let mut balls = BallManager::new();
        
        // Create initial balls
        balls.create_initial_balls(
            &mut physics.rigid_body_set,
            &mut physics.collider_set,
        );

        Self {
            physics,
            balls,
            chat: ChatPanel::new(),
            controls: ControlsPanel::new(),
        }
    }

    /// Initialize the UI skin
    pub fn init_ui(&self) {
        let skin = create_custom_skin();
        root_ui().push_skin(&skin);
    }

    /// Step the physics simulation
    pub fn update_physics(&mut self) {
        self.physics.step();
    }

    /// Render the scene
    pub fn render(&self) {
        SceneRenderer::draw_scene(
            &self.balls.handles,
            &self.physics.rigid_body_set,
        );
    }

    /// Render UI and handle UI interactions
    pub fn render_ui(&mut self) {
        // Render controls panel (note: render needs &mut self to track window position)
        let controls_result = self.controls.render(
            self.balls.count(),
            self.chat.visible,
        );

        if controls_result.add_ball_clicked {
            self.add_random_ball();
        }

        if controls_result.toggle_chat_clicked {
            self.chat.toggle();
        }

        // Render chat panel
        let chat_result = self.chat.render(self.balls.count());
        
        if chat_result.command == ChatCommand::AddBall {
            self.add_random_ball();
        }
    }

    /// Handle keyboard input
    pub fn handle_keyboard_input(&mut self) {
        // SPACE: Add random ball
        if is_key_pressed(KeyCode::Space) {
            self.add_random_ball();
        }

        // T: Toggle chat
        if is_key_pressed(KeyCode::T) {
            self.chat.toggle();
        }

        // R: Reset simulation
        if is_key_pressed(KeyCode::R) {
            self.reset();
        }
    }

    /// Handle mouse input
    pub fn handle_mouse_input(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();

            // DEBUG: Log mouse position
            println!("DEBUG: Mouse clicked at ({}, {})", mx, my);

            // Check if click is outside UI areas
            let in_controls = self.controls.contains_point(mx, my);
            let in_chat = self.chat.contains_point(mx, my);

            // DEBUG: Log results
            println!("DEBUG: in_controls={}, in_chat={}", in_controls, in_chat);

            if !in_controls && !in_chat {
                // Convert screen to world coordinates and add ball
                let (world_x, world_y) = screen_to_world(mx, my);
                self.balls.add_ball_at(
                    &mut self.physics.rigid_body_set,
                    &mut self.physics.collider_set,
                    world_x,
                    world_y,
                    0.0,
                    0.0,
                );
            }
        }
    }

    /// Add a random ball to the simulation
    fn add_random_ball(&mut self) {
        self.balls.add_random_ball(
            &mut self.physics.rigid_body_set,
            &mut self.physics.collider_set,
        );
    }

    /// Reset the simulation
    fn reset(&mut self) {
        self.balls.reset(
            &mut self.physics.rigid_body_set,
            &mut self.physics.collider_set,
            &mut self.physics.island_manager,
            &mut self.physics.impulse_joint_set,
            &mut self.physics.multibody_joint_set,
        );

        self.chat.add_system_message("Physics world reset!", ORANGE);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
