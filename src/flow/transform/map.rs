use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::transform::map::MapPipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct MapNode<'a> {
  source: DataStream<'a>,
  pipe: &'a MapPipe<'a>,
}

impl<'a> MapNode<'a> {
  pub fn new(source: DataStream<'a>, pipe: &'a MapPipe<'a>) -> MapNode<'a> {
    MapNode { source, pipe }
  }

  #[inline]
  pub fn chain(source: DataStream<'a>, pipe: &'a MapPipe<'a>) -> DataStream<'a> {
    let node = MapNode::new(source, pipe);
    Box::new(node)
  }
}

impl<'a> Unpin for MapNode<'a> {}

impl<'a> Stream for MapNode<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        let result = source.map(|mut value| {
          self.pipe.apply(&mut value);
          value
        });
        break result;
      }
    })
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
  use crate::flow::transform::map::MapNode;
  use crate::transform::map::MapPipe;

  #[test]
  fn applies() {
    let map = MapPipe::new("a + 3", "b").unwrap();
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = source_finite(data);

    let node = MapNode::chain(source, &map);

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into()), ("b", 5.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into()), ("b", 7.0.into())]),
        ]
      )
    })
  }
}
