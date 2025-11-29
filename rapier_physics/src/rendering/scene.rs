//! Scene rendering
//! 
//! Handles rendering of the physics scene including background, walls, and balls.

use macroquad::prelude::*;
use rapier2d::prelude::*;

use crate::constants::{world_to_screen, SCALE, BALL_COLORS, BALL_RADIUS};

/// Background color for the scene
pub const BACKGROUND_COLOR: Color = Color::new(0.2, 0.2, 0.25, 1.0);

/// Scene renderer for drawing physics objects
pub struct SceneRenderer;

impl SceneRenderer {
    /// Clear the background
    pub fn clear_background() {
        clear_background(BACKGROUND_COLOR);
    }

    /// Draw the ground platform
    pub fn draw_ground() {
        let ground_pos = world_to_screen(&vector![0.0, -5.0]);
        draw_rectangle(
            ground_pos.x - 10.0 * SCALE,
            ground_pos.y - 0.5 * SCALE,
            20.0 * SCALE,
            1.0 * SCALE,
            DARKGRAY,
        );
    }

    /// Draw the side walls
    pub fn draw_walls() {
        // Left wall
        let left_wall_pos = world_to_screen(&vector![-8.0, 0.0]);
        draw_rectangle(
            left_wall_pos.x - 0.5 * SCALE,
            left_wall_pos.y - 8.0 * SCALE,
            1.0 * SCALE,
            16.0 * SCALE,
            DARKGRAY,
        );

        // Right wall
        let right_wall_pos = world_to_screen(&vector![8.0, 0.0]);
        draw_rectangle(
            right_wall_pos.x - 0.5 * SCALE,
            right_wall_pos.y - 8.0 * SCALE,
            1.0 * SCALE,
            16.0 * SCALE,
            DARKGRAY,
        );
    }

    /// Draw all balls in the simulation
    pub fn draw_balls(
        ball_handles: &[RigidBodyHandle],
        rigid_body_set: &RigidBodySet,
    ) {
        for (i, &handle) in ball_handles.iter().enumerate() {
            if let Some(ball_body) = rigid_body_set.get(handle) {
                let pos = world_to_screen(ball_body.translation());
                let color = BALL_COLORS[i % BALL_COLORS.len()];
                
                // Draw main ball
                draw_circle(pos.x, pos.y, BALL_RADIUS * SCALE, color);
                
                // Add a highlight for 3D effect
                draw_circle(pos.x - 5.0, pos.y - 5.0, 0.15 * SCALE, WHITE);
            }
        }
    }

    /// Draw the entire scene
    pub fn draw_scene(
        ball_handles: &[RigidBodyHandle],
        rigid_body_set: &RigidBodySet,
    ) {
        Self::clear_background();
        Self::draw_ground();
        Self::draw_walls();
        Self::draw_balls(ball_handles, rigid_body_set);
    }
}