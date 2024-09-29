use core::f32;

pub(crate) fn normalize(value: f32, (min, max): (f32, f32)) -> f32 {
  let value = value.clamp(min, max);
  (value - min) / (max - min)
}

pub(crate) fn normalize_by<F>(value: f32, (min, max): (f32, f32), by: F) -> f32
where
  F: Fn(f32) -> f32,
{
  normalize(by(value), (by(min), by(max)))
}

fn logn(value: f32) -> f32 {
  if value > 0.0 {
    value.log10()
  } else if value < 0.0 {
    -f32::log10(-value)
  } else {
    1e-10
  }
}

pub(crate) fn normalize_log10(value: f32, domain: (f32, f32)) -> f32 {
  normalize_by(value, domain, logn)
}

pub(crate) fn interpolate(value: f32, (min, max): (f32, f32)) -> f32 {
  (max - min) * value + min
}

pub(crate) fn radians_to_degrees(radians: f32) -> f32 {
  radians * 180.0 / f32::consts::PI
}
