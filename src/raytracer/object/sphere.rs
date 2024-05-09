use crate::{color::Color, raytracer::Raytraceable, vector::Vector};

pub struct Sphere {
    center: Vector,
    radius: f64,
    color: Color,
    specular: Option<f64>,
}

impl Sphere {
    pub fn new(center: Vector, radius: f64, color: Color, specular: Option<f64>) -> Self {
        Self {
            center,
            radius,
            color,
            specular,
        }
    }
}

impl Raytraceable for Sphere {
    fn intersect_closest(
        &self,
        origin: Vector,
        direction: Vector,
        t_min: f64,
        t_max: f64,
    ) -> Option<f64> {
        let r = self.radius;
        let co = origin - self.center;

        let a = direction.dot(direction);
        let b = 2. * co.dot(direction);
        let c = co.dot(co) - r * r;

        let discriminant = b * b - 4. * a * c;
        if discriminant < 0. {
            return None;
        }

        let t1 = (-b - discriminant.sqrt()) / (2. * a);
        let t2 = (-b + discriminant.sqrt()) / (2. * a);

        if t1 >= t_min && t1 <= t_max {
            Some(t1)
        } else if t2 >= t_min && t2 <= t_max {
            Some(t2)
        } else {
            None
        }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn normal(&self, point: Vector) -> Vector {
        (point - self.center).normalize()
    }

    fn specular(&self) -> Option<f64> {
        self.specular
    }
}
