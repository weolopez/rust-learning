use macroquad::prelude::*;
use rapier2d::prelude::*;

// Scale factor: Rapier uses meters, we'll scale to screen pixels
const SCALE: f32 = 50.0;

fn world_to_screen(pos: &Vector<Real>) -> Vec2 {
    vec2(
        pos.x * SCALE + screen_width() / 2.0,
        screen_height() / 2.0 - pos.y * SCALE, // Flip Y for screen coords
    )
}

#[macroquad::main("Rapier Physics Demo")]
async fn main() {
    // Create physics world
    let mut rigid_body_set = RigidBodySet::new();
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

    // Create initial bouncing balls
    let mut ball_handles: Vec<RigidBodyHandle> = Vec::new();
    
    for i in 0..3 {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![-2.0 + i as f32 * 2.0, 5.0 + i as f32])
            .build();
        let collider = ColliderBuilder::ball(0.5)
            .restitution(0.7)
            .build();
        let ball_handle = rigid_body_set.insert(rigid_body);
        collider_set.insert_with_parent(collider, ball_handle, &mut rigid_body_set);
        ball_handles.push(ball_handle);
    }

    // Physics pipeline setup
    let gravity = vector![0.0, -9.81];
    let integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let physics_hooks = ();
    let event_handler = ();

    // Ball colors
    let colors = [RED, BLUE, GREEN, YELLOW, ORANGE, PURPLE, PINK];
    let mut color_idx = 0;

    loop {
        // Step physics
        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            None,
            &physics_hooks,
            &event_handler,
        );

        // Clear background
        clear_background(LIGHTGRAY);

        // Draw ground
        let ground_pos = world_to_screen(&vector![0.0, -5.0]);
        draw_rectangle(
            ground_pos.x - 10.0 * SCALE,
            ground_pos.y - 0.5 * SCALE,
            20.0 * SCALE,
            1.0 * SCALE,
            DARKGRAY,
        );

        // Draw walls
        let left_wall_pos = world_to_screen(&vector![-8.0, 0.0]);
        draw_rectangle(
            left_wall_pos.x - 0.5 * SCALE,
            left_wall_pos.y - 8.0 * SCALE,
            1.0 * SCALE,
            16.0 * SCALE,
            DARKGRAY,
        );

        let right_wall_pos = world_to_screen(&vector![8.0, 0.0]);
        draw_rectangle(
            right_wall_pos.x - 0.5 * SCALE,
            right_wall_pos.y - 8.0 * SCALE,
            1.0 * SCALE,
            16.0 * SCALE,
            DARKGRAY,
        );

        // Draw all balls
        for (i, &handle) in ball_handles.iter().enumerate() {
            if let Some(ball_body) = rigid_body_set.get(handle) {
                let pos = world_to_screen(ball_body.translation());
                let color = colors[i % colors.len()];
                draw_circle(pos.x, pos.y, 0.5 * SCALE, color);
            }
        }

        // UI Panel
        draw_rectangle(10.0, 10.0, 280.0, 140.0, Color::new(0.2, 0.2, 0.2, 0.8));
        draw_text(&format!("FPS: {:.0}", get_fps()), 20.0, 35.0, 20.0, WHITE);
        draw_text(&format!("Bodies: {}", ball_handles.len()), 20.0, 60.0, 20.0, WHITE);
        draw_text("SPACE: Add ball", 20.0, 85.0, 18.0, LIGHTGRAY);
        draw_text("Click: Add ball at cursor", 20.0, 105.0, 18.0, LIGHTGRAY);
        draw_text("R: Reset", 20.0, 125.0, 18.0, LIGHTGRAY);
        draw_text("C: Change color", 20.0, 145.0, 18.0, LIGHTGRAY);

        // Handle input: SPACE to add ball at top
        if is_key_pressed(KeyCode::Space) {
            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![
                    rand::gen_range(-5.0, 5.0),
                    rand::gen_range(3.0, 6.0)
                ])
                .linvel(vector![
                    rand::gen_range(-3.0, 3.0),
                    rand::gen_range(-1.0, 2.0)
                ])
                .build();
            let collider = ColliderBuilder::ball(0.5)
                .restitution(0.7)
                .build();
            let handle = rigid_body_set.insert(rigid_body);
            collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);
            ball_handles.push(handle);
        }

        // Handle input: Click to add ball at cursor
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();
            // Convert screen to world coordinates
            let world_x = (mx - screen_width() / 2.0) / SCALE;
            let world_y = (screen_height() / 2.0 - my) / SCALE;

            let rigid_body = RigidBodyBuilder::dynamic()
                .translation(vector![world_x, world_y])
                .build();
            let collider = ColliderBuilder::ball(0.5)
                .restitution(0.7)
                .build();
            let handle = rigid_body_set.insert(rigid_body);
            collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);
            ball_handles.push(handle);
        }

        // Handle input: C to change color
        if is_key_pressed(KeyCode::C) {
            color_idx = (color_idx + 1) % colors.len();
        }

        // Handle input: R to reset
        if is_key_pressed(KeyCode::R) {
            // Remove all ball bodies
            for handle in ball_handles.drain(..) {
                rigid_body_set.remove(
                    handle,
                    &mut island_manager,
                    &mut collider_set,
                    &mut impulse_joint_set,
                    &mut multibody_joint_set,
                    true,
                );
            }

            // Add 3 new balls
            for i in 0..3 {
                let rigid_body = RigidBodyBuilder::dynamic()
                    .translation(vector![-2.0 + i as f32 * 2.0, 5.0 + i as f32])
                    .build();
                let collider = ColliderBuilder::ball(0.5)
                    .restitution(0.7)
                    .build();
                let handle = rigid_body_set.insert(rigid_body);
                collider_set.insert_with_parent(collider, handle, &mut rigid_body_set);
                ball_handles.push(handle);
            }
        }

        next_frame().await
    }
}
