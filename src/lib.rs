use crate::scale::Scale;
use crate::transform::Transform;

pub mod data;
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
}

impl<'a> Specification<'a> {
  pub fn new(transform: Option<Transform<'a>>, scales: Vec<Scale<'a>>) -> Specification<'a> {
    Specification { transform, scales }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
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
        "scales": [ 
          {
            "type": "linear",
            "name": "horizontal",
            "domain": [0, 100],
            "range": [0, 20]
          }
        ]
      }"#,
    )
    .unwrap();
    assert_eq!(
      spec,
      Specification::new(
        None,
        vec![Scale::Linear(LinearScale::new(
          "horizontal",
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 20.0),
        ))],
      )
    );
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
      )
    );
  }
}
