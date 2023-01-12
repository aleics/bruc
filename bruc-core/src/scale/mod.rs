use crate::scale::linear::LinearScale;
use bruc_expression::data::DataItem;

pub mod domain;
pub mod linear;
pub mod range;

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Scale {
  pub(crate) name: String,

  #[cfg_attr(feature = "serde", serde(flatten))]
  pub(crate) kind: ScaleKind,
}

impl Scale {
  pub fn new(name: &str, kind: ScaleKind) -> Self {
    Scale {
      name: name.to_string(),
      kind,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum ScaleKind {
  Linear(LinearScale),
}

pub trait Scaler {
  type Item;
  fn scale(&self, value: &DataItem) -> Option<Self::Item>;
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;
  use crate::scale::{Scale, ScaleKind};

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
      Scale::new(
        "x",
        ScaleKind::Linear(LinearScale::new(
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 2.0)
        ))
      )
    )
  }
}
