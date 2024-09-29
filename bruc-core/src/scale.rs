#[derive(PartialEq, Debug)]
pub(crate) struct Scale {
  kind: ScaleKind,
  range: (f32, f32),
}

impl Scale {
  pub(crate) fn linear(range: (f32, f32)) -> Self {
    Scale {
      kind: ScaleKind::Linear,
      range,
    }
  }

  pub(crate) fn log(range: (f32, f32)) -> Self {
    Scale {
      kind: ScaleKind::Log,
      range,
    }
  }

  pub(crate) fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
    match self.kind {
      ScaleKind::Linear => self.apply_linear(value, domain),
      ScaleKind::Log => self.apply_log(value, domain),
    }
  }

  fn apply_linear(&self, value: f32, domain: (f32, f32)) -> f32 {
    interpolate(normalize(value, domain), self.range)
  }

  fn apply_log(&self, value: f32, domain: (f32, f32)) -> f32 {
    interpolate(normalize_log10(value, domain), self.range)
  }
}

#[derive(PartialEq, Debug)]
enum ScaleKind {
  Linear,
  Log,
}

fn normalize(value: f32, (min, max): (f32, f32)) -> f32 {
  let value = value.clamp(min, max);
  (value - min) / (max - min)
}

fn normalize_by<F>(value: f32, (min, max): (f32, f32), by: F) -> f32
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

fn normalize_log10(value: f32, domain: (f32, f32)) -> f32 {
  normalize_by(value, domain, logn)
}

fn interpolate(value: f32, (min, max): (f32, f32)) -> f32 {
  (max - min) * value + min
}
