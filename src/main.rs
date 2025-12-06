use macroquad::prelude::*;

// === Constants ===
const SIM_MIN_WIDTH: f32 = 20.0;
const BOUNDARY_PADDING: f32 = 1.0;
const TIME_STEP: f32 = 1.0 / 60.0;
const VELOCITY_THRESHOLD: f32 = 0.1;

// === Coordinate Conversion ===
fn pixels_per_meter() -> f32 {
    screen_width().min(screen_height()) / SIM_MIN_WIDTH
}

fn world_dimensions() -> Vec2 {
    let ppm = pixels_per_meter();
    Vec2::new(screen_width() / ppm, screen_height() / ppm)
}

#[allow(dead_code)]
fn screen_to_world(screen_pos: Vec2) -> Vec2 {
    let ppm = pixels_per_meter();
    Vec2::new(screen_pos.x / ppm, (screen_height() - screen_pos.y) / ppm)
}

fn world_to_screen(world_pos: Vec2) -> Vec2 {
    let ppm = pixels_per_meter();
    Vec2::new(world_pos.x * ppm, screen_height() - world_pos.y * ppm)
}

// === Physics ===
struct Physics {
    gravity: f32,
    restitution: f32,
    friction: f32,
}

impl Default for Physics {
    fn default() -> Self {
        Self {
            gravity: -9.8,
            restitution: 0.7,
            friction: 0.99,
        }
    }
}

// === Boundary ===
struct Boundary {
    left: f32,
    right: f32,
    bottom: f32,
    top: f32,
}

impl Boundary {
    fn new() -> Self {
        let world = world_dimensions();
        Self {
            left: BOUNDARY_PADDING,
            right: world.x - BOUNDARY_PADDING,
            bottom: BOUNDARY_PADDING,
            top: world.y - BOUNDARY_PADDING,
        }
    }

    fn draw(&self) {
        let corners = [
            world_to_screen(Vec2::new(self.left, self.top)),
            world_to_screen(Vec2::new(self.right, self.top)),
            world_to_screen(Vec2::new(self.right, self.bottom)),
            world_to_screen(Vec2::new(self.left, self.bottom)),
        ];

        for i in 0..4 {
            let next = (i + 1) % 4;
            draw_line(corners[i].x, corners[i].y, corners[next].x, corners[next].y, 2.0, WHITE);
        }
    }
}

// === Particle ===
struct Particle {
    position: Vec2,
    velocity: Vec2,
    radius: f32,
    mass: f32,
    color: Color,
}

impl Particle {
    fn new(position: Vec2, velocity: Vec2, radius: f32, mass: f32, color: Color) -> Self {
        Self { position, velocity, radius, mass, color }
    }

    fn update(&mut self, physics: &Physics, dt: f32) {
        self.velocity.y += physics.gravity * dt;
        self.position += self.velocity * dt;
    }

    fn handle_boundary_collision(&mut self, physics: &Physics) {
        let bounds = Boundary::new();

        let min_x = bounds.left + self.radius;
        let max_x = bounds.right - self.radius;
        let min_y = bounds.bottom + self.radius;
        let max_y = bounds.top - self.radius;

        // Vertical boundaries
        if self.position.y <= min_y {
            self.position.y = min_y;
            self.velocity.x *= physics.friction;
            if self.velocity.y < 0.0 {
                self.velocity.y = -self.velocity.y * physics.restitution;
                if self.velocity.y.abs() < VELOCITY_THRESHOLD {
                    self.velocity.y = 0.0;
                }
            }
        } else if self.position.y >= max_y {
            self.position.y = max_y;
            if self.velocity.y > 0.0 {
                self.velocity.y = -self.velocity.y * physics.restitution;
            }
        }

        // Horizontal boundaries
        if self.position.x <= min_x {
            self.position.x = min_x;
            if self.velocity.x < 0.0 {
                self.velocity.x = -self.velocity.x * physics.restitution;
            }
        } else if self.position.x >= max_x {
            self.position.x = max_x;
            if self.velocity.x > 0.0 {
                self.velocity.x = -self.velocity.x * physics.restitution;
            }
        }
    }

    fn draw(&self) {
        let screen_pos = world_to_screen(self.position);
        let screen_radius = self.radius * pixels_per_meter();
        draw_circle(screen_pos.x, screen_pos.y, screen_radius, self.color);
    }
}

fn resolve_particle_collision(p1: &mut Particle, p2: &mut Particle, physics: &Physics) {
    let delta = p2.position - p1.position;
    let distance = delta.length();
    let min_dist = p1.radius + p2.radius;

    if distance >= min_dist || distance == 0.0 {
        return;
    }

    let normal = delta / distance;
    let overlap = min_dist - distance;
    let total_mass = p1.mass + p2.mass;

    // Separate particles
    p1.position -= normal * overlap * (p2.mass / total_mass);
    p2.position += normal * overlap * (p1.mass / total_mass);

    // Calculate impulse
    let rel_vel = p2.velocity - p1.velocity;
    let vel_along_normal = rel_vel.dot(normal);

    if vel_along_normal > 0.0 {
        return; // Already separating
    }

    let impulse = -(1.0 + physics.restitution) * vel_along_normal / (1.0 / p1.mass + 1.0 / p2.mass);
    let impulse_vec = impulse * normal;

    p1.velocity -= impulse_vec / p1.mass;
    p2.velocity += impulse_vec / p2.mass;
}

// === Main ===
#[macroquad::main("Falling Particle Simulation")]
async fn main() {
    let physics = Physics::default();

    let mut particles = vec![
        Particle::new(Vec2::new(8.0, 0.0), Vec2::new(1.0, 40.0), 0.8, 10.0, WHITE),
        Particle::new(Vec2::new(8.0, 9.0), Vec2::new(0.0, 0.0), 0.4, 2.0, WHITE),
    ];

    let mut accumulator = 0.0;

    loop {
        clear_background(BLACK);

        accumulator += get_frame_time();

        while accumulator >= TIME_STEP {
            // Update particles
            for p in &mut particles {
                p.update(&physics, TIME_STEP);
            }

            // Particle-particle collisions
            let (left, right) = particles.split_at_mut(1);
            resolve_particle_collision(&mut left[0], &mut right[0], &physics);

            // Boundary collisions
            for p in &mut particles {
                p.handle_boundary_collision(&physics);
            }

            accumulator -= TIME_STEP;
        }

        // Draw
        Boundary::new().draw();
        for p in &particles {
            p.draw();
        }

        next_frame().await;
    }
}
