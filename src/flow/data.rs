use crate::data::DataValue;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub type DataStream<'a> = Box<dyn Stream<Item = DataValue<'a>> + Unpin + 'a>;

pub fn source(data: Vec<DataValue>) -> DataStream {
  Box::new(Source::new(data))
}

pub fn source_finite(data: Vec<DataValue>) -> DataStream {
  Box::new(Source::finite(data))
}

pub struct Source<'a> {
  source: Vec<DataValue<'a>>,
  index: usize,
  terminate: bool,
}

impl<'a> Source<'a> {
  pub fn new(source: Vec<DataValue<'a>>) -> Source<'a> {
    Source {
      source,
      index: 0,
      terminate: false,
    }
  }

  pub fn finite(source: Vec<DataValue<'a>>) -> Source<'a> {
    Source {
      source,
      index: 0,
      terminate: true,
    }
  }
}

impl<'a> Extend<DataValue<'a>> for Source<'a> {
  fn extend<T: IntoIterator<Item = DataValue<'a>>>(&mut self, iter: T) {
    self.source.extend(iter);
  }
}

impl<'a> Unpin for Source<'a> {}

impl<'a> Stream for Source<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    match self.source.get(self.index).cloned() {
      Some(value) => {
        self.index += 1;
        Poll::Ready(Some(value))
      }
      None if !self.terminate => Poll::Pending,
      _ => Poll::Ready(None),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::flow::data::Source;
  use futures::StreamExt;

  #[test]
  fn sends() {
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = Source::finite(data);

    futures::executor::block_on(async {
      let values: Vec<_> = source.collect().await;
      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())]),
        ]
      )
    })
  }

  #[test]
  fn appends() {
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let mut source = Source::finite(data);

    source.extend(vec![
      DataValue::from_pairs(vec![("a", 6.0.into())]),
      DataValue::from_pairs(vec![("a", 8.0.into())]),
    ]);

    futures::executor::block_on(async {
      let values: Vec<_> = source.collect().await;

      assert_eq!(
        values,
        vec![
          DataValue::from_pairs(vec![("a", 2.0.into())]),
          DataValue::from_pairs(vec![("a", 4.0.into())]),
          DataValue::from_pairs(vec![("a", 6.0.into())]),
          DataValue::from_pairs(vec![("a", 8.0.into())]),
        ]
      )
    });
  }
}
