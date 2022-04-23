use std::mem;

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

/// Sorts in ascending order.
pub fn simple_bubble_sort_vector_by_y(
    mv0x: &mut i32,
    mv0y: &mut i32,
    mv1x: &mut i32,
    mv1y: &mut i32,
    mv2x: &mut i32,
    mv2y: &mut i32,
) {
    if mv0y > mv1y {
        mem::swap(mv0x, mv1x);
        mem::swap(mv0y, mv1y);
    }
    if mv0y > mv2y {
        mem::swap(mv0x, mv2x);
        mem::swap(mv0y, mv2y);
    }
    if mv1y > mv2y {
        mem::swap(mv1x, mv2x);
        mem::swap(mv1y, mv2y);
    }
}
