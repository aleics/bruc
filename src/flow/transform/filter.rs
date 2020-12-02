use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::flow::transform::TransformStream;
use crate::transform::filter::FilterPipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct FilterStream<'a> {
  source: DataStream<'a>,
  pipe: &'a FilterPipe<'a>,
}

impl<'a> FilterStream<'a> {
  pub fn new(source: DataStream<'a>, pipe: &'a FilterPipe<'a>) -> FilterStream<'a> {
    FilterStream { source, pipe }
  }

  #[inline]
  pub fn chain(source: TransformStream<'a>, pipe: &'a FilterPipe<'a>) -> TransformStream<'a> {
    let stream = FilterStream::new(Box::new(source), pipe);
    TransformStream::new(Box::new(stream))
  }
}

impl<'a> Unpin for FilterStream<'a> {}

impl<'a> Stream for FilterStream<'a> {
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
  use crate::flow::transform::filter::FilterStream;
  use crate::flow::transform::TransformStream;
  use crate::transform::filter::FilterPipe;

  #[test]
  fn applies() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = TransformStream::source(&data);
    let stream = FilterStream::chain(source, &filter);

    futures::executor::block_on(async {
      let values: Vec<_> = stream.collect().await;

      assert_eq!(
        values,
        vec![DataValue::from_pairs(vec![("a", 4.0.into())]),]
      )
    })
  }
}
