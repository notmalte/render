use crate::raytracer::Raytraceable;

pub struct Scene {
    objects: Vec<Box<dyn Raytraceable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn objects(&self) -> &Vec<Box<dyn Raytraceable>> {
        &self.objects
    }

    pub fn add_object(&mut self, object: Box<dyn Raytraceable>) {
        self.objects.push(object);
    }
}
