use render::{
    canvas::Canvas,
    color::Color,
    rasterizer::{
        point::Point,
        triangle::{draw_triangle_filled, draw_triangle_wireframe},
    },
};

fn main() {
    let mut c = Canvas::new(512, 512);

    let p0 = Point::new(0, 115);
    let p1 = Point::new(100, -57);
    let p2 = Point::new(-100, -57);

    draw_triangle_filled(&mut c, p0, p1, p2, Color::RED);
    draw_triangle_wireframe(&mut c, p0, p1, p2, Color::WHITE);

    c.display();
}
