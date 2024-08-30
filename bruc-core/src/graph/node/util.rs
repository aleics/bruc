use core::f32;

pub(crate) fn normalize(x: f32, (min, max): (f32, f32)) -> f32 {
  let x = x.clamp(min, max);
  (x - min) / (max - min)
}

pub(crate) fn interpolate(x: f32, (min, max): (f32, f32)) -> f32 {
  (max - min) * x + min
}

pub(crate) fn radians_to_degrees(radians: f32) -> f32 {
  radians * 180.0 / f32::consts::PI
}
