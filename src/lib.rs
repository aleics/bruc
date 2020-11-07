use crate::data::DataValue;
use crate::pipe::{chain, Pipe, PipeStream};

pub mod data;
pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

#[cfg(feature = "serde")]
pub mod serde;

#[derive(Debug)]
pub struct Source<'a> {
  data: Vec<DataValue<'a>>,
  pipes: Vec<Pipe<'a>>,
}

impl<'a> Source<'a> {
  pub fn new(data: Vec<DataValue<'a>>, pipes: Vec<Pipe<'a>>) -> Source<'a> {
    Source { data, pipes }
  }

  pub fn data(&self) -> &Vec<DataValue<'a>> {
    &self.data
  }

  pub fn pipes(&self) -> &Vec<Pipe<'a>> {
    &self.pipes
  }
}

pub fn run<'a>(source: &'a Source<'a>) -> PipeStream<'a> {
  chain(source.data(), source.pipes())
}
