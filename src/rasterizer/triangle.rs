use crate::{
    canvas::Canvas,
    color::Color,
    rasterizer::{interpolate::interpolate, line::draw_line, point::Point},
};

pub fn draw_triangle_wireframe(canvas: &mut Canvas, p0: Point, p1: Point, p2: Point, color: Color) {
    draw_line(canvas, p0, p1, color);
    draw_line(canvas, p1, p2, color);
    draw_line(canvas, p2, p0, color);
}

pub fn draw_triangle_filled(
    canvas: &mut Canvas,
    mut p0: Point,
    mut p1: Point,
    mut p2: Point,
    color: Color,
) {
    if p1.y < p0.y {
        (p0, p1) = (p1, p0);
    }

    if p2.y < p0.y {
        (p0, p2) = (p2, p0);
    }

    if p2.y < p1.y {
        (p1, p2) = (p2, p1);
    }

    let x01 = interpolate(p0.y, p0.x as f64, p1.y, p1.x as f64);
    let x12 = interpolate(p1.y, p1.x as f64, p2.y, p2.x as f64);
    let x02 = interpolate(p0.y, p0.x as f64, p2.y, p2.x as f64);

    let x012 = x01
        .into_iter()
        .chain(x12.into_iter().skip(1))
        .collect::<Vec<_>>();

    let m = x02.len() / 2;

    let (x_left, x_right) = if x02[m] < x012[m] {
        (x02, x012)
    } else {
        (x012, x02)
    };

    for (y, (x_l, x_r)) in (p0.y..=p2.y).zip(x_left.into_iter().zip(x_right.into_iter())) {
        for x in x_l as i32..=x_r as i32 {
            canvas.set_pixel(x, y, color);
        }
    }
}
