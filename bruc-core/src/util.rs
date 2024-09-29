use core::f32;

pub(crate) fn radians_to_degrees(radians: f32) -> f32 {
  radians * 180.0 / f32::consts::PI
}
