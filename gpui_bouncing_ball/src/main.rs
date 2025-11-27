use macroquad::prelude::*;
use std::f32::consts::PI;

struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: Color,
}

struct Hexagon {
    center: Vec2,
    radius: f32,
    rotation: f32,
}

impl Hexagon {
    fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            rotation: 0.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.rotation += 0.5 * dt; // Rotate slowly
    }

    fn vertices(&self) -> Vec<Vec2> {
        (0..6)
            .map(|i| {
                let angle = self.rotation + (i as f32) * PI / 3.0;
                Vec2::new(
                    self.center.x + self.radius * angle.cos(),
                    self.center.y + self.radius * angle.sin(),
                )
            })
            .collect()
    }

    fn draw(&self) {
        let verts = self.vertices();
        for i in 0..6 {
            let next = (i + 1) % 6;
            draw_line(
                verts[i].x,
                verts[i].y,
                verts[next].x,
                verts[next].y,
                4.0,
                DARKGRAY,
            );
        }
    }

    fn collide_ball(&self, ball: &mut Ball) {
        let verts = self.vertices();
        
        for i in 0..6 {
            let next = (i + 1) % 6;
            let edge_start = verts[i];
            let edge_end = verts[next];
            
            // Edge vector and normal
            let edge = edge_end - edge_start;
            let normal = Vec2::new(-edge.y, edge.x).normalize();
            
            // Vector from edge start to ball center
            let to_ball = ball.pos - edge_start;
            
            // Project ball onto edge
            let edge_length_sq = edge.length_squared();
            let t = (to_ball.dot(edge) / edge_length_sq).clamp(0.0, 1.0);
            let closest = edge_start + edge * t;
            
            // Distance to edge
            let dist_vec = ball.pos - closest;
            let dist = dist_vec.length();
            
            // Collision check
            if dist < ball.radius {
                let collision_normal = if dist > 0.0001 {
                    dist_vec / dist
                } else {
                    normal
                };
                
                // Only resolve if ball is moving toward the wall
                let vel_toward_wall = ball.vel.dot(collision_normal);
                if vel_toward_wall < 0.0 {
                    // Push ball out to prevent tunneling
                    let penetration = ball.radius - dist;
                    ball.pos += collision_normal * penetration;
                    
                    // Reflect velocity with restitution
                    const RESTITUTION: f32 = 0.8;
                    ball.vel -= collision_normal * vel_toward_wall * (1.0 + RESTITUTION);
                }
            }
        }
    }
}

impl Ball {
    fn update(&mut self, dt: f32) {
        // Apply gravity
        const GRAVITY: f32 = 900.0;
        self.vel.y += GRAVITY * dt;

        // Apply friction (air resistance)
        const FRICTION: f32 = 0.98;
        self.vel *= FRICTION;

        self.pos += self.vel * dt;
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }

    fn collide_with(&mut self, other: &mut Ball) {
        let delta = other.pos - self.pos;
        let distance = delta.length();
        let min_distance = self.radius + other.radius;

        if distance < min_distance && distance > 0.0001 {
            // Collision detected
            let normal = delta / distance;
            
            // Calculate relative velocity
            let relative_vel = other.vel - self.vel;
            let vel_along_normal = relative_vel.dot(normal);

            // Don't resolve if velocities are separating (moving apart)
            if vel_along_normal >= 0.0 {
                return;
            }

            // Separate balls to prevent overlap
            let overlap = min_distance - distance;
            let separation = normal * (overlap * 0.5);
            self.pos -= separation;
            other.pos += separation;

            // Calculate impulse (assuming equal mass and elasticity)
            let restitution = 0.8; // Bounciness factor
            let impulse = -(1.0 + restitution) * vel_along_normal / 2.0;

            // Apply impulse
            let impulse_vec = normal * impulse;
            self.vel -= impulse_vec;
            other.vel += impulse_vec;
        }
    }
}

fn handle_ball_collisions(balls: &mut Vec<Ball>) {
    let len = balls.len();
    for i in 0..len {
        for j in (i + 1)..len {
            // Split mutable references safely
            let (left, right) = balls.split_at_mut(j);
            left[i].collide_with(&mut right[0]);
        }
    }
}

#[macroquad::main("Bouncing Ball")]
async fn main() {
    let mut balls: Vec<Ball> = vec![Ball {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        vel: vec2(150.0, 120.0),
        radius: 20.0,
        color: RED,
    }];

    let mut hexagon = Hexagon::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        200.0,
    );

    // UI state
    let mut ball_radius: f32 = 20.0;
    let mut ball_color_idx: usize = 0;
    let colors = vec![RED, BLUE, GREEN, YELLOW, ORANGE, PURPLE, PINK];

    loop {
        let dt = get_frame_time();

        // background
        clear_background(LIGHTGRAY);

        // update hexagon rotation
        hexagon.update(dt);
        hexagon.draw();

        // handle input for first ball (if exists)
        const MOVE_SPEED: f32 = 6000.0;
        if !balls.is_empty() {
            if is_key_down(KeyCode::Up) {
                balls[0].vel.y -= MOVE_SPEED * dt;
            }
            if is_key_down(KeyCode::Down) {
                balls[0].vel.y += MOVE_SPEED * dt;
            }
            if is_key_down(KeyCode::Left) {
                balls[0].vel.x -= MOVE_SPEED * dt;
            }
            if is_key_down(KeyCode::Right) {
                balls[0].vel.x += MOVE_SPEED * dt;
            }
        }

        // Run physics simulation multiple times per frame for stability
        const SUBSTEPS: i32 = 8;
        let sub_dt = dt / SUBSTEPS as f32;
        
        for _ in 0..SUBSTEPS {
            // update all balls
            for ball in balls.iter_mut() {
                ball.update(sub_dt);
            }

            // collide with hexagon
            for ball in balls.iter_mut() {
                hexagon.collide_ball(ball);
            }

            // Handle ball-to-ball collisions
            handle_ball_collisions(&mut balls);
        }

        // Draw all balls
        for ball in balls.iter() {
            ball.draw();
        }

        // UI Panel
        let panel_x = 10.0;
        let panel_y = 10.0;
        let panel_w = 250.0;
        let panel_h = 180.0;
        
        draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::new(0.2, 0.2, 0.2, 0.8));
        
        let mut y_offset = panel_y + 20.0;
        draw_text(&format!("FPS: {:.0}", get_fps()), panel_x + 10.0, y_offset, 20.0, WHITE);
        
        y_offset += 25.0;
        draw_text(&format!("Balls: {}", balls.len()), panel_x + 10.0, y_offset, 20.0, WHITE);
        
        y_offset += 25.0;
        draw_text(&format!("Radius: {:.0}", ball_radius), panel_x + 10.0, y_offset, 20.0, WHITE);
        
        y_offset += 25.0;
        draw_text("Color:", panel_x + 10.0, y_offset, 20.0, WHITE);
        draw_circle(panel_x + 80.0, y_offset - 5.0, 8.0, colors[ball_color_idx]);
        
        y_offset += 30.0;
        draw_text("SPACE: Add ball", panel_x + 10.0, y_offset, 18.0, LIGHTGRAY);
        
        y_offset += 20.0;
        draw_text("C: Change color", panel_x + 10.0, y_offset, 18.0, LIGHTGRAY);
        
        y_offset += 20.0;
        draw_text("+/-: Resize", panel_x + 10.0, y_offset, 18.0, LIGHTGRAY);

        // Handle UI input
        if is_key_pressed(KeyCode::Space) {
            let new_ball = Ball {
                pos: hexagon.center,
                vel: vec2(
                    rand::gen_range(-200.0, 200.0),
                    rand::gen_range(-200.0, 200.0),
                ),
                radius: ball_radius,
                color: colors[ball_color_idx],
            };
            balls.push(new_ball);
        }

        if is_key_pressed(KeyCode::C) {
            ball_color_idx = (ball_color_idx + 1) % colors.len();
        }

        if is_key_pressed(KeyCode::Equal) || is_key_pressed(KeyCode::KpAdd) {
            ball_radius = (ball_radius + 5.0).min(50.0);
        }

        if is_key_pressed(KeyCode::Minus) || is_key_pressed(KeyCode::KpSubtract) {
            ball_radius = (ball_radius - 5.0).max(5.0);
        }

        if is_key_pressed(KeyCode::R) {
            balls.clear();
            balls.push(Ball {
                pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
                vel: vec2(150.0, 120.0),
                radius: 20.0,
                color: RED,
            });
        }

        next_frame().await
    }
}
