use crate::{
    canvas::Canvas,
    color::Color,
    rasterizer::{interpolate::interpolate, point::Point},
};

pub fn draw_line(canvas: &mut Canvas, mut p0: Point, mut p1: Point, color: Color) {
    let dx = p1.x - p0.x;
    let dy = p1.y - p0.y;

    if dx.abs() > dy.abs() {
        if p0.x > p1.x {
            (p0, p1) = (p1, p0);
        }

        let ys = interpolate(p0.x, p0.y as f64, p1.x, p1.y as f64);
        for (x, y) in (p0.x..=p1.x).zip(ys) {
            canvas.set_pixel(x, y as i32, color);
        }
    } else {
        if p0.y > p1.y {
            (p0, p1) = (p1, p0);
        }

        let xs = interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64);
        for (y, x) in (p0.y..=p1.y).zip(xs) {
            canvas.set_pixel(x as i32, y, color);
        }
    }
}
