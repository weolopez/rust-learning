//! Application constants and utility functions

use macroquad::prelude::*;
use rapier2d::prelude::*;

/// Scale factor: Rapier uses meters, we scale to screen pixels
pub const SCALE: f32 = 50.0;

/// Ball colors available in the simulation
pub const BALL_COLORS: [Color; 7] = [RED, BLUE, GREEN, YELLOW, ORANGE, PURPLE, PINK];

/// Default ball radius in physics units
pub const BALL_RADIUS: f32 = 0.5;

/// Default ball restitution (bounciness)
pub const BALL_RESTITUTION: f32 = 0.7;

/// Convert world (physics) coordinates to screen coordinates
pub fn world_to_screen(pos: &Vector<Real>) -> Vec2 {
    vec2(
        pos.x * SCALE + screen_width() / 2.0,
        screen_height() / 2.0 - pos.y * SCALE, // Flip Y for screen coords
    )
}

/// Convert screen coordinates to world (physics) coordinates
pub fn screen_to_world(screen_x: f32, screen_y: f32) -> (f32, f32) {
    let world_x = (screen_x - screen_width() / 2.0) / SCALE;
    let world_y = (screen_height() / 2.0 - screen_y) / SCALE;
    (world_x, world_y)
}