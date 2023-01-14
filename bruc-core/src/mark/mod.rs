use crate::mark::line::LineMark;
use bruc_expression::data::DataItem;

pub(crate) mod base;
pub(crate) mod line;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Mark {
  pub(crate) from: String,
  #[cfg_attr(feature = "serde", serde(flatten))]
  pub(crate) kind: MarkKind,
}

impl Mark {
  pub fn line(from: &str, mark: LineMark) -> Mark {
    Mark {
      from: from.to_string(),
      kind: MarkKind::Line(mark),
    }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum MarkKind {
  Line(LineMark),
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
pub enum DataSource {
  FieldSource {
    field: String,
    scale: Option<String>,
  },
  ValueSource(DataItem),
}

impl DataSource {
  pub fn field(field: &str, scale: Option<&str>) -> DataSource {
    DataSource::FieldSource {
      field: field.to_string(),
      scale: scale.map(|value| value.to_string()),
    }
  }

  pub fn value(item: DataItem) -> DataSource {
    DataSource::ValueSource(item)
  }
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
    assert_eq!(data_source, DataSource::field("x", None));

    let data_source: DataSource =
      serde_json::from_str(r#"{ "field": "x", "scale": "horizontal" }"#).unwrap();
    assert_eq!(data_source, DataSource::field("x", Some("horizontal")));

    let data_source: DataSource = serde_json::from_str(r#"1"#).unwrap();
    assert_eq!(data_source, DataSource::ValueSource(1.0.into()));

    let data_source: DataSource = serde_json::from_str(r#"true"#).unwrap();
    assert_eq!(data_source, DataSource::ValueSource(true.into()));
  }
}
