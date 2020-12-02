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
  source: &'a str,
  #[cfg_attr(feature = "serde", serde(borrow))]
  pipes: Vec<Pipe<'a>>,
}

impl<'a> Transform<'a> {
  pub fn new(source: &'a str, pipes: Vec<Pipe<'a>>) -> Transform<'a> {
    Transform { source, pipes }
  }

  pub fn source(&self) -> &str {
    &self.source
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
      "source": "primary",
      "pipes": [
        { "type": "filter", "fn": "a > 2" },
        { "type": "map", "fn": "a + 2", "output": "b" },
        { "type": "group", "by": "b", "op": "count", "output": "count" }
       ]
    }"#;

    let transform: Transform = serde_json::from_str(transform_json).unwrap();
    assert_eq!(transform.source(), "primary");
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
