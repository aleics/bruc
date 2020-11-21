use crate::scale::linear::LinearScale;

pub mod domain;
pub mod linear;
pub mod range;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Scale<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  Linear(LinearScale<'a>),
}

pub trait Scaler {
  type Item;

  fn scale(&self, value: Self::Item) -> Self::Item;
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;
  use crate::scale::Scale;

  #[test]
  fn deserialize_scale_linear() {
    let scale: Scale = serde_json::from_str(
      r#"{
        "type": "linear",
        "name": "x",
        "domain": [0, 100],
        "range": [0, 2]
      }"#,
    )
    .unwrap();

    assert_eq!(
      scale,
      Scale::Linear(LinearScale::new(
        "x",
        Domain::Literal(0.0, 100.0),
        Range::Literal(0.0, 2.0)
      ))
    )
  }
}
