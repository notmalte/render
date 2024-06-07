use render::{
    canvas::Canvas,
    color::Color,
    rasterizer::{line::draw_line, Rasterizer},
    vector::Vector,
    viewport::Viewport,
};

fn main() {
    let mut c = Canvas::new(512, 512);

    let r = Rasterizer::new(Viewport::default());

    let vaf = r.project_vertex(Vector::new(-2., -0.5, 5.), &c);
    let vbf = r.project_vertex(Vector::new(-2., 0.5, 5.), &c);
    let vcf = r.project_vertex(Vector::new(-1., 0.5, 5.), &c);
    let vdf = r.project_vertex(Vector::new(-1., -0.5, 5.), &c);

    let vab = r.project_vertex(Vector::new(-2., -0.5, 6.), &c);
    let vbb = r.project_vertex(Vector::new(-2., 0.5, 6.), &c);
    let vcb = r.project_vertex(Vector::new(-1., 0.5, 6.), &c);
    let vdb = r.project_vertex(Vector::new(-1., -0.5, 6.), &c);

    draw_line(&mut c, vaf, vbf, Color::WHITE);
    draw_line(&mut c, vbf, vcf, Color::WHITE);
    draw_line(&mut c, vcf, vdf, Color::WHITE);
    draw_line(&mut c, vdf, vaf, Color::WHITE);

    draw_line(&mut c, vab, vbb, Color::WHITE);
    draw_line(&mut c, vbb, vcb, Color::WHITE);
    draw_line(&mut c, vcb, vdb, Color::WHITE);
    draw_line(&mut c, vdb, vab, Color::WHITE);

    draw_line(&mut c, vaf, vab, Color::WHITE);
    draw_line(&mut c, vbf, vbb, Color::WHITE);
    draw_line(&mut c, vcf, vcb, Color::WHITE);
    draw_line(&mut c, vdf, vdb, Color::WHITE);

    c.display();
}
