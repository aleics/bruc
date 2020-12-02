use crate::data::DataValue;
use crate::flow::data::DataStream;
use crate::flow::transform::TransformStream;
use crate::transform::map::MapPipe;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub struct MapStream<'a> {
  source: DataStream<'a>,
  pipe: &'a MapPipe<'a>,
}

impl<'a> MapStream<'a> {
  pub fn new(source: DataStream<'a>, pipe: &'a MapPipe<'a>) -> MapStream<'a> {
    MapStream { source, pipe }
  }

  #[inline]
  pub fn chain(source: TransformStream<'a>, pipe: &'a MapPipe<'a>) -> TransformStream<'a> {
    let stream = MapStream::new(Box::new(source), pipe);
    TransformStream::new(Box::new(stream))
  }
}

impl<'a> Unpin for MapStream<'a> {}

impl<'a> Stream for MapStream<'a> {
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
  use crate::flow::transform::map::MapStream;
  use crate::flow::transform::TransformStream;
  use crate::transform::map::MapPipe;

  #[test]
  fn applies() {
    let map = MapPipe::new("a + 3", "b").unwrap();
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = TransformStream::source(&data);

    let stream = MapStream::chain(source, &map);

    futures::executor::block_on(async {
      let values: Vec<_> = stream.collect().await;

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
