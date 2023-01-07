use crate::data::DataValue;
use crate::transform::error::Error;
use crate::transform::filter::FilterPipe;
use crate::transform::group::GroupPipe;
use crate::transform::map::MapPipe;

#[derive(PartialEq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub enum Pipe {
  Filter(FilterPipe),
  Map(MapPipe),
  Group(GroupPipe),
}

pub trait Predicate {
  type Value;

  fn interpret(&self, vars: &DataValue) -> Result<Self::Value, Error>;
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::transform::pipe::Pipe;

  #[test]
  fn deserializes_pipes() {
    let pipes_json = r#"[
        { "type": "filter", "fn": "a > 2" },
        { "type": "map", "fn": "a + 2", "output": "b" },
        { "type": "group", "by": "b", "op": "count", "output": "count" }
      ]"#;
    let pipes: Vec<Pipe> = serde_json::from_str(pipes_json).unwrap();

    assert_eq!(pipes.len(), 3);
  }
}
