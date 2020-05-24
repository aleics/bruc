use std::slice::Iter;

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
    .fold(PipeIterator::source(data.iter()), |mut acc, pipe| {
      acc = PipeIterator::chain(acc, pipe);
      acc
    })
}

pub type DataIterator<'a> = dyn Iterator<Item = Variables<'a>> + 'a;

pub struct PipeIterator<'a> {
  source: Box<DataIterator<'a>>,
}

impl<'a> PipeIterator<'a> {
  #[inline]
  pub fn chain(source: PipeIterator<'a>, pipe: &'a Pipe<'a>) -> PipeIterator<'a> {
    match pipe {
      Pipe::Filter(pipe) => PipeIterator::filter(source, pipe),
      Pipe::Map(pipe) => PipeIterator::map(source, pipe),
      Pipe::Group(pipe) => PipeIterator::group(source, pipe),
    }
  }

  #[inline]
  pub fn source(input: Iter<'a, Variables<'a>>) -> PipeIterator<'a> {
    let iterator = SourceIterator::new(input);
    PipeIterator {
      source: Box::new(iterator),
    }
  }

  #[inline]
  fn map(source: PipeIterator<'a>, pipe: &'a MapPipe<'a>) -> PipeIterator<'a> {
    let iterator = MapIterator::chain(Box::new(source), pipe);
    PipeIterator {
      source: Box::new(iterator),
    }
  }

  #[inline]
  fn filter(source: PipeIterator<'a>, pipe: &'a FilterPipe<'a>) -> PipeIterator<'a> {
    let iterator = FilterIterator::chain(Box::new(source), pipe);
    PipeIterator {
      source: Box::new(iterator),
    }
  }

  #[inline]
  fn group(source: PipeIterator<'a>, pipe: &'a GroupPipe<'a>) -> PipeIterator<'a> {
    let iterator = GroupIterator::chain(Box::new(source), pipe);
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

pub struct SourceIterator<'a> {
  source: Iter<'a, Variables<'a>>,
}

impl<'a> SourceIterator<'a> {
  pub fn new(source: Iter<'a, Variables<'a>>) -> SourceIterator<'a> {
    SourceIterator { source }
  }
}

impl<'a> Iterator for SourceIterator<'a> {
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
