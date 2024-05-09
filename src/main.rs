use crate::{
    canvas::Canvas,
    color::Color,
    raytracer::{object::sphere::Sphere, scene::Scene, Raytracer},
    vector::Vector,
};

mod canvas;
mod color;
mod raytracer;
mod vector;

fn main() {
    let mut c = Canvas::new(512, 512);

    let mut scene = Scene::new();
    scene.add_object(Box::new(Sphere::new(
        Vector::new(0., -1., 3.),
        1.,
        Color::RED,
    )));

    let raytracer = Raytracer::new(scene);

    raytracer.render(&mut c);

    c.display();
}
