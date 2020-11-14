use crate::scale::domain::Domain;
use crate::scale::Scaler;

const DEFAULT_UNIT: (f32, f32) = (0f32, 1f32);

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LinearScale<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  name: &'a str,
  domain: Domain,
  range: (f32, f32),
}

impl<'a> LinearScale<'a> {
  pub fn new(name: &'a str, domain: Option<Domain>, range: Option<(f32, f32)>) -> LinearScale<'a> {
    let domain = domain.unwrap_or(Domain::Literal(DEFAULT_UNIT));
    let range = range.unwrap_or(DEFAULT_UNIT);

    LinearScale {
      name,
      domain,
      range,
    }
  }

  pub fn name(&self) -> &'a str {
    self.name
  }

  pub fn domain(&self) -> &Domain {
    &self.domain
  }

  pub fn range(&self) -> &(f32, f32) {
    &self.range
  }
}

impl<'a> Scaler for LinearScale<'a> {
  type Item = f32;

  fn scale(&self, value: f32) -> Self::Item {
    let Domain::Literal(domain) = self.domain;
    interpolate(normalize(value, domain), self.range)
  }
}

// TODO: replace for native clamp implementation once is stabilised (see: https://github.com/rust-lang/rust/pull/77872)
fn clamp(x: f32, (min, max): (f32, f32)) -> f32 {
  if x < min {
    return min;
  }
  if x > max {
    return max;
  }
  x
}

fn normalize(x: f32, (min, max): (f32, f32)) -> f32 {
  let x = clamp(x, (min, max));
  (x - min) / max
}

fn interpolate(x: f32, (min, max): (f32, f32)) -> f32 {
  (max - min) * x + min
}

#[cfg(test)]
mod tests {
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::Scaler;

  #[test]
  fn defaults_domain() {
    let scale = LinearScale::new("x", None, Some((0.0, 100.0)));
    assert_eq!(scale.domain(), &Domain::Literal((0.0, 1.0)));
  }

  #[test]
  fn defaults_range() {
    let scale = LinearScale::new("x", Some(Domain::Literal((0.0, 100.0))), None);
    assert_eq!(scale.range(), &(0.0, 1.0));
  }

  #[test]
  fn applies() {
    let scale = LinearScale::new("x", Some(Domain::Literal((0.0, 10.0))), Some((0.0, 100.0)));
    assert_eq!(scale.scale(5.0), 50.0);
    assert_eq!(scale.scale(10.0), 100.0);
    assert_eq!(scale.scale(0.0), 0.0);
  }

  #[test]
  fn clamps() {
    let scale = LinearScale::new("x", Some(Domain::Literal((0.0, 10.0))), Some((0.0, 100.0)));
    assert_eq!(scale.scale(12.0), 100.0);
    assert_eq!(scale.scale(-2.0), 0.0);
  }
}
