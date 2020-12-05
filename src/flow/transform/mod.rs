use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::flow::transform::filter::FilterNode;
use crate::flow::transform::group::GroupNode;
use crate::flow::transform::map::MapNode;
use crate::transform::pipe::Pipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub mod filter;
pub mod group;
pub mod map;

#[inline]
pub fn chain<'a>(source: DataStream<'a>, pipes: &'a [Pipe<'a>]) -> TransformNode<'a> {
  pipes
    .iter()
    .fold(TransformNode::new(source), |mut acc, pipe| {
      acc = TransformNode::chain(acc, pipe);
      acc
    })
}

pub struct TransformNode<'a> {
  source: DataStream<'a>,
}

impl<'a> TransformNode<'a> {
  pub fn new(source: DataStream<'a>) -> TransformNode<'a> {
    TransformNode { source }
  }

  pub fn chain(source: TransformNode<'a>, pipe: &'a Pipe<'a>) -> TransformNode<'a> {
    match pipe {
      Pipe::Filter(pipe) => FilterNode::chain(source, pipe),
      Pipe::Map(pipe) => MapNode::chain(source, pipe),
      Pipe::Group(pipe) => GroupNode::chain(source, pipe),
    }
  }
}

impl<'a> Unpin for TransformNode<'a> {}

impl<'a> Stream for TransformNode<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source).poll_next(cx)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

  use crate::data::DataValue;
  use crate::flow::data::source_finite;
  use crate::flow::transform::chain;
  use crate::transform::filter::FilterPipe;
  use crate::transform::group::{GroupOperator, GroupPipe};
  use crate::transform::map::MapPipe;
  use crate::transform::pipe::Pipe;

  #[test]
  fn chain_empty() {
    let pipes: [Pipe; 0] = [];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let node = chain(source_finite(data), &pipes);

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;

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

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let node = chain(source_finite(data), &pipes);

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;

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

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let node = chain(source_finite(data), &pipes);

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;
      assert_eq!(values, vec![DataValue::from_pairs(vec![("a", 3.0.into())])]);
    });
  }

  #[test]
  fn chain_groups() {
    let pipes = [
      Pipe::Group(GroupPipe::new("a", GroupOperator::Count, "a_count")),
      Pipe::Group(GroupPipe::new(
        "a_count",
        GroupOperator::Count,
        "count_a_count",
      )),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let node = chain(source_finite(data), &pipes);

    futures::executor::block_on(async {
      let result = node.collect::<Vec<DataValue>>().await;

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

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let node = chain(source_finite(data), &pipes);

    futures::executor::block_on(async {
      let result = node.collect::<Vec<DataValue>>().await;
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
      Pipe::Group(GroupPipe::new("a", GroupOperator::Count, "a_count")),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let node = chain(source_finite(data), &pipes);

    futures::executor::block_on(async {
      let result = node.collect::<Vec<DataValue>>().await;
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
