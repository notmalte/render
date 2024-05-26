pub fn interpolate(i0: i32, d0: f64, i1: i32, d1: f64) -> Vec<f64> {
    if i0 == i1 {
        return vec![d0];
    }

    let mut values = vec![];
    let a = (d1 - d0) / (i1 - i0) as f64;
    let mut d = d0;

    for _ in i0..=i1 {
        values.push(d);
        d += a;
    }

    values
}
