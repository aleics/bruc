use crate::mark::base::{BaseMarkProperties, Phases};

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LineMark<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  on: Phases<LineMarkProperties<'a>>,
}

impl<'a> LineMark<'a> {
  pub fn new(props: LineMarkProperties<'a>) -> LineMark<'a> {
    LineMark {
      on: Phases::new(props),
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct LineMarkProperties<'a> {
  #[cfg_attr(feature = "serde", serde(default))]
  interpolate: Interpolate,

  #[cfg_attr(feature = "serde", serde(borrow))]
  #[cfg_attr(feature = "serde", serde(flatten))]
  base: BaseMarkProperties<'a>,
}

impl<'a> LineMarkProperties<'a> {
  pub fn new(interpolate: Interpolate, base: BaseMarkProperties<'a>) -> LineMarkProperties<'a> {
    LineMarkProperties { interpolate, base }
  }
}

#[derive(Debug, PartialEq)]
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
  use crate::mark::base::{BaseMarkProperties, DataSource};
  use crate::mark::line::{Interpolate, LineMark, LineMarkProperties};

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
        Interpolate::Linear,
        BaseMarkProperties::new(
          Some(DataSource::field("x", Some("xscale"))),
          Some(DataSource::field("y", Some("yscale"))),
          None,
          None,
        )
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
        Interpolate::Linear,
        BaseMarkProperties::new(
          Some(DataSource::field("x", Some("xscale"))),
          Some(DataSource::field("y", Some("yscale"))),
          Some(DataSource::value(100.0.into())),
          Some(DataSource::value(100.0.into())),
        )
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
        Interpolate::Linear,
        BaseMarkProperties::new(
          Some(DataSource::field("x", Some("xscale"))),
          Some(DataSource::field("y", Some("yscale"))),
          Some(DataSource::value(100.0.into())),
          Some(DataSource::value(100.0.into())),
        )
      )
    );
  }
}
