/// Calculates the lerp exponent that should be used to achieve `a + (1 - epsilon) * (b - a)`
/// in a lerp within the specified number of frames, where `a` is the lower bound of the lerp, and
/// `b` is the upper bound.
///
/// In case you need this for seconds, use [`lerp_exponent_in_time`].
pub fn lerp_exponent_in_frames(number_of_frames: f32, epsilon: f32) -> f32 {
    dbg!(number_of_frames);
    1.0 - epsilon.powf(1.0 / number_of_frames)
}

/// Calculates the lerp exponent that should be used to achieve `a + (1 - epsilon) * (b - a)`
/// in a lerp within the specified number of seconds, where `a` is the lower bound of the lerp, and
/// `b` is the upper bound.
pub fn lerp_exponent_in_time(time: f32, epsilon: f32, delta_time: f32) -> f32 {
    lerp_exponent_in_frames(time / delta_time, epsilon)
}
