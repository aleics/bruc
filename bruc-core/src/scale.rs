use crate::spec::scale::range::Range;
use crate::spec::scale::Scale as ScaleSpec;
use crate::spec::scale::ScaleKind as ScaleKindSpec;

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Scale {
  kind: ScaleKind,
}

impl Scale {
  pub(crate) fn from_spec(scale: &ScaleSpec) -> Self {
    match &scale.kind {
      ScaleKindSpec::Linear(linear) => {
        let Range::Literal(min, max) = linear.range;
        Scale::linear((min, max))
      }
      ScaleKindSpec::Band(band) => {
        let Range::Literal(min, max) = band.range;
        Scale::band((min, max))
      }
      ScaleKindSpec::Log(log) => {
        let Range::Literal(min, max) = log.range;
        Scale::log((min, max))
      }
    }
  }

  pub(crate) fn linear(range: (f32, f32)) -> Self {
    Scale {
      kind: ScaleKind::Linear(ScaleLinear { range }),
    }
  }

  pub(crate) fn band(range: (f32, f32)) -> Self {
    Scale {
      kind: ScaleKind::Band(ScaleBand { range }),
    }
  }

  pub(crate) fn log(range: (f32, f32)) -> Self {
    Scale {
      kind: ScaleKind::Log(ScaleLog { range }),
    }
  }

  pub(crate) fn range(&self) -> (f32, f32) {
    match &self.kind {
      ScaleKind::Linear(linear) => linear.range,
      ScaleKind::Log(log) => log.range,
      ScaleKind::Band(band) => band.range,
    }
  }

  pub(crate) fn ticks(&self, domain: (f32, f32)) -> Vec<(f32, f32)> {
    match &self.kind {
      ScaleKind::Linear(linear) => linear.ticks(domain),
      ScaleKind::Log(log) => log.ticks(domain),
      ScaleKind::Band(band) => band.ticks(domain),
    }
  }

  pub(crate) fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
    match &self.kind {
      ScaleKind::Linear(linear) => linear.apply(value, domain),
      ScaleKind::Log(log) => log.apply(value, domain),
      ScaleKind::Band(band) => band.apply(value, domain),
    }
  }
}

#[derive(PartialEq, Debug, Clone)]
enum ScaleKind {
  Linear(ScaleLinear),
  Log(ScaleLog),
  Band(ScaleBand),
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct ScaleLinear {
  pub(crate) range: (f32, f32),
}

impl ScaleLinear {
  fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
    interpolate(normalize(value, domain), self.range)
  }

  fn ticks(&self, domain: (f32, f32)) -> Vec<(f32, f32)> {
    create_tick_relative_positions(10, domain)
      .into_iter()
      .map(|value| (self.apply(value, domain), value))
      .collect()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct ScaleLog {
  pub(crate) range: (f32, f32),
}

impl ScaleLog {
  fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
    interpolate(normalize_log10(value, domain), self.range)
  }

  fn ticks(&self, domain: (f32, f32)) -> Vec<(f32, f32)> {
    create_tick_relative_positions(10, domain)
      .into_iter()
      .map(|value| (self.apply(value, domain), value))
      .collect()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct ScaleBand {
  pub(crate) range: (f32, f32),
}

impl ScaleBand {
  fn apply(&self, value: f32, domain: (f32, f32)) -> f32 {
    let count = domain.1 - domain.0 + 1.0;

    let step = (self.range.1 - self.range.0) / count;
    let padding = step / 2.0;
    let range = (self.range.0 + padding, self.range.1 - padding);

    Self::calculate(value, domain, range)
  }

  fn ticks(&self, domain: (f32, f32)) -> Vec<(f32, f32)> {
    let count = domain.1 - domain.0;

    let step = (self.range.1 - self.range.0) / (count + 1.0);
    let padding = step / 2.0;
    let range = (self.range.0 + padding, self.range.1 - padding);

    create_tick_relative_positions(count as usize, domain)
      .into_iter()
      .map(|value| (Self::calculate(value, domain, range), value))
      .collect()
  }

  fn calculate(value: f32, domain: (f32, f32), range: (f32, f32)) -> f32 {
    interpolate(normalize(value, domain), range)
  }
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

fn create_tick_relative_positions(count: usize, (from, to): (f32, f32)) -> Vec<f32> {
  let step = (to - from) / (count as f32);
  (0..count + 1).map(|i| from + step * (i as f32)).collect()
}
