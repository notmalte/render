use crate::{
    canvas::Canvas,
    color::Color,
    raytracer::{scene::Scene, viewport::Viewport},
    vector::Vector,
};

pub mod object;
pub mod scene;
mod viewport;

pub trait Raytraceable {
    fn intersect_closest(
        &self,
        origin: Vector,
        direction: Vector,
        t_min: f64,
        t_max: f64,
    ) -> Option<f64>;

    fn color(&self) -> Color;
}

pub struct Raytracer {
    viewport: Viewport,
    scene: Scene,
}

impl Raytracer {
    pub fn new(scene: Scene) -> Self {
        Self {
            viewport: Viewport::default(),
            scene,
        }
    }

    fn canvas_to_viewport(&self, canvas: &Canvas, cx: i32, cy: i32) -> Vector {
        let vx = cx as f64 * (self.viewport.width() / canvas.width() as f64);
        let vy = cy as f64 * (self.viewport.height() / canvas.height() as f64);

        Vector::new(vx, vy, self.viewport.distance())
    }

    fn trace_ray(&self, origin: Vector, direction: Vector, t_min: f64, t_max: f64) -> Color {
        let mut closest_t = None;
        let mut closest_color = None;

        for object in self.scene.objects() {
            if let Some(t) = object.intersect_closest(origin, direction, t_min, t_max) {
                assert!(t >= t_min && t <= t_max);
                if closest_t.is_none() || t < closest_t.unwrap() {
                    closest_t = Some(t);
                    closest_color = Some(object.color());
                }
            }
        }

        closest_color.unwrap_or(Color::BLACK)
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let camera_position = Vector::new(0., 0., 0.);

        for x in canvas.x_range() {
            for y in canvas.y_range() {
                let d = self.canvas_to_viewport(canvas, x, y);
                let color = self.trace_ray(camera_position, d, 0., f64::INFINITY);

                canvas.set_pixel(x, y, color);
            }
        }
    }
}
