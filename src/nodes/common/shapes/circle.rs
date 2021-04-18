use nalgebra::Vector2;

pub struct Circle {
    position: Vector2<f64>,
    radius: f64
}

impl Circle {
    pub fn new(position: Vector2<f64>, radius: f64) -> Circle {
        Circle {
            position,
            radius
        }
    }

    pub fn set_position(&mut self, position: Vector2<f64>) {
        self.position = position;
    }

    pub fn set_radius(&mut self, radius: f64) {
        self.radius = radius;
    }

    pub fn get_position(&self) -> &Vector2<f64> {
        &self.position
    }

    pub fn get_radius(&self) -> &f64 {
        &self.radius
    }
}