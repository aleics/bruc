use domain::Domain;
use log::LogScale;

use crate::spec::scale::linear::LinearScale;

use self::band::BandScale;

pub mod band;
pub mod domain;
pub mod linear;
pub mod log;
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
  Band(BandScale),
  Log(LogScale),
}

impl ScaleKind {
  pub(crate) fn domain(&self) -> &Domain {
    match self {
      ScaleKind::Linear(linear) => &linear.domain,
      ScaleKind::Log(log) => &log.domain,
      ScaleKind::Band(band) => &band.domain,
    }
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::spec::scale::band::BandScale;
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::log::LogScale;
  use crate::spec::scale::range::Range;
  use crate::spec::scale::{Scale, ScaleKind};

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
        ScaleKind::Linear(LinearScale {
          domain: Domain::Literal(vec![0.0, 100.0]),
          range: Range::Literal(0.0, 2.0)
        })
      )
    )
  }

  #[test]
  fn deserialize_scale_log() {
    let scale: Scale = serde_json::from_str(
      r#"{
        "type": "log",
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
        ScaleKind::Log(LogScale {
          domain: Domain::Literal(vec![0.0, 100.0]),
          range: Range::Literal(0.0, 2.0)
        })
      )
    )
  }

  #[test]
  fn deserialize_scale_band() {
    let scale: Scale = serde_json::from_str(
      r#"{
        "type": "band",
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
        ScaleKind::Band(BandScale {
          domain: Domain::Literal(vec![0.0, 100.0]),
          range: Range::Literal(0.0, 2.0),
          padding: 0.0
        })
      )
    )
  }
}
