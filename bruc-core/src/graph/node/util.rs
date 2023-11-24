pub(crate) fn normalize(x: f32, (min, max): (f32, f32)) -> f32 {
  let x = x.clamp(min, max);
  (x - min) / max
}

pub(crate) fn interpolate(x: f32, (min, max): (f32, f32)) -> f32 {
  (max - min) * x + min
}
