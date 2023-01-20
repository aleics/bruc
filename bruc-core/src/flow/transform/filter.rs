use crate::data::DataValue;
use crate::spec::transform::filter::FilterPipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct FilterNode<S> {
  source: S,
  pipe: FilterPipe,
}

impl<S> FilterNode<S> {
  pub fn new(source: S, pipe: FilterPipe) -> FilterNode<S> {
    FilterNode { source, pipe }
  }
}

impl<S> Unpin for FilterNode<S> {}

impl<S> Stream for FilterNode<S>
where
  S: Stream<Item = Option<DataValue>> + Unpin,
{
  type Item = Option<DataValue>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        match source {
          Some(item) => match item {
            Some(value) => {
              if self.pipe.apply(&value) {
                break Some(Some(value));
              }
            }
            None => break Some(None),
          },
          None => break None,
        }
      }
    })
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

impl<S> Clone for FilterNode<S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    FilterNode {
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
  use crate::flow::transform::filter::FilterNode;
  use crate::spec::transform::filter::FilterPipe;

  #[test]
  fn applies() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = Source::new();
    let node = FilterNode::new(source.link(), filter);

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(node).collect().await;

      assert_eq!(values, vec![DataValue::from_pairs(vec![("a", 4.0.into())])])
    });
  }

  #[test]
  fn clones() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = Source::new();
    let first = FilterNode::new(source.link(), filter);
    let second = first.clone();

    source.send(data);
    futures::executor::block_on(async {
      let values: Vec<_> = Chunks::new(first).collect().await;
      assert_eq!(values, vec![DataValue::from_pairs(vec![("a", 4.0.into())])]);

      let values: Vec<_> = Chunks::new(second).collect().await;
      assert_eq!(values, vec![DataValue::from_pairs(vec![("a", 4.0.into())])]);
    });
  }
}
