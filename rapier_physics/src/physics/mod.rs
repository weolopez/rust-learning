//! Physics engine module
//! 
//! Contains all Rapier2D physics-related functionality.

mod world;
mod bodies;

pub use world::PhysicsWorld;
pub use bodies::BallManager;