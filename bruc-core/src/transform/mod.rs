use crate::transform::pipe::Pipe;

pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Transform {
  pub(crate) from: String,
  #[cfg_attr(feature = "serde", serde(rename = "as"))]
  pub(crate) output :String,
  pub(crate) pipes: Vec<Pipe>,
}

impl Transform {
  pub fn new(source: &str, output: &str, pipes: Vec<Pipe>) -> Transform {
    Transform {
      from: source.to_string(),
      output: output.to_string(),
      pipes,
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::transform::filter::FilterPipe;
  use crate::transform::group::{GroupOperator, GroupPipe};
  use crate::transform::map::MapPipe;
  use crate::transform::pipe::Pipe;
  use crate::transform::Transform;

  #[test]
  fn deserializes_transform() {
    let transform_json = r#"{
      "from": "primary",
      "as": "x",
      "pipes": [
        { "type": "filter", "fn": "a > 2" },
        { "type": "map", "fn": "a + 2", "output": "b" },
        { "type": "group", "by": "b", "op": "count", "output": "count" }
       ]
    }"#;

    let transform: Transform = serde_json::from_str(transform_json).unwrap();
    assert_eq!(transform.from, "primary");
    assert_eq!(transform.output, "x");
    assert_eq!(
      transform.pipes,
      vec![
        Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
        Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
        Pipe::Group(GroupPipe::new("b", GroupOperator::Count, "count"))
      ]
    );
  }
}
