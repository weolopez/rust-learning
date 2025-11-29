//! Ball body management
//! 
//! Handles creation, removal, and management of ball rigid bodies.

use rapier2d::prelude::*;
use crate::constants::{BALL_RADIUS, BALL_RESTITUTION};

/// Manages ball rigid bodies in the physics simulation
pub struct BallManager {
    pub handles: Vec<RigidBodyHandle>,
}

impl BallManager {
    /// Create a new ball manager
    pub fn new() -> Self {
        Self {
            handles: Vec::new(),
        }
    }

    /// Create initial balls in the simulation
    pub fn create_initial_balls(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) {
        for i in 0..3 {
            self.add_ball_at(
                rigid_body_set,
                collider_set,
                -2.0 + i as f32 * 2.0,
                5.0 + i as f32,
                0.0,
                0.0,
            );
        }
    }

    /// Add a ball at specific world coordinates with velocity
    pub fn add_ball_at(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        x: f32,
        y: f32,
        vel_x: f32,
        vel_y: f32,
    ) -> RigidBodyHandle {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .linvel(vector![vel_x, vel_y])
            .build();
        let collider = ColliderBuilder::ball(BALL_RADIUS)
            .restitution(BALL_RESTITUTION)
            .build();
        let handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, handle, rigid_body_set);
        self.handles.push(handle);
        handle
    }

    /// Add a ball at random position with random velocity
    pub fn add_random_ball(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
    ) -> RigidBodyHandle {
        self.add_ball_at(
            rigid_body_set,
            collider_set,
            macroquad::rand::gen_range(-5.0, 5.0),
            macroquad::rand::gen_range(3.0, 6.0),
            macroquad::rand::gen_range(-3.0, 3.0),
            macroquad::rand::gen_range(-1.0, 2.0),
        )
    }

    /// Remove all balls and reset with initial configuration
    pub fn reset(
        &mut self,
        rigid_body_set: &mut RigidBodySet,
        collider_set: &mut ColliderSet,
        island_manager: &mut IslandManager,
        impulse_joint_set: &mut ImpulseJointSet,
        multibody_joint_set: &mut MultibodyJointSet,
    ) {
        // Remove all existing balls
        for handle in self.handles.drain(..) {
            rigid_body_set.remove(
                handle,
                island_manager,
                collider_set,
                impulse_joint_set,
                multibody_joint_set,
                true,
            );
        }

        // Create new initial balls
        self.create_initial_balls(rigid_body_set, collider_set);
    }

    /// Get the number of balls
    pub fn count(&self) -> usize {
        self.handles.len()
    }
}

impl Default for BallManager {
    fn default() -> Self {
        Self::new()
    }
}