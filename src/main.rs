use crate::{
    canvas::Canvas,
    color::Color,
    raytracer::{light::Light, object::sphere::Sphere, scene::Scene, Raytracer},
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
        Some(500.),
        Some(0.2),
    )));
    scene.add_object(Box::new(Sphere::new(
        Vector::new(2., 0., 4.),
        1.,
        Color::BLUE,
        Some(500.),
        Some(0.3),
    )));
    scene.add_object(Box::new(Sphere::new(
        Vector::new(-2., 0., 4.),
        1.,
        Color::GREEN,
        Some(10.),
        Some(0.4),
    )));
    scene.add_object(Box::new(Sphere::new(
        Vector::new(0., -5001., 0.),
        5000.,
        Color::YELLOW,
        Some(1000.),
        Some(0.5),
    )));

    scene.add_light(Light::Ambient { intensity: 0.2 });
    scene.add_light(Light::Point {
        intensity: 0.6,
        position: Vector::new(2., 1., 0.),
    });
    scene.add_light(Light::Directional {
        intensity: 0.2,
        direction: Vector::new(1., 4., 4.),
    });

    let raytracer = Raytracer::new(scene);

    raytracer.render(&mut c);

    c.display();
}
