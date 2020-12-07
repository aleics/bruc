use crate::data::DataValue;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub type DataStream<'a> = Box<dyn Stream<Item = DataValue<'a>> + Unpin + 'a>;

pub fn source(data: Vec<DataValue>) -> DataStream {
  Box::new(Source::new(data))
}

pub fn source_finite(data: Vec<DataValue>) -> DataStream {
  Box::new(SourceFinite::new(data.into_iter()))
}

pub struct SourceFinite<I> {
  source: I,
}

impl<'a, I> SourceFinite<I>
where
  I: Iterator<Item = DataValue<'a>>,
{
  pub fn new(source: I) -> SourceFinite<I> {
    SourceFinite { source }
  }
}

impl<I> Unpin for SourceFinite<I> {}

impl<'a, I> Stream for SourceFinite<I>
where
  I: Iterator<Item = DataValue<'a>>,
{
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(self.source.next())
  }
}

pub struct Source<'a> {
  source: Vec<DataValue<'a>>,
  index: usize,
}

impl<'a> Source<'a> {
  pub fn new(source: Vec<DataValue<'a>>) -> Source<'a> {
    Source { source, index: 0 }
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
      None => Poll::Pending,
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::flow::data::{Source, SourceFinite};
  use futures::StreamExt;

  #[test]
  fn sends_finite() {
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = SourceFinite::new(data.into_iter());

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
  fn sends() {
    let data = vec![
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let mut source = Source::new(data);

    futures::executor::block_on(async {
      assert_eq!(
        vec![source.next().await.unwrap(), source.next().await.unwrap()],
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
    let mut source = Source::new(data);

    source.extend(vec![
      DataValue::from_pairs(vec![("a", 6.0.into())]),
      DataValue::from_pairs(vec![("a", 8.0.into())]),
    ]);

    futures::executor::block_on(async {
      assert_eq!(
        vec![
          source.next().await.unwrap(),
          source.next().await.unwrap(),
          source.next().await.unwrap(),
          source.next().await.unwrap()
        ],
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
