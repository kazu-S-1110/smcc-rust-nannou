use std;

pub fn calc_area(radius: i32) -> f64 {
    let area_circle = radius as f64 * radius as f64 * std::f64::consts::PI;

    let area_square = (radius * 2) * (radius * 2) / 2;

    return area_circle - area_square as f64;
}
