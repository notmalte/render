use crate::{canvas::Canvas, rasterizer::point::Point, vector::Vector, viewport::Viewport};

pub mod interpolate;
pub mod line;
pub mod point;
pub mod triangle;

pub struct Rasterizer {
    viewport: Viewport,
}

impl Rasterizer {
    pub fn new(viewport: Viewport) -> Self {
        Self { viewport }
    }

    fn viewport_to_canvas(&self, canvas: &Canvas, x: f64, y: f64) -> Point {
        Point::new(
            (x * canvas.width() as f64 / self.viewport.width()) as i32,
            (y * canvas.height() as f64 / self.viewport.height()) as i32,
        )
    }

    pub fn project_vertex(&self, v: Vector, canvas: &Canvas) -> Point {
        self.viewport_to_canvas(
            canvas,
            v.x * self.viewport.distance() / v.z,
            v.y * self.viewport.distance() / v.z,
        )
    }
}
