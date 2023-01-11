#![feature(async_fn_in_trait)]

use data::DataEntry;

use crate::mark::Mark;
use crate::scale::Scale;

pub mod data;
pub mod flow;
pub mod graph;
pub mod mark;
pub mod parser;
pub mod scale;
pub mod transform;

#[derive(Debug, PartialEq)]
pub struct Engine {
  spec: Specification,
}

impl Engine {
  pub fn new(spec: Specification) -> Engine {
    Engine { spec }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Specification {
  data: Vec<DataEntry>,
  scales: Vec<Scale>,
  marks: Vec<Mark>,
}

impl Specification {
  pub fn new(data: Vec<DataEntry>, scales: Vec<Scale>, marks: Vec<Mark>) -> Self {
    Specification {
      data,
      scales,
      marks,
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::data::{DataEntry, DataValue};
  use crate::mark::line::{Interpolate, LineMark, LineMarkProperties};
  use crate::mark::{DataSource, Mark};
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;
  use crate::scale::Scale;
  use crate::transform::filter::FilterPipe;
  use crate::transform::pipe::Pipe;
  use crate::Specification;

  #[test]
  fn deserializes_empty_spec() {
    let spec: Specification = serde_json::from_str(
      r#"{
        "data": [],
        "scales": [],
        "marks": []
      }"#,
    )
    .unwrap();
    assert_eq!(spec, Specification::new(vec![], vec![], vec![]));
  }

  #[test]
  fn deserializes_spec() {
    let spec: Specification = serde_json::from_str(
      r#"{
        "data": [
          {
            "name": "primary",
            "values": [
              { "a": 10, "b": 1 },
              { "a": 0, "b": 5 },
              { "a": 3, "b": 3 }
            ],
            "transform": [
              { "type": "filter", "fn": "a > 2" }
            ]
          }
        ],
        "scales": [ 
          {
            "type": "linear",
            "name": "horizontal",
            "domain": [0, 100],
            "range": [0, 20]
          }
        ],
        "marks": [
          {
            "from": "valid",
            "type": "line",
            "on": {
              "update": {
                "x": { "field": "x", "scale": "horizontal" }
              }
            }
          }
        ]
      }"#,
    )
    .unwrap();
    assert_eq!(
      spec,
      Specification::new(
        vec![DataEntry::new(
          "primary",
          vec![
            DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
            DataValue::from_pairs(vec![("a", 0.0.into()), ("b", 5.0.into())]),
            DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 3.0.into())])
          ],
          vec![Pipe::Filter(FilterPipe::new("a > 2").unwrap())]
        )],
        vec![Scale::Linear(LinearScale::new(
          "horizontal",
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 20.0),
        ))],
        vec![Mark::line(
          "valid",
          LineMark::new(LineMarkProperties::new(
            Some(DataSource::field("x", Some("horizontal"))),
            None,
            None,
            None,
            Interpolate::Linear,
          )),
        )],
      )
    );
  }
}
