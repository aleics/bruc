use crate::spec::data::DataEntry;
use crate::spec::scale::Scale;
use crate::spec::shape::Shape;

pub mod data;
pub mod scale;
pub mod shape;
pub mod transform;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Specification {
  #[cfg_attr(feature = "serde", serde(default))]
  pub(crate) dimensions: Dimensions,
  pub(crate) data: Vec<DataEntry>,
  pub(crate) scales: Vec<Scale>,
  pub(crate) shapes: Vec<Shape>,
}

impl Specification {
  pub fn new(
    dimensions: Dimensions,
    data: Vec<DataEntry>,
    scales: Vec<Scale>,
    shapes: Vec<Shape>,
  ) -> Self {
    Specification {
      dimensions,
      data,
      scales,
      shapes,
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Dimensions {
  pub width: usize,
  pub height: usize,
}

impl Dimensions {
  pub fn new(width: usize, height: usize) -> Self {
    Dimensions { width, height }
  }
}

impl Default for Dimensions {
  fn default() -> Self {
    Dimensions {
      width: 500,
      height: 200,
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::data::DataValue;
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::range::Range;
  use crate::spec::scale::{Scale, ScaleKind};
  use crate::spec::shape::line::{LinePropertiesBuilder, LineShape};
  use crate::spec::shape::{DataSource, Shape};
  use crate::spec::transform::filter::FilterPipe;
  use crate::spec::transform::pipe::Pipe;
  use crate::spec::{DataEntry, Dimensions};
  use crate::Specification;

  #[test]
  fn deserializes_empty_spec() {
    let spec: Specification = serde_json::from_str(
      r#"{
        "data": [],
        "scales": [],
        "shapes": []
      }"#,
    )
    .unwrap();
    assert_eq!(
      spec,
      Specification::new(Dimensions::default(), vec![], vec![], vec![])
    );
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
        "shapes": [
          {
            "from": "primary",
            "type": "line",
            "properties": {
              "x": { "field": "a", "scale": "horizontal" }
            }
          }
        ]
      }"#,
    )
    .unwrap();
    assert_eq!(
      spec,
      Specification::new(
        Dimensions::default(),
        vec![DataEntry::new(
          "primary",
          vec![
            DataValue::from_pairs(vec![("a", 10.0.into()), ("b", 1.0.into())]),
            DataValue::from_pairs(vec![("a", 0.0.into()), ("b", 5.0.into())]),
            DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 3.0.into())])
          ],
          vec![Pipe::Filter(FilterPipe::new("a > 2").unwrap())]
        )],
        vec![Scale::new(
          "horizontal",
          ScaleKind::Linear(LinearScale::new(
            Domain::Literal(0.0, 100.0),
            Range::Literal(0.0, 20.0),
          ))
        )],
        vec![Shape::line(
          "primary",
          LineShape::new(
            LinePropertiesBuilder::new()
              .with_x(DataSource::field("a", Some("horizontal")))
              .build()
          ),
        )],
      )
    );
  }
}
