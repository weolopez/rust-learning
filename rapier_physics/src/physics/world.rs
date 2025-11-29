//! Physics world management
//! 
//! Contains the PhysicsWorld struct that encapsulates all Rapier2D physics state.

use rapier2d::prelude::*;

/// Encapsulates all physics simulation state
pub struct PhysicsWorld {
    pub gravity: Vector<Real>,
    pub integration_parameters: IntegrationParameters,
    pub physics_pipeline: PhysicsPipeline,
    pub island_manager: IslandManager,
    pub broad_phase: DefaultBroadPhase,
    pub narrow_phase: NarrowPhase,
    pub rigid_body_set: RigidBodySet,
    pub collider_set: ColliderSet,
    pub impulse_joint_set: ImpulseJointSet,
    pub multibody_joint_set: MultibodyJointSet,
    pub ccd_solver: CCDSolver,
}

impl PhysicsWorld {
    /// Create a new physics world with default settings
    pub fn new() -> Self {
        let mut collider_set = ColliderSet::new();
        
        // Create the ground (static box at bottom)
        let ground_collider = ColliderBuilder::cuboid(10.0, 0.5)
            .translation(vector![0.0, -5.0])
            .build();
        collider_set.insert(ground_collider);

        // Create walls
        let left_wall = ColliderBuilder::cuboid(0.5, 8.0)
            .translation(vector![-8.0, 0.0])
            .build();
        collider_set.insert(left_wall);

        let right_wall = ColliderBuilder::cuboid(0.5, 8.0)
            .translation(vector![8.0, 0.0])
            .build();
        collider_set.insert(right_wall);

        Self {
            gravity: vector![0.0, -9.81],
            integration_parameters: IntegrationParameters::default(),
            physics_pipeline: PhysicsPipeline::new(),
            island_manager: IslandManager::new(),
            broad_phase: DefaultBroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            rigid_body_set: RigidBodySet::new(),
            collider_set,
            impulse_joint_set: ImpulseJointSet::new(),
            multibody_joint_set: MultibodyJointSet::new(),
            ccd_solver: CCDSolver::new(),
        }
    }

    /// Step the physics simulation forward
    pub fn step(&mut self) {
        let physics_hooks = ();
        let event_handler = ();

        self.physics_pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.rigid_body_set,
            &mut self.collider_set,
            &mut self.impulse_joint_set,
            &mut self.multibody_joint_set,
            &mut self.ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );
    }
}

impl Default for PhysicsWorld {
    fn default() -> Self {
        Self::new()
    }
}