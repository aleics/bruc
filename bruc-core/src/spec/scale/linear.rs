use crate::spec::scale::domain::Domain;
use crate::spec::scale::range::Range;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LinearScale {
  #[cfg_attr(feature = "serde", serde(default = "Domain::default_literal"))]
  pub(crate) domain: Domain,

  #[cfg_attr(feature = "serde", serde(default = "Range::default_literal"))]
  pub(crate) range: Range,
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::range::Range;

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
      LinearScale {
        domain: Domain::Literal(0.0, 100.0),
        range: Range::Literal(0.0, 1.0)
      }
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
      LinearScale {
        domain: Domain::Literal(0.0, 1.0),
        range: Range::Literal(0.0, 1.0)
      }
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
      LinearScale {
        domain: Domain::Literal(0.0, 100.0),
        range: Range::Literal(0.0, 1.0)
      }
    )
  }
}
