use super::{domain::Domain, range::Range};

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct BandScale {
  #[cfg_attr(feature = "serde", serde(default = "Domain::default_literal"))]
  pub(crate) domain: Domain,

  #[cfg_attr(feature = "serde", serde(default = "Range::default_literal"))]
  pub(crate) range: Range,

  #[cfg_attr(feature = "serde", serde(default))]
  pub(crate) padding: f32,
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::spec::scale::band::BandScale;
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::range::Range;

  #[test]
  fn deserialize_band_scale() {
    let band_scale: BandScale = serde_json::from_str(
      r#"{
        "name": "x",
        "domain": [0, 50, 100],
        "range": [0, 2],
        "padding": 0.05
      }"#,
    )
    .unwrap();

    assert_eq!(
      band_scale,
      BandScale {
        domain: Domain::Literal(vec![0.0, 50.0, 100.0]),
        range: Range::Literal(0.0, 2.0),
        padding: 0.05
      }
    )
  }

  #[test]
  fn deserialize_band_scale_default() {
    let band_scale: BandScale = serde_json::from_str(r#"{ "name": "x" }"#).unwrap();

    assert_eq!(
      band_scale,
      BandScale {
        domain: Domain::Literal(vec![0.0, 1.0]),
        range: Range::Literal(0.0, 1.0),
        padding: 0.0
      }
    )
  }
}
