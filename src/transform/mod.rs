use crate::data::DataValue;
use crate::transform::pipe::{chain, Pipe, PipeStream};

pub mod error;
pub mod filter;
pub mod group;
pub mod map;
pub mod pipe;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Source<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  data: Vec<DataValue<'a>>,
  #[cfg_attr(feature = "serde", serde(borrow))]
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
