use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::flow::transform::TransformNode;
use crate::transform::filter::FilterPipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct FilterNode<'a> {
  source: DataStream<'a>,
  pipe: &'a FilterPipe<'a>,
}

impl<'a> FilterNode<'a> {
  pub fn new(source: DataStream<'a>, pipe: &'a FilterPipe<'a>) -> FilterNode<'a> {
    FilterNode { source, pipe }
  }

  #[inline]
  pub fn chain(source: TransformNode<'a>, pipe: &'a FilterPipe<'a>) -> TransformNode<'a> {
    let node = FilterNode::new(Box::new(source), pipe);
    TransformNode::new(Box::new(node))
  }
}

impl<'a> Unpin for FilterNode<'a> {}

impl<'a> Stream for FilterNode<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(loop {
      if let Poll::Ready(source) = Pin::new(&mut self.source).poll_next(cx) {
        match source {
          Some(item) => {
            let result = self.pipe.apply(item);
            if result.is_some() {
              break result;
            }
          }
          None => break None,
        }
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
  use crate::flow::transform::filter::FilterNode;
  use crate::flow::transform::TransformNode;
  use crate::transform::filter::FilterPipe;

  #[test]
  fn applies() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = source_finite(data);
    let node = FilterNode::chain(TransformNode::new(source), &filter);

    futures::executor::block_on(async {
      let values: Vec<_> = node.collect().await;

      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![("a", 4.0.into())]),]
      )
    })
  }
}
