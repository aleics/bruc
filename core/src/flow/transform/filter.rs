use crate::data::DataValue;
use crate::transform::filter::FilterPipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct FilterNode<'a, S> {
  source: S,
  pipe: &'a FilterPipe<'a>,
}

impl<'a, S> FilterNode<'a, S> {
  pub fn new(source: S, pipe: &'a FilterPipe<'a>) -> FilterNode<'a, S> {
    FilterNode { source, pipe }
  }
}

impl<'a, S> Unpin for FilterNode<'a, S> {}

impl<'a, S> Stream for FilterNode<'a, S>
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
              let result = self.pipe.apply(value);
              if result.is_some() {
                break Some(result);
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

impl<'a, S> Clone for FilterNode<'a, S>
where
  S: Clone,
{
  fn clone(&self) -> Self {
    FilterNode {
      source: self.source.clone(),
      pipe: self.pipe,
    }
  }
}

#[cfg(test)]
mod tests {
  use futures::StreamExt;

  use crate::data::DataValue;
  use crate::flow::data::{Chunks, Source};
  use crate::flow::transform::filter::FilterNode;
  use crate::transform::filter::FilterPipe;

  #[test]
  fn applies() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = Source::new();
    let node = FilterNode::new(source.link(), &filter);

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
    let first = FilterNode::new(source.link(), &filter);
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
