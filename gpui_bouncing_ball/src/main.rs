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
                // Push ball out
                let penetration = ball.radius - dist;
                ball.pos += dist_vec.normalize() * penetration;
                
                // Reflect velocity
                let vel_along_normal = ball.vel.dot(normal);
                if vel_along_normal < 0.0 {
                    ball.vel -= normal * vel_along_normal * 1.8; // Bounce with slight damping
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

        self.pos += self.vel * dt;
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }
}

#[macroquad::main("Bouncing Ball")]
async fn main() {
    let mut ball = Ball {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        vel: vec2(150.0, 120.0),
        radius: 20.0,
        color: RED,
    };

    let mut hexagon = Hexagon::new(
        vec2(screen_width() / 2.0, screen_height() / 2.0),
        200.0,
    );

    loop {
        let dt = get_frame_time();

        // background
        clear_background(LIGHTGRAY);

        // update hexagon rotation
        hexagon.update(dt);
        hexagon.draw();

        // handle input
        const MOVE_SPEED: f32 = 6000.0;
        if is_key_down(KeyCode::Up) {
            ball.vel.y -= MOVE_SPEED * dt;
        }
        if is_key_down(KeyCode::Down) {
            ball.vel.y += MOVE_SPEED * dt;
        }
        if is_key_down(KeyCode::Left) {
            ball.vel.x -= MOVE_SPEED * dt;
        }
        if is_key_down(KeyCode::Right) {
            ball.vel.x += MOVE_SPEED * dt;
        }

        // update & draw ball
        ball.update(dt);
        hexagon.collide_ball(&mut ball);
        ball.draw();

        // FPS and instructions
        draw_text(&format!("FPS: {:.0}", get_fps()), 10.0, 20.0, 20.0, BLACK);
        draw_text("Press R to reset", 10.0, 40.0, 20.0, BLACK);
        draw_text("Arrow keys to move", 10.0, 60.0, 20.0, BLACK);

        if is_key_pressed(KeyCode::R) {
            ball.pos = vec2(screen_width() / 2.0, screen_height() / 2.0);
            ball.vel = vec2(150.0, 120.0);
        }

        next_frame().await
    }
}
