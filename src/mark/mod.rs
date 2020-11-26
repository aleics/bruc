use crate::mark::line::LineMark;

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

#[cfg(test)]
#[cfg(feature = "serde")]
mod serde_tests {
  use crate::mark::base::{BaseMarkProperties, DataSource};
  use crate::mark::line::{Interpolate, LineMark, LineMarkProperties};
  use crate::mark::Mark;

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
          Interpolate::Linear,
          BaseMarkProperties::new(
            Some(DataSource::field("x", Some("xscale"))),
            Some(DataSource::field("y", Some("yscale"))),
            None,
            None,
          ),
        )),
      )
    );
  }
}
