use std::ops::Mul;

use nalgebra::Vector2;

pub struct Particle {
    mass: f64,
    acceleration: Vector2<f64>,
    velocity: Vector2<f64>,
    position: Vector2<f64>,
}

impl Particle {
    pub fn new(
    ) -> Self {
        Particle {
            mass: 1.,
            acceleration: Vector2::new(0., 0.),
            velocity: Vector2::new(0., 0.),
            position: Vector2::new(0., 0.),
        }
    }

    pub fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration.x = 0.;
        self.acceleration.y = 0.;
    }

    pub fn apply_force(&mut self, force: Vector2<f64>) {
        self.acceleration += force / self.mass;
    }

    pub fn get_mass(&self) -> &f64 {
        &self.mass
    }

    pub fn get_acceleration(&self) -> &Vector2<f64> {
        &self.acceleration
    }

    pub fn get_velocity(&self) -> &Vector2<f64> {
        &self.velocity
    }

    pub fn get_position(&self) -> &Vector2<f64> {
        &self.position
    }

    pub fn get_mut_mass(&mut self) -> &mut f64 {
        &mut self.mass
    }

    pub fn get_mut_acceleration(&mut self) -> &mut Vector2<f64> {
        &mut self.acceleration
    }

    pub fn get_mut_velocity(&mut self) -> &mut Vector2<f64> {
        &mut self.velocity
    }

    pub fn get_mut_position(&mut self) -> &mut Vector2<f64> {
        &mut self.position
    }
}
