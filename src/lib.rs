use crate::mark::Mark;
use crate::scale::Scale;
use crate::transform::Transform;

pub mod data;
pub mod flow;
pub mod mark;
pub mod scale;
pub mod transform;

#[derive(Debug, PartialEq)]
pub struct Engine<'a> {
  spec: Specification<'a>,
}

impl<'a> Engine<'a> {
  pub fn new(spec: Specification<'a>) -> Engine<'a> {
    Engine { spec }
  }
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Specification<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  transform: Option<Transform<'a>>,
  scales: Vec<Scale<'a>>,
  marks: Vec<Mark<'a>>,
}

impl<'a> Specification<'a> {
  pub fn new(
    transform: Option<Transform<'a>>,
    scales: Vec<Scale<'a>>,
    marks: Vec<Mark<'a>>,
  ) -> Specification<'a> {
    Specification {
      transform,
      scales,
      marks,
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
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
        "scales": [],
        "marks": []
      }"#,
    )
    .unwrap();
    assert_eq!(spec, Specification::new(None, vec![], vec![]));
  }

  #[test]
  fn deserializes_spec() {
    let spec: Specification = serde_json::from_str(
      r#"{
        "transform": {
          "source": "primary",
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
            "from": "primary",
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
        Some(Transform::new(
          "primary",
          vec![Pipe::Filter(FilterPipe::new("a > 2").unwrap())],
        )),
        vec![Scale::Linear(LinearScale::new(
          "horizontal",
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 20.0),
        ))],
        vec![Mark::line(
          "primary",
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
