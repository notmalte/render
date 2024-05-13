use render::{canvas::Canvas, color::Color};

fn main() {
    let mut c = Canvas::new(512, 512);

    for x in -10..10 {
        for y in -10..10 {
            c.set_pixel(x, y, Color::RED);
        }
    }

    c.display();
}
