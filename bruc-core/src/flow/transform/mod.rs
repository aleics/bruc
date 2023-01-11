use crate::data::DataValue;
use crate::flow::data::DataNode;
use crate::flow::transform::filter::FilterNode;
use crate::flow::transform::group::GroupNode;
use crate::flow::transform::map::MapNode;
use crate::transform::pipe::Pipe;
use crate::transform::Transform;
use futures::task::{Context, Poll};
use futures::{Stream, StreamExt};
use std::pin::Pin;

pub mod filter;
pub mod group;
pub mod map;

pub enum TransformNode<S> {
  Filter(FilterNode<S>),
  Map(MapNode<S>),
  Group(GroupNode<S>),
}

impl<S> TransformNode<S>
where
  S: Stream<Item = Option<DataValue>> + Unpin,
{
  pub fn new(source: S, pipe: Pipe) -> TransformNode<S> {
    match pipe {
      Pipe::Filter(pipe) => TransformNode::Filter(FilterNode::new(source, pipe)),
      Pipe::Map(pipe) => TransformNode::Map(MapNode::new(source, pipe)),
      Pipe::Group(pipe) => TransformNode::Group(GroupNode::new(source, pipe)),
    }
  }

  #[inline]
  pub fn node(source: S, transform: &Transform) -> DataNode
  where
    S: Stream<Item = Option<DataValue>> + Unpin + 'static,
  {
    transform
      .iter()
      .cloned()
      .fold(Box::new(source), |acc, pipe| {
        Box::new(TransformNode::new(acc, pipe))
      })
  }
}

impl<S> Unpin for TransformNode<S> {}

impl<S> Stream for TransformNode<S>
where
  S: Stream<Item = Option<DataValue>> + Unpin,
{
  type Item = Option<DataValue>;

  fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    match self.get_mut() {
      TransformNode::Filter(node) => node.poll_next_unpin(cx),
      TransformNode::Map(node) => node.poll_next_unpin(cx),
      TransformNode::Group(node) => node.poll_next_unpin(cx),
    }
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    match self {
      TransformNode::Filter(node) => node.size_hint(),
      TransformNode::Map(node) => node.size_hint(),
      TransformNode::Group(node) => node.size_hint(),
    }
  }
}

impl<S> Clone for TransformNode<S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    match self {
      TransformNode::Filter(node) => TransformNode::Filter(node.clone()),
      TransformNode::Map(node) => TransformNode::Map(node.clone()),
      TransformNode::Group(node) => TransformNode::Group(node.clone()),
    }
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

  use crate::data::DataValue;
  use crate::flow::data::{Chunks, Source};
  use crate::flow::transform::TransformNode;
  use crate::transform::filter::FilterPipe;
  use crate::transform::group::{GroupOperator, GroupPipe};
  use crate::transform::map::MapPipe;
  use crate::transform::pipe::Pipe;

  #[test]
  fn chain_empty() {
    let transform = vec![];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = TransformNode::node(source.link(), &transform);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 1.0.into())]),
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 3.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())])
        ]
      )
    });
  }

  #[test]
  fn chain_maps() {
    let transform = vec![
      Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
      Pipe::Map(MapPipe::new("a + 4", "c").unwrap()),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = TransformNode::node(source.link(), &transform);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

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
          ])
        ]
      )
    });
  }

  #[test]
  fn chain_filters() {
    let transform = vec![
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Filter(FilterPipe::new("a < 4").unwrap()),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = TransformNode::node(source.link(), &transform);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;
      assert_eq!(values, vec![DataValue::from_pairs(vec![("a", 3.0.into())])]);
    });
  }

  #[test]
  fn chain_groups() {
    let transform = vec![
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

    let source = Source::new();
    let node = TransformNode::node(source.link(), &transform);

    source.send(data);
    futures::executor::block_on(async {
      let values = Chunks::new(node).collect::<Vec<_>>().await;

      assert_eq!(values.len(), 2);
      assert!(values.contains(&DataValue::from_pairs(vec![
        ("a_count", 2.0.into()),
        ("count_a_count", 1.0.into())
      ])));
      assert!(values.contains(&DataValue::from_pairs(vec![
        ("a_count", 1.0.into()),
        ("count_a_count", 2.0.into())
      ])));
    });
  }

  #[test]
  fn chain_filter_map() {
    let transform = vec![
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Map(MapPipe::new("a * 2", "b").unwrap()),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = TransformNode::node(source.link(), &transform);

    source.send(data);
    futures::executor::block_on(async {
      let values = Chunks::new(node).collect::<Vec<_>>().await;
      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 3.0.into()), ("b", 6.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 8.0.into())])
        ]
      );
    });
  }

  #[test]
  fn chain_filter_group() {
    let transform = vec![
      Pipe::Filter(FilterPipe::new("a > 2").unwrap()),
      Pipe::Group(GroupPipe::new("a", GroupOperator::Count, "a_count")),
    ];

    let data = vec![
      DataValue::from_pairs(vec![("a", 1.0.into())]),
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 3.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = TransformNode::node(source.link(), &transform);

    source.send(data);
    futures::executor::block_on(async {
      let result = Chunks::new(node).collect::<Vec<_>>().await;
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
