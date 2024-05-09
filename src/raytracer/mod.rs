use crate::{
    canvas::Canvas,
    color::Color,
    raytracer::{light::Light, scene::Scene, viewport::Viewport},
    vector::Vector,
};

pub mod light;
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

    fn normal(&self, point: Vector) -> Vector;

    fn specular(&self) -> Option<f64> {
        None
    }
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

    fn compute_lighting(
        &self,
        point: Vector,
        normal: Vector,
        view: Vector,
        specular: Option<f64>,
    ) -> f64 {
        let mut i = 0.;

        let diffuse_and_specular = |i: f64, l: Vector, n: Vector| -> f64 {
            let mut sum = 0.;

            let n_dot_l = n.dot(l);

            if n_dot_l > 0. {
                sum += i * n_dot_l / (n.length() * l.length());
            }

            if let Some(specular) = specular {
                let r = (2. * n * n_dot_l) - l;
                let r_dot_v = r.dot(view);

                if r_dot_v > 0. {
                    sum += i * (r_dot_v / (r.length() * view.length())).powf(specular);
                }
            }

            sum
        };

        for light in self.scene.lights() {
            match *light {
                Light::Ambient { intensity } => {
                    i += intensity;
                }
                Light::Point {
                    intensity,
                    position,
                } => {
                    let l = position - point;

                    i += diffuse_and_specular(intensity, l, normal);
                }
                Light::Directional {
                    intensity,
                    direction,
                } => {
                    i += diffuse_and_specular(intensity, direction, normal);
                }
            }
        }

        i
    }

    fn trace_ray(&self, origin: Vector, direction: Vector, t_min: f64, t_max: f64) -> Color {
        let mut closest_t = None;
        let mut closest_object = None;

        for object in self.scene.objects() {
            if let Some(t) = object.intersect_closest(origin, direction, t_min, t_max) {
                assert!(t >= t_min && t <= t_max);
                if closest_t.is_none() || t < closest_t.unwrap() {
                    closest_t = Some(t);
                    closest_object = Some(object);
                }
            }
        }

        closest_object.map_or(self.scene.base_color(), |object| {
            let point = origin + closest_t.unwrap() * direction;
            let normal = object.normal(point);

            object.color() * self.compute_lighting(point, normal, -direction, object.specular())
        })
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
