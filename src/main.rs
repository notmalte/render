mod canvas;

fn main() {
    let mut c = canvas::Canvas::new(800, 600);

    for n in 0..32 {
        for x in -400..400 {
            let y = ((((x as f32) / 25.).sin() * 25.) as i32) + (25 * n - 400);

            let r = (((x + 25 * n) as f64 / 25.).sin() * 255. + 127.) as u8;
            let g = ((((x + 25 * n) + 25) as f64 / 25.).sin() * 255. + 127.) as u8;
            let b = ((((x + 25 * n) + 200) as f64 / 25.).sin() * 255. + 127.) as u8;

            c.set_pixel(x, y, r, g, b);
        }
    }

    c.display();
}
