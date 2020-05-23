use ebooler::vars::Variables;
use serde::Deserialize;

use crate::pipe::{Pipable, Pipe};

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
  values: Vec<Variables<'a>>,
}

impl<'a> Engine<'a> {
  pub fn new(values: Vec<Variables<'a>>) -> Engine<'a> {
    Engine { values }
  }

  pub fn values(&self) -> &Vec<Variables<'a>> {
    &self.values
  }

  #[inline]
  pub fn run(&mut self, pipes: &'a [Pipe<'a>]) {
    for pipe in pipes {
      self.values = self.apply_pipe(pipe);
    }
  }

  #[inline]
  fn apply_pipe(&mut self, pipe: &'a Pipe<'a>) -> Vec<Variables<'a>> {
    match pipe {
      Pipe::Filter(filter) => filter.transform(&self.values),
      Pipe::Map(map) => map.transform(&self.values),
      Pipe::Group(group) => group.transform(&self.values),
    }
  }
}

#[cfg(test)]
mod tests {
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

    let engine = &mut Engine::new(data.values);
    engine.run(&data.pipes);

    assert_eq!(engine.values().len(), 1);
  }
}
