use crate::scale::domain::Domain;
use crate::scale::range::Range;
use crate::scale::Scaler;
use bruc_expreter::data::DataItem;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LinearScale<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  name: &'a str,

  #[cfg_attr(feature = "serde", serde(default = "Domain::default_literal"))]
  domain: Domain,

  #[cfg_attr(feature = "serde", serde(default = "Range::default_literal"))]
  range: Range,
}

impl<'a> LinearScale<'a> {
  pub fn new(name: &'a str, domain: Domain, range: Range) -> LinearScale<'a> {
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

  pub fn range(&self) -> &Range {
    &self.range
  }
}

impl Scaler for LinearScale<'_> {
  type Item = f32;

  fn scale(&self, value: &DataItem) -> Option<Self::Item> {
    let Domain::Literal(domain_min, domain_max) = &self.domain;
    let Range::Literal(range_min, range_max) = &self.range;

    match value {
      DataItem::Bool(_) => None,
      DataItem::Number(value) => Some(interpolate(
        normalize(*value, (*domain_min, *domain_max)),
        (*range_min, *range_max),
      )),
    }
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
    min
  } else if x > max {
    max
  } else {
    x
  }
}

#[cfg(test)]
mod tests {
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;
  use crate::scale::Scaler;

  #[test]
  fn applies() {
    let scale = LinearScale::new("x", Domain::Literal(0.0, 10.0), Range::Literal(0.0, 100.0));
    assert_eq!(scale.scale(&5.0.into()), Some(50.0));
    assert_eq!(scale.scale(&10.0.into()), Some(100.0));
    assert_eq!(scale.scale(&0.0.into()), Some(0.0));
    assert_eq!(scale.scale(&true.into()), None);
    assert_eq!(scale.scale(&false.into()), None);
  }

  #[test]
  fn clamps() {
    let scale = LinearScale::new("x", Domain::Literal(0.0, 10.0), Range::Literal(0.0, 100.0));
    assert_eq!(scale.scale(&12.0.into()), Some(100.0));
    assert_eq!(scale.scale(&(-2.0).into()), Some(0.0));
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;

  #[test]
  fn deserialize_linear_scale() {
    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
        "name": "x",
        "domain": [0, 100],
        "range": [0, 1]
      }"#,
    )
    .unwrap();

    assert_eq!(
      linear_scale,
      LinearScale::new("x", Domain::Literal(0.0, 100.0), Range::Literal(0.0, 1.0))
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
      LinearScale::new("x", Domain::Literal(0.0, 1.0), Range::Literal(0.0, 1.0))
    )
  }

  #[test]
  fn deserialize_linear_scale_default_range() {
    let linear_scale: LinearScale = serde_json::from_str(
      r#"{
        "name": "x",
        "domain": [0, 100]
      }"#,
    )
    .unwrap();

    assert_eq!(
      linear_scale,
      LinearScale::new("x", Domain::Literal(0.0, 100.0), Range::Literal(0.0, 1.0))
    )
  }
}
