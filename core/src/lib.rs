use crate::data::Data;
use crate::mark::Mark;
use crate::scale::Scale;
use crate::transform::Transform;

pub mod data;
pub mod flow;
pub mod mark;
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
  data: Data,
  transform: Option<Transform>,
  scales: Vec<Scale>,
  marks: Vec<Mark>,
}

impl Specification {
  pub fn new(
    data: Data,
    transform: Option<Transform>,
    scales: Vec<Scale>,
    marks: Vec<Mark>,
  ) -> Specification {
    Specification {
      data,
      transform,
      scales,
      marks,
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::data::{Data, DataValue};
  use crate::mark::line::{Interpolate, LineMark, LineMarkProperties};
  use crate::mark::{DataSource, Mark};
  use crate::scale::domain::Domain;
  use crate::scale::linear::LinearScale;
  use crate::scale::range::Range;
  use crate::scale::Scale;
  use crate::transform::filter::FilterPipe;
  use crate::transform::pipe::Pipe;
  use crate::transform::Transform;
  use crate::Specification;

  #[test]
  fn deserializes_empty_spec() {
    let spec: Specification = serde_json::from_str(
      r#"{
        "data": {},
        "scales": [],
        "marks": []
      }"#,
    )
    .unwrap();
    assert_eq!(
      spec,
      Specification::new(Data::from_pairs(vec![]), None, vec![], vec![])
    );
  }

  #[test]
  fn deserializes_spec() {
    let spec: Specification = serde_json::from_str(
      r#"{
        "data": {
          "primary": [
            { "a": 10, "b": 1 },
            { "a": 0, "b": 5 },
            { "a": 3, "b": 3 }
          ]  
        },      
        "transform": {
          "from": "primary",
          "as": "valid",
          "pipes": [
            { "type": "filter", "fn": "a > 2" }
          ]
        },
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
        Data::from_pairs(vec![(
          "primary",
          vec![
            DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
            DataValue::from_pairs(vec![("a", 0.0.into()), ("b", 5.0.into())]),
            DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 3.0.into())])
          ]
        )]),
        Some(Transform::new(
          "primary",
          "valid",
          vec![Pipe::Filter(FilterPipe::new("a > 2").unwrap())],
        )),
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
