use crate::data::DataValue;
use crate::pipe::{chain, Pipe, PipeIterator};
use crate::pipe_async::{chain_async, PipeStream};

pub mod data;
pub mod error;
pub mod filter;
pub mod filter_async;
pub mod group;
pub mod group_async;
pub mod map;
pub mod map_async;
pub mod pipe;
pub mod pipe_async;

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

pub fn run<'a>(source: &'a Source<'a>) -> PipeIterator {
  chain(&source.data, &source.pipes)
}

pub fn run_async<'a>(source: &'a Source<'a>) -> PipeStream<'a> {
  chain_async(source.data(), &source.pipes())
}
