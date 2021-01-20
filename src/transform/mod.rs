use crate::transform::pipe::Pipe;

pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Transform<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  from: &'a str,
  #[cfg_attr(feature = "serde", serde(borrow))]
  #[cfg_attr(feature = "serde", serde(rename = "as"))]
  output: &'a str,
  #[cfg_attr(feature = "serde", serde(borrow))]
  pipes: Vec<Pipe<'a>>,
}

impl<'a> Transform<'a> {
  pub fn new(source: &'a str, output: &'a str, pipes: Vec<Pipe<'a>>) -> Transform<'a> {
    Transform {
      from: source,
      output,
      pipes,
    }
  }

  pub fn from(&self) -> &str {
    self.from
  }

  pub fn output(&self) -> &str {
    self.output
  }

  pub fn pipes(&self) -> &Vec<Pipe<'a>> {
    &self.pipes
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
    assert_eq!(transform.from(), "primary");
    assert_eq!(transform.output(), "x");
    assert_eq!(
      transform.pipes(),
      &vec![
        Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
        Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
        Pipe::Group(GroupPipe::new("b", GroupOperator::Count, "count"))
      ]
    );
  }
}
