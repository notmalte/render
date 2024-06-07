use crate::{
    canvas::Canvas,
    color::Color,
    raytracer::{light::Light, scene::Scene},
    vector::Vector,
    viewport::Viewport,
};

pub mod light;
pub mod object;
pub mod scene;

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

    fn reflective(&self) -> Option<f64> {
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

    fn canvas_viewport_x_step(&self, canvas: &Canvas) -> f64 {
        self.viewport.width() / canvas.width() as f64
    }

    fn canvas_viewport_y_step(&self, canvas: &Canvas) -> f64 {
        self.viewport.height() / canvas.height() as f64
    }

    fn canvas_to_viewport(&self, canvas: &Canvas, cx: i32, cy: i32) -> Vector {
        let vx = cx as f64 * self.canvas_viewport_x_step(canvas);
        let vy = cy as f64 * self.canvas_viewport_y_step(canvas);

        Vector::new(vx, vy, self.viewport.distance())
    }

    fn reflect_ray(&self, ray: Vector, normal: Vector) -> Vector {
        2. * normal * normal.dot(ray) - ray
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
                let r = self.reflect_ray(l, n);
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

                    if self.intersect_closest(point, l, 0.001, 1.).is_some() {
                        continue;
                    }

                    i += diffuse_and_specular(intensity, l, normal);
                }
                Light::Directional {
                    intensity,
                    direction,
                } => {
                    if self
                        .intersect_closest(point, direction, 0.001, f64::INFINITY)
                        .is_some()
                    {
                        continue;
                    }

                    i += diffuse_and_specular(intensity, direction, normal);
                }
            }
        }

        i
    }

    fn intersect_closest(
        &self,
        origin: Vector,
        direction: Vector,
        t_min: f64,
        t_max: f64,
    ) -> Option<(f64, &dyn Raytraceable)> {
        let mut closest: Option<(_, _)> = None;

        for object in self.scene.objects() {
            if let Some(t) = object.intersect_closest(origin, direction, t_min, t_max) {
                assert!(t >= t_min && t <= t_max);
                if closest.is_none() || t < closest.unwrap().0 {
                    closest = Some((t, object.as_ref()));
                }
            }
        }

        closest
    }

    fn trace_ray(
        &self,
        origin: Vector,
        direction: Vector,
        t_min: f64,
        t_max: f64,
        recursion_depth: u8,
    ) -> Color {
        let closest = self.intersect_closest(origin, direction, t_min, t_max);

        if closest.is_none() {
            return self.scene.base_color();
        }

        let (t, object) = closest.unwrap();

        let point = origin + t * direction;
        let normal = object.normal(point);

        let local_color =
            object.color() * self.compute_lighting(point, normal, -direction, object.specular());

        let reflective = object.reflective();

        if recursion_depth == 0 || reflective.is_none() {
            return local_color;
        }

        let reflective = reflective.unwrap();

        let reflected_ray = self.reflect_ray(-direction, normal);
        let reflected_color = self.trace_ray(
            point,
            reflected_ray,
            0.001,
            f64::INFINITY,
            recursion_depth - 1,
        );

        local_color * (1. - reflective) + reflected_color * reflective
    }

    pub fn render(&self, canvas: &mut Canvas) {
        let camera_position = Vector::new(0., 0., 0.);

        let recursion_depth = 3;

        let super_sampling = 2;

        for x in canvas.x_range() {
            for y in canvas.y_range() {
                let d = self.canvas_to_viewport(canvas, x, y);

                let mut color = Color::BLACK;

                let x_step = self.canvas_viewport_x_step(canvas) / super_sampling as f64;
                let y_step = self.canvas_viewport_y_step(canvas) / super_sampling as f64;

                for dx in 0..super_sampling {
                    for dy in 0..super_sampling {
                        let d = d + Vector::new(dx as f64 * x_step, dy as f64 * y_step, 0.);

                        let color_sample = self.trace_ray(
                            camera_position,
                            d,
                            self.viewport.distance(),
                            f64::INFINITY,
                            recursion_depth,
                        );

                        color += color_sample / (super_sampling * super_sampling) as f64;
                    }
                }

                canvas.set_pixel(x, y, color);
            }
        }
    }
}
