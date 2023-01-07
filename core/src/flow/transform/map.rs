use crate::data::DataValue;
use crate::transform::map::MapPipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct MapNode<S> {
  source: S,
  pipe: MapPipe,
}

impl<S> MapNode<S> {
  pub fn new(source: S, pipe: MapPipe) -> MapNode<S> {
    MapNode { source, pipe }
  }
}

impl<S> Unpin for MapNode<S> {}

impl<S> Stream for MapNode<S>
where
  S: Stream<Item = Option<DataValue>> + Unpin,
{
  type Item = Option<DataValue>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source).poll_next(cx).map(|value| {
      value.map(|value| {
        value.map(|mut value| {
          self.pipe.apply(&mut value);
          value
        })
      })
    })
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

impl<S> Clone for MapNode<S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    MapNode {
      source: self.source.clone(),
      pipe: self.pipe.clone(),
    }
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

  use crate::data::DataValue;
  use crate::flow::data::{Chunks, Source};
  use crate::flow::transform::map::MapNode;
  use crate::transform::map::MapPipe;

  #[test]
  fn applies() {
    let map = MapPipe::new("a + 3", "b").unwrap();
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let node = MapNode::new(source.link(), map);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into()), ("b", 5.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 7.0.into())])
        ]
      );
    })
  }

  #[test]
  fn clones() {
    let map = MapPipe::new("a + 3", "b").unwrap();
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];

    let source = Source::new();
    let first = MapNode::new(source.link(), map);
    let second = first.clone();

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(first).collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into()), ("b", 5.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 7.0.into())])
        ]
      );

      let values: Vec<_> = Chunks::new(second).collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into()), ("b", 5.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 7.0.into())])
        ]
      );
    })
  }
}
