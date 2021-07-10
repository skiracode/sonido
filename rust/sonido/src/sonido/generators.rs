use std::f64::consts::PI;
pub fn sine(time: f64) -> f64 {
    (time * PI).sin()
}
pub fn square(time: f64) -> f64 {
    1.0
}
pub fn saw(time: f64) -> f64 {
    time
}
pub fn ellipsis(mut time: f64) -> f64 {
    return 1.0 - (1.0 - time * time).sqrt();
}
pub fn quad(mut time: f64) -> f64 {
    time * time
}
pub fn triangle(time: f64) -> f64 {
    if time < 0.5 {
        return time * 2.0;
    }
    return 2.0 - 2.0 * time;
}