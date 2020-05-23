use ebooler::vars::Variables;
use serde::Deserialize;

use crate::error::Error;
use crate::filter::FilterPipe;
use crate::group::GroupPipe;
use crate::map::MapPipe;

#[derive(Deserialize, PartialEq, Debug)]
pub enum Pipe<'a> {
  #[serde(rename = "filter", borrow)]
  Filter(FilterPipe<'a>),
  #[serde(rename = "map", borrow)]
  Map(MapPipe<'a>),
  #[serde(rename = "group", borrow)]
  Group(GroupPipe<'a>),
}

pub trait Pipable<'a> {
  fn transform(&self, data: &[Variables<'a>]) -> Vec<Variables<'a>>;
}

pub trait Predicate {
  type Value;

  fn interpret(&self, vars: &Variables) -> Result<Self::Value, Error>;
}

#[cfg(test)]
mod tests {
  use crate::pipe::Pipe;

  #[test]
  fn deserializes_pipes() {
    let pipes_json = r#"[
      { "filter": "a > 2" },
      { "map": { "fn": "a + 2", "output": "b" } },
      { "group": { "by": "b", "op": "count", "output": "count" } }
    ]"#;
    let pipes: Vec<Pipe> = serde_json::from_str(pipes_json).unwrap();

    assert_eq!(pipes.len(), 3);
  }
}
