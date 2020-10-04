use serde::Deserialize;

use crate::data::DataValue;
use crate::pipe::{chain, Pipe, PipeIterator};

pub mod data;
pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

#[derive(Deserialize, Debug)]
pub struct Source<'a> {
  #[serde(borrow)]
  pub data: Vec<DataValue<'a>>,
  pub pipes: Vec<Pipe<'a>>,
}

pub fn run<'a>(source: &'a Source<'a>) -> PipeIterator {
  chain(&source.data, &source.pipes)
}
