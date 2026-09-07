use std::ops::{Add, AddAssign, Mul, Sub, SubAssign};

const DEFAULT_GRAVITY: f32 = 28.0;
const BODY_RADIUS: f32 = 0.8;
const RESTITUTION: f32 = 0.82;
const SETTLE_SPEED: f32 = 1.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    fn length_squared(self) -> f32 {
        self.dot(self)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
    pub width: f32,
    pub height: f32,
}

impl Bounds {
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
    pub restitution: f32,
    pub dynamic: bool,
}

impl Body {
    fn dynamic(position: Vec2, velocity: Vec2) -> Self {
        Self {
            position,
            velocity,
            radius: BODY_RADIUS,
            restitution: RESTITUTION,
            dynamic: true,
        }
    }
}

#[derive(Debug)]
pub struct PhysicsWorld {
    pub bodies: Vec<Body>,
    pub bounds: Bounds,
    pub gravity_enabled: bool,
    gravity: Vec2,
    spawn_count: usize,
}

impl PhysicsWorld {
    pub fn new(bounds: Bounds) -> Self {
        Self {
            bodies: Vec::new(),
            bounds,
            gravity_enabled: true,
            gravity: Vec2::new(0.0, DEFAULT_GRAVITY),
            spawn_count: 0,
        }
    }

    pub fn step(&mut self, dt: f32) {
        if dt <= 0.0 || !dt.is_finite() {
            return;
        }

        for body in &mut self.bodies {
            if !body.dynamic {
                continue;
            }
            if self.gravity_enabled {
                body.velocity += self.gravity * dt;
            }
            body.position += body.velocity * dt;
            solve_boundary_collision(body, self.bounds);
        }

        for first in 0..self.bodies.len() {
            for second in first + 1..self.bodies.len() {
                let (left, right) = self.bodies.split_at_mut(second);
                resolve_circle_collision(&mut left[first], &mut right[0]);
            }
        }

        for body in &mut self.bodies {
            solve_boundary_collision(body, self.bounds);
        }
    }

    pub fn resize(&mut self, bounds: Bounds) {
        self.bounds = Bounds::new(bounds.width.max(2.0), bounds.height.max(2.0));
        for body in &mut self.bodies {
            solve_boundary_collision(body, self.bounds);
        }
    }

    pub fn reset(&mut self) {
        self.bodies.clear();
        self.spawn_count = 0;
        self.gravity_enabled = true;
        self.bodies.push(Body::dynamic(
            Vec2::new(self.bounds.width * 0.25, BODY_RADIUS + 0.1),
            Vec2::new(4.0, 0.0),
        ));
        self.bodies.push(Body::dynamic(
            Vec2::new(self.bounds.width * 0.5, BODY_RADIUS + 0.1),
            Vec2::new(-2.0, 0.0),
        ));
        self.bodies.push(Body::dynamic(
            Vec2::new(self.bounds.width * 0.75, BODY_RADIUS + 0.1),
            Vec2::new(1.0, 0.0),
        ));
    }

    pub fn clear_dynamic(&mut self) {
        self.bodies.retain(|body| !body.dynamic);
    }

    pub fn spawn_ball(&mut self) {
        let radius = BODY_RADIUS;
        if self.bounds.width < radius * 2.0 || self.bounds.height < radius * 2.0 {
            return;
        }

        let lane = self.spawn_count % 5;
        let lane_offset = lane as f32 - 2.0;
        let position_x =
            (self.bounds.width * 0.5 + lane_offset * 2.5).clamp(radius, self.bounds.width - radius);
        let velocity_x = lane_offset * 2.4;
        self.bodies.push(Body::dynamic(
            Vec2::new(position_x, radius + 0.1),
            Vec2::new(velocity_x, 0.0),
        ));
        self.spawn_count += 1;
    }

    pub fn toggle_gravity(&mut self) {
        self.gravity_enabled = !self.gravity_enabled;
    }
}

fn solve_boundary_collision(body: &mut Body, bounds: Bounds) {
    let radius = body.radius;
    if body.position.x - radius < 0.0 {
        body.position.x = radius;
        if body.velocity.x < 0.0 {
            body.velocity.x = -body.velocity.x * body.restitution;
        }
    }
    if body.position.x + radius > bounds.width {
        body.position.x = bounds.width - radius;
        if body.velocity.x > 0.0 {
            body.velocity.x = -body.velocity.x * body.restitution;
        }
    }
    if body.position.y - radius < 0.0 {
        body.position.y = radius;
        if body.velocity.y < 0.0 {
            body.velocity.y = -body.velocity.y * body.restitution;
        }
    }
    if body.position.y + radius > bounds.height {
        body.position.y = bounds.height - radius;
        if body.velocity.y > 0.0 {
            body.velocity.y = if body.velocity.y < SETTLE_SPEED {
                0.0
            } else {
                -body.velocity.y * body.restitution
            };
        }
    }
}

fn resolve_circle_collision(first: &mut Body, second: &mut Body) {
    if !first.dynamic && !second.dynamic {
        return;
    }

    let delta = second.position - first.position;
    let minimum_distance = first.radius + second.radius;
    let distance_squared = delta.length_squared();
    if distance_squared >= minimum_distance * minimum_distance {
        return;
    }

    let (normal, distance) = if distance_squared > f32::EPSILON {
        let distance = distance_squared.sqrt();
        (delta * (1.0 / distance), distance)
    } else {
        (Vec2::new(1.0, 0.0), 0.0)
    };
    let penetration = minimum_distance - distance;
    let first_movable = first.dynamic;
    let second_movable = second.dynamic;
    let movable_count = first_movable as u8 + second_movable as u8;
    if first_movable {
        first.position -= normal * (penetration / f32::from(movable_count));
    }
    if second_movable {
        second.position += normal * (penetration / f32::from(movable_count));
    }

    let relative_velocity = second.velocity - first.velocity;
    let separating_speed = relative_velocity.dot(normal);
    if separating_speed >= 0.0 {
        return;
    }

    let restitution = first.restitution.min(second.restitution);
    let impulse = -(1.0 + restitution) * separating_speed / f32::from(movable_count);
    let impulse_vector = normal * impulse;
    if first_movable {
        first.velocity -= impulse_vector;
    }
    if second_movable {
        second.velocity += impulse_vector;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gravity_changes_velocity_and_position() {
        let mut world = PhysicsWorld::new(Bounds::new(40.0, 40.0));
        world
            .bodies
            .push(Body::dynamic(Vec2::new(20.0, 10.0), Vec2::new(0.0, 0.0)));

        world.step(0.5);

        assert!((world.bodies[0].velocity.y - 14.0).abs() < 0.001);
        assert!((world.bodies[0].position.y - 17.0).abs() < 0.001);
    }

    #[test]
    fn floor_collision_corrects_penetration_and_bounces() {
        let mut world = PhysicsWorld::new(Bounds::new(40.0, 20.0));
        world
            .bodies
            .push(Body::dynamic(Vec2::new(20.0, 19.5), Vec2::new(0.0, 8.0)));

        world.step(0.1);

        assert_eq!(world.bodies[0].position.y, 19.2);
        assert!(world.bodies[0].velocity.y < 0.0);
        assert!((world.bodies[0].velocity.y + 8.856).abs() < 0.001);
    }

    #[test]
    fn gravity_toggle_stops_acceleration() {
        let mut world = PhysicsWorld::new(Bounds::new(40.0, 40.0));
        world
            .bodies
            .push(Body::dynamic(Vec2::new(20.0, 10.0), Vec2::new(0.0, 0.0)));
        world.toggle_gravity();

        world.step(0.5);

        assert_eq!(world.bodies[0].velocity, Vec2::new(0.0, 0.0));
        assert_eq!(world.bodies[0].position, Vec2::new(20.0, 10.0));
    }

    #[test]
    fn overlapping_balls_are_separated_and_exchange_velocity() {
        let mut world = PhysicsWorld::new(Bounds::new(40.0, 40.0));
        world
            .bodies
            .push(Body::dynamic(Vec2::new(19.0, 20.0), Vec2::new(2.0, 0.0)));
        world
            .bodies
            .push(Body::dynamic(Vec2::new(20.0, 20.0), Vec2::new(-2.0, 0.0)));

        world.step(0.001);

        assert!(world.bodies[1].position.x - world.bodies[0].position.x >= 1.59);
        assert!(world.bodies[0].velocity.x < 0.0);
        assert!(world.bodies[1].velocity.x > 0.0);
    }

    #[test]
    fn reset_restores_three_balls_and_gravity() {
        let mut world = PhysicsWorld::new(Bounds::new(40.0, 40.0));
        world.spawn_ball();
        world.toggle_gravity();

        world.reset();

        assert_eq!(world.bodies.len(), 3);
        assert!(world.gravity_enabled);
    }
}
