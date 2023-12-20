/// Delta-time independent exponential function exponent.
/// Most commonly used as an argument to `lerp`.
pub fn delta_time_independent_lerp_exponent(exponent: f32, delta_time: f32) -> f32 {
    const BASE_DELTA_TIME: f32 = 1.0 / 60.0;

    let speed_factor = delta_time / BASE_DELTA_TIME;
    1.0 - (1.0 - exponent).powf(speed_factor)
}
