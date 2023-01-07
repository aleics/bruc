use crate::mark::base::{BaseMarkProperties, Phases};
use crate::mark::DataSource;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LineMark {
  pub(crate) on: Phases<LineMarkProperties>,
}

impl LineMark {
  pub fn new(props: LineMarkProperties) -> LineMark {
    LineMark {
      on: Phases::new(props),
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LineMarkProperties {
  #[cfg_attr(feature = "serde", serde(default))]
  pub(crate) interpolate: Interpolate,

  #[cfg_attr(feature = "serde", serde(flatten))]
  pub(crate) base: BaseMarkProperties,
}

impl LineMarkProperties {
  pub fn new(
    x: Option<DataSource>,
    y: Option<DataSource>,
    width: Option<DataSource>,
    height: Option<DataSource>,
    interpolate: Interpolate,
  ) -> LineMarkProperties {
    LineMarkProperties {
      interpolate,
      base: BaseMarkProperties::new(x, y, width, height),
    }
  }
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Interpolate {
  Linear,
}

impl Default for Interpolate {
  fn default() -> Self {
    Interpolate::Linear
  }
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::mark::line::{Interpolate, LineMark, LineMarkProperties};
  use crate::mark::DataSource;

  #[test]
  fn deserialize_line_mark() {
    let line_mark: LineMark = serde_json::from_str(
      r#"{
        "on": {
          "update": {
            "x": { "field": "x", "scale": "xscale" },
            "y": { "field": "y", "scale": "yscale" }
          }
        }
      }"#,
    )
    .unwrap();

    assert_eq!(
      line_mark,
      LineMark::new(LineMarkProperties::new(
        Some(DataSource::field("x", Some("xscale"))),
        Some(DataSource::field("y", Some("yscale"))),
        None,
        None,
        Interpolate::Linear,
      ))
    )
  }

  #[test]
  fn deserialize_props() {
    let props: LineMarkProperties = serde_json::from_str(
      r#"{
        "x": { "field": "x", "scale": "xscale" },
        "y": { "field": "y", "scale": "yscale" },
        "width": 100,
        "height": 100
      }"#,
    )
    .unwrap();
    assert_eq!(
      props,
      LineMarkProperties::new(
        Some(DataSource::field("x", Some("xscale"))),
        Some(DataSource::field("y", Some("yscale"))),
        Some(DataSource::value(100.0.into())),
        Some(DataSource::value(100.0.into())),
        Interpolate::Linear,
      )
    );

    let props: LineMarkProperties = serde_json::from_str(
      r#"{
        "x": { "field": "x", "scale": "xscale" },
        "y": { "field": "y", "scale": "yscale" },
        "width": 100,
        "height": 100,
        "interpolate": "linear"
      }"#,
    )
    .unwrap();
    assert_eq!(
      props,
      LineMarkProperties::new(
        Some(DataSource::field("x", Some("xscale"))),
        Some(DataSource::field("y", Some("yscale"))),
        Some(DataSource::value(100.0.into())),
        Some(DataSource::value(100.0.into())),
        Interpolate::Linear,
      )
    );
  }
}
