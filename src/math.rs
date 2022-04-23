pub fn clamp<T: PartialOrd>(n: T, min: T, max: T) -> T {
    if n < min {
        min
    } else if n > max {
        max
    } else {
        n
    }
}

pub fn distance(origin_x: f64, origin_y: f64, x: f64, y: f64) -> f64 {
    let x_dist = (origin_x - x).abs();
    let y_dist = (origin_y - y).abs();
    let dist = (x_dist.powf(2.0) + y_dist.powf(2.0)).sqrt();
    dist
}
