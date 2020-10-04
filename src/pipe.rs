use crate::data::DataValue;
use crate::error::Error;
use crate::filter::{FilterIterator, FilterPipe};
use crate::group::{GroupIterator, GroupPipe};
use crate::map::{MapIterator, MapPipe};

#[derive(PartialEq, Debug)]
pub enum Pipe<'a> {
  Filter(FilterPipe<'a>),
  Map(MapPipe<'a>),
  Group(GroupPipe<'a>),
}

pub trait Predicate {
  type Value;

  fn interpret(&self, vars: &DataValue) -> Result<Self::Value, Error>;
}

#[inline]
pub fn chain<'a>(data: &'a [DataValue<'a>], pipes: &'a [Pipe<'a>]) -> PipeIterator<'a> {
  pipes
    .iter()
    .fold(PipeIterator::source(data), |mut acc, pipe| {
      acc = PipeIterator::chain(acc, pipe);
      acc
    })
}

pub type DataIterator<'a> = dyn Iterator<Item = DataValue<'a>> + 'a;

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
    I: IntoIterator<Item = &'a DataValue<'a>>,
  {
    let iterator = SourceIterator::new(input.into_iter());
    PipeIterator {
      source: Box::new(iterator),
    }
  }
}

impl<'a> Iterator for PipeIterator<'a> {
  type Item = DataValue<'a>;

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
  I: Iterator<Item = &'a DataValue<'a>>,
{
  pub fn new(source: I) -> SourceIterator<I> {
    SourceIterator { source }
  }
}

impl<'a, I> Iterator for SourceIterator<I>
where
  I: Iterator<Item = &'a DataValue<'a>>,
{
  type Item = DataValue<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    self.source.next().cloned()
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::filter::FilterPipe;
  use crate::group::{GroupPipe, Operation};
  use crate::map::MapPipe;
  use crate::pipe::{chain, Pipe};

  #[test]
  fn chain_empty() {
    let pipes: [Pipe; 0] = [];

    let data = [
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<DataValue>>();
    assert_eq!(
      result,
      vec![
        DataValue::from_pairs(vec![("a", 1.0.into())]),
        DataValue::from_pairs(vec![("a", 2.0.into())]),
        DataValue::from_pairs(vec![("a", 3.0.into())]),
        DataValue::from_pairs(vec![("a", 4.0.into())]),
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
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<DataValue>>();
    assert_eq!(
      result,
      vec![
        DataValue::from_pairs(vec![
          ("a", 1.0.into()),
          ("b", 3.0.into()),
          ("c", 5.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("a", 2.0.into()),
          ("b", 4.0.into()),
          ("c", 6.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("a", 3.0.into()),
          ("b", 5.0.into()),
          ("c", 7.0.into())
        ]),
        DataValue::from_pairs(vec![
          ("a", 4.0.into()),
          ("b", 6.0.into()),
          ("c", 8.0.into())
        ]),
      ]
    );
  }

  #[test]
  fn chain_filters() {
    let pipes = [
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Filter(FilterPipe::new("a < 4").unwrap()),
    ];

    let data = [
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<DataValue>>();
    assert_eq!(result, vec![DataValue::from_pairs(vec![("a", 3.0.into())])]);
  }

  #[test]
  fn chain_groups() {
    let pipes = [
      Pipe::Group(GroupPipe::new("a", Operation::Count, "a_count")),
      Pipe::Group(GroupPipe::new("a_count", Operation::Count, "count_a_count")),
    ];

    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<DataValue>>();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&DataValue::from_pairs(vec![
      ("a_count", 2.0.into()),
      ("count_a_count", 1.0.into())
    ])));
    assert!(result.contains(&DataValue::from_pairs(vec![
      ("a_count", 1.0.into()),
      ("count_a_count", 2.0.into())
    ])));
  }

  #[test]
  fn chain_filter_map() {
    let pipes = [
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Map(MapPipe::new("a * 2", "b").unwrap()),
    ];

    let data = [
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<DataValue>>();
    assert_eq!(
      result,
      vec![
        DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 6.0.into())]),
        DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 8.0.into())])
      ]
    );
  }

  #[test]
  fn chain_filter_group() {
    let pipes = [
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Group(GroupPipe::new("a", Operation::Count, "a_count")),
    ];

    let data = [
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let iterator = chain(&data, &pipes);

    let result = iterator.collect::<Vec<DataValue>>();
    assert_eq!(result.len(), 2);
    assert!(result.contains(&DataValue::from_pairs(vec![
      ("a", 3.0.into()),
      ("a_count", 1.0.into())
    ])));
    assert!(result.contains(&DataValue::from_pairs(vec![
      ("a", 4.0.into()),
      ("a_count", 1.0.into())
    ])));
  }
}
