use render::{
    canvas::Canvas,
    color::Color,
    rasterizer::{line::draw_line, point::Point},
};

fn main() {
    let mut c = Canvas::new(512, 512);

    draw_line(
        &mut c,
        Point::new(-50, -50),
        Point::new(50, 50),
        Color::WHITE,
    );

    draw_line(
        &mut c,
        Point::new(-50, 50),
        Point::new(50, -50),
        Color::WHITE,
    );

    c.display();
}
