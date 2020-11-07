use std::pin::Pin;

use futures::task::{Context, Poll};
use futures::Stream;

use crate::data::DataValue;
use crate::error::Error;
use crate::filter::{FilterPipe, FilterStream};
use crate::group::{GroupPipe, GroupStream};
use crate::map::{MapPipe, MapStream};

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
pub fn chain<'a>(data: &'a [DataValue<'a>], pipes: &'a [Pipe<'a>]) -> PipeStream<'a> {
  pipes
    .iter()
    .fold(PipeStream::source(data), |mut acc, pipe| {
      acc = PipeStream::chain(acc, pipe);
      acc
    })
}

pub type DataStream<'a> = Box<dyn Stream<Item = DataValue<'a>> + Unpin + 'a>;

pub struct PipeStream<'a> {
  source: DataStream<'a>,
}

impl<'a> PipeStream<'a> {
  pub fn new(source: DataStream<'a>) -> PipeStream<'a> {
    PipeStream { source }
  }

  pub fn chain(source: PipeStream<'a>, pipe: &'a Pipe<'a>) -> PipeStream<'a> {
    match pipe {
      Pipe::Filter(pipe) => FilterStream::chain(source, pipe),
      Pipe::Map(pipe) => MapStream::chain(source, pipe),
      Pipe::Group(pipe) => GroupStream::chain(source, pipe),
    }
  }

  pub fn source<I: 'a>(input: I) -> PipeStream<'a>
  where
    I: IntoIterator<Item = &'a DataValue<'a>>,
  {
    let stream = SourceStream::new(input.into_iter());
    PipeStream {
      source: Box::new(stream),
    }
  }
}

impl<'a> Unpin for PipeStream<'a> {}

impl<'a> Stream for PipeStream<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source).poll_next(cx)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

pub struct SourceStream<I> {
  source: I,
}

impl<'a, I> SourceStream<I>
where
  I: Iterator<Item = &'a DataValue<'a>>,
{
  pub fn new(source: I) -> SourceStream<I> {
    SourceStream { source }
  }
}

impl<I> Unpin for SourceStream<I> {}

impl<'a, I> Stream for SourceStream<I>
where
  I: Iterator<Item = &'a DataValue<'a>>,
{
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(self.source.next().cloned())
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

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

    let stream = chain(&data, &pipes);

    futures::executor::block_on(async {
      let values: Vec<_> = stream.collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 1.0.into())]),
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 3.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())]),
        ]
      )
    });
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

    let stream = chain(&data, &pipes);

    futures::executor::block_on(async {
      let values: Vec<_> = stream.collect().await;

      assert_eq!(
        values,
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
      )
    });
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

    let stream = chain(&data, &pipes);

    futures::executor::block_on(async {
      let values: Vec<_> = stream.collect().await;
      assert_eq!(values, vec![DataValue::from_pairs(vec![("a", 3.0.into())])]);
    });
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

    let stream = chain(&data, &pipes);

    futures::executor::block_on(async {
      let result = stream.collect::<Vec<DataValue>>().await;

      assert_eq!(result.len(), 2);
      assert!(result.contains(&DataValue::from_pairs(vec![
        ("a_count", 2.0.into()),
        ("count_a_count", 1.0.into())
      ])));
      assert!(result.contains(&DataValue::from_pairs(vec![
        ("a_count", 1.0.into()),
        ("count_a_count", 2.0.into())
      ])));
    });
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

    let stream = chain(&data, &pipes);

    futures::executor::block_on(async {
      let result = stream.collect::<Vec<DataValue>>().await;
      assert_eq!(
        result,
        vec![
          DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 6.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 8.0.into())])
        ]
      );
    });
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

    let stream = chain(&data, &pipes);

    futures::executor::block_on(async {
      let result = stream.collect::<Vec<DataValue>>().await;
      assert_eq!(result.len(), 2);
      assert!(result.contains(&DataValue::from_pairs(vec![
        ("a", 3.0.into()),
        ("a_count", 1.0.into())
      ])));
      assert!(result.contains(&DataValue::from_pairs(vec![
        ("a", 4.0.into()),
        ("a_count", 1.0.into())
      ])));
    });
  }
}
