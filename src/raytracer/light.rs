use crate::vector::Vector;

pub enum Light {
    Ambient { intensity: f64 },
    Point { intensity: f64, position: Vector },
    Directional { intensity: f64, direction: Vector },
}
