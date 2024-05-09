use crate::color::Color;

mod canvas;
mod color;

fn main() {
    let mut c = canvas::Canvas::new(800, 600);

    c.set_pixel(0, 0, Color::RED);
    c.set_pixel(100, 100, Color::GREEN);
    c.set_pixel(-100, -100, Color::BLUE);

    c.display();
}
