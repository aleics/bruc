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

pub fn run<'a>(data: &'a Data<'a>) -> PipeIterator<'a> {
  chain(&data.values, &data.pipes)
}

#[cfg(test)]
mod tests {
  use crate::Data;

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
}
