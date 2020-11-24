use crate::mark::line::LineMark;
use bruc_expreter::data::DataItem;

mod base;
pub mod line;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Mark<'a> {
  from: &'a str,
  #[cfg_attr(feature = "serde", serde(flatten))]
  kind: MarkKind<'a>,
}

impl<'a> Mark<'a> {
  pub fn line(from: &'a str, mark: LineMark<'a>) -> Mark<'a> {
    Mark {
      from,
      kind: MarkKind::Line(mark),
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
enum MarkKind<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  Line(LineMark<'a>),
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataSource<'a> {
  FieldSource {
    field: &'a str,
    scale: Option<&'a str>,
  },
  ValueSource(DataItem),
}

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::mark::line::{Interpolate, LineMark, LineMarkProperties};
  use crate::mark::{DataSource, Mark};

  #[test]
  fn deserialize_mark() {
    let mark: Mark = serde_json::from_str(
      r#"{
        "from": "table",
        "type": "line",
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
      mark,
      Mark::line(
        "table",
        LineMark::new(LineMarkProperties::new(
          Some(DataSource::field("x", Some("xscale"))),
          Some(DataSource::field("y", Some("yscale"))),
          None,
          None,
          Interpolate::Linear,
        )),
      )
    );
  }

  #[test]
  fn deserialize_data_source() {
    let data_source: DataSource = serde_json::from_str(r#"{ "field": "x" }"#).unwrap();
    assert_eq!(
      data_source,
      DataSource::FieldSource {
        field: "x",
        scale: None,
      }
    );

    let data_source: DataSource =
      serde_json::from_str(r#"{ "field": "x", "scale": "horizontal" }"#).unwrap();
    assert_eq!(
      data_source,
      DataSource::FieldSource {
        field: "x",
        scale: Some("horizontal"),
      }
    );

    let data_source: DataSource = serde_json::from_str(r#"1"#).unwrap();
    assert_eq!(data_source, DataSource::ValueSource(1.0.into()));

    let data_source: DataSource = serde_json::from_str(r#"true"#).unwrap();
    assert_eq!(data_source, DataSource::ValueSource(true.into()));
  }
}
