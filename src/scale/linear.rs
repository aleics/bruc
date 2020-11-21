use crate::scale::domain::Domain;
use crate::scale::Scaler;

const fn default_range() -> (f32, f32) {
  (0.0, 1.0)
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LinearScale<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  name: &'a str,

  #[cfg_attr(feature = "serde", serde(default = "Domain::default_literal"))]
  domain: Domain,

  #[cfg_attr(feature = "serde", serde(default = "default_range"))]
  range: (f32, f32),
}

impl<'a> LinearScale<'a> {
  pub fn new(name: &'a str, domain: Domain, range: (f32, f32)) -> LinearScale<'a> {
    LinearScale {
      name,
      domain,
      range,
    }
  }

  pub fn name(&self) -> &'a str {
    &self.name
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
    let Domain::Literal(domain) = &self.domain;
    interpolate(normalize(value, *domain.value()), self.range)
  }
}

fn normalize(x: f32, (min, max): (f32, f32)) -> f32 {
  let x = clamp(x, (min, max));
  (x - min) / max
}

fn interpolate(x: f32, (min, max): (f32, f32)) -> f32 {
  (max - min) * x + min
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

#[cfg(test)]
mod tests {
  use crate::scale::domain::{Domain, DomainLiteral};
  use crate::scale::linear::LinearScale;
  use crate::scale::Scaler;

  #[test]
  fn applies() {
    let scale = LinearScale::new(
      "x",
      Domain::Literal(DomainLiteral::new((0.0, 10.0))),
      (0.0, 100.0),
    );
    assert_eq!(scale.scale(5.0), 50.0);
    assert_eq!(scale.scale(10.0), 100.0);
    assert_eq!(scale.scale(0.0), 0.0);
  }

  #[test]
  fn clamps() {
    let scale = LinearScale::new(
      "x",
      Domain::Literal(DomainLiteral::new((0.0, 10.0))),
      (0.0, 100.0),
    );
    assert_eq!(scale.scale(12.0), 100.0);
    assert_eq!(scale.scale(-2.0), 0.0);
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::scale::domain::{Domain, DomainLiteral};
  use crate::scale::linear::LinearScale;

  #[test]
  fn deserialize_linear_scale() {
    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
      "name": "x",
      "domain": { "type": "literal", "value": [0, 100] },
      "range": [0, 1]
    }"#,
    )
    .unwrap();

    assert_eq!(
      linear_scale,
      LinearScale::new(
        "x",
        Domain::Literal(DomainLiteral::new((0.0, 100.0))),
        (0.0, 1.0),
      )
    )
  }

  #[test]
  fn deserialize_linear_scale_default_domain() {
    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
      "name": "x",
      "range": [0, 1]
    }"#,
    )
    .unwrap();

    assert_eq!(
      linear_scale,
      LinearScale::new(
        "x",
        Domain::Literal(DomainLiteral::new((0.0, 1.0))),
        (0.0, 1.0),
      )
    )
  }

  #[test]
  fn deserialize_linear_scale_default_range() {
    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
      "name": "x",
      "domain": { "type": "literal", "value": [0, 100] }
    }"#,
    )
    .unwrap();

    assert_eq!(
      linear_scale,
      LinearScale::new(
        "x",
        Domain::Literal(DomainLiteral::new((0.0, 100.0))),
        (0.0, 1.0),
      )
    )
  }
}
