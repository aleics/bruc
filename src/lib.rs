use ebooler::vars::Variables;
use serde::Deserialize;

use crate::pipe::{chain, Pipe, PipeIterator};

pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

#[derive(Deserialize, Debug)]
pub struct Data<'a> {
  #[serde(borrow)]
  pub values: Vec<Variables<'a>>,
  pub pipes: Vec<Pipe<'a>>,
}

pub struct Engine<'a> {
  values: &'a [Variables<'a>],
}

impl<'a> Engine<'a> {
  pub fn new(values: &'a [Variables<'a>]) -> Engine<'a> {
    Engine { values }
  }

  pub fn run(&self, pipes: &'a [Pipe<'a>]) -> PipeIterator<'a> {
    chain(self.values, pipes)
  }
}

#[cfg(test)]
mod tests {
  use ebooler::vars::Variables;

  use crate::{Data, Engine};

  #[test]
  fn from_json() {
    let data = serde_json::from_str::<Data>(
      r#"
      {
        "values": [{ "a": 2, "b": 3 }],
        "pipes": [{ "filter": "a > 2" }]
      }
    "#,
    );

    assert!(data.is_ok());
  }

  #[test]
  fn apply_filter_pipe() {
    let data: Data = serde_json::from_str(
      r#"
      {
        "values": [{ "a": 2 }, { "a": 3 }],
        "pipes": [{ "filter": "a > 2" }]
      }
    "#,
    )
    .unwrap();

    let engine = Engine::new(&data.values);
    let result = engine.run(&data.pipes).collect::<Vec<Variables>>();

    assert_eq!(result.len(), 1);
  }
}
