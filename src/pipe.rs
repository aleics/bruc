use ebooler::vars::Variables;
use serde::Deserialize;

use crate::error::Error;
use crate::filter::{FilterIterator, FilterPipe};
use crate::group::{GroupIterator, GroupPipe};
use crate::map::{MapIterator, MapPipe};

#[derive(Deserialize, PartialEq, Debug)]
pub enum Pipe<'a> {
  #[serde(rename = "filter", borrow)]
  Filter(FilterPipe<'a>),
  #[serde(rename = "map", borrow)]
  Map(MapPipe<'a>),
  #[serde(rename = "group", borrow)]
  Group(GroupPipe<'a>),
}

pub trait Predicate {
  type Value;

  fn interpret(&self, vars: &Variables) -> Result<Self::Value, Error>;
}

#[inline]
pub fn chain<'a>(data: &'a [Variables<'a>], pipes: &'a [Pipe<'a>]) -> PipeIterator<'a> {
  pipes
    .iter()
    .fold(PipeIterator::source(data), |mut acc, pipe| {
      acc = PipeIterator::chain(acc, pipe);
      acc
    })
}

pub type DataIterator<'a> = dyn Iterator<Item = Variables<'a>> + 'a;

pub struct PipeIterator<'a> {
  source: Box<DataIterator<'a>>,
}

impl<'a> PipeIterator<'a> {
  pub fn new(source: Box<DataIterator<'a>>) -> PipeIterator<'a> {
    PipeIterator { source }
  }

  #[inline]
  pub fn chain(source: PipeIterator<'a>, pipe: &'a Pipe<'a>) -> PipeIterator<'a> {
    match pipe {
      Pipe::Filter(pipe) => FilterIterator::chain(source, pipe),
      Pipe::Map(pipe) => MapIterator::chain(source, pipe),
      Pipe::Group(pipe) => GroupIterator::chain(source, pipe),
    }
  }

  #[inline]
  pub fn source<I: 'a>(input: I) -> PipeIterator<'a>
  where
    I: IntoIterator<Item = &'a Variables<'a>>,
  {
    let iterator = SourceIterator::new(input.into_iter());
    PipeIterator {
      source: Box::new(iterator),
    }
  }
}

impl<'a> Iterator for PipeIterator<'a> {
  type Item = Variables<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.source.next()
  }
}

pub struct SourceIterator<I> {
  source: I,
}

impl<'a, I> SourceIterator<I>
where
  I: Iterator<Item = &'a Variables<'a>>,
{
  pub fn new(source: I) -> SourceIterator<I> {
    SourceIterator { source }
  }
}

impl<'a, I> Iterator for SourceIterator<I>
where
  I: Iterator<Item = &'a Variables<'a>>,
{
  type Item = Variables<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.source.next().cloned()
  }
}

#[cfg(test)]
mod tests {
  use ebooler::vars::Variables;

  use crate::map::MapPipe;
  use crate::pipe::{chain, Pipe};

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

  #[test]
  fn chain_empty() {
    let pipes: [Pipe; 0] = [];

    let data = [
      Variables::from_pairs(vec![("a", 1.0.into())]),
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("a", 3.0.into())]),
      Variables::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<Variables>>();
    assert_eq!(
      result,
      vec![
        Variables::from_pairs(vec![("a", 1.0.into())]),
        Variables::from_pairs(vec![("a", 2.0.into())]),
        Variables::from_pairs(vec![("a", 3.0.into())]),
        Variables::from_pairs(vec![("a", 4.0.into())]),
      ]
    );
  }

  #[test]
  fn chain_maps() {
    let pipes = [
      Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
      Pipe::Map(MapPipe::new("a + 4", "c").unwrap()),
    ];

    let data = [
      Variables::from_pairs(vec![("a", 1.0.into())]),
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("a", 3.0.into())]),
      Variables::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<Variables>>();
    assert_eq!(
      result,
      vec![
        Variables::from_pairs(vec![
          ("a", 1.0.into()),
          ("b", 3.0.into()),
          ("c", 5.0.into())
        ]),
        Variables::from_pairs(vec![
          ("a", 2.0.into()),
          ("b", 4.0.into()),
          ("c", 6.0.into())
        ]),
        Variables::from_pairs(vec![
          ("a", 3.0.into()),
          ("b", 5.0.into()),
          ("c", 7.0.into())
        ]),
        Variables::from_pairs(vec![
          ("a", 4.0.into()),
          ("b", 6.0.into()),
          ("c", 8.0.into())
        ]),
      ]
    );
  }
}
