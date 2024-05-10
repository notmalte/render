use crate::{
    color::Color,
    raytracer::{light::Light, Raytraceable},
};

pub struct Scene {
    base_color: Color,
    objects: Vec<Box<dyn Raytraceable>>,
    lights: Vec<Light>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            base_color: Color::BLACK,
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn base_color(&self) -> Color {
        self.base_color
    }

    pub fn objects(&self) -> &Vec<Box<dyn Raytraceable>> {
        &self.objects
    }

    pub fn add_object(&mut self, object: Box<dyn Raytraceable>) {
        self.objects.push(object);
    }

    pub fn lights(&self) -> &Vec<Light> {
        &self.lights
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light);
    }
}
