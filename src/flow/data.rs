use crate::data::DataValue;
use futures::task::{Context, Poll};
use futures::Stream;
use std::pin::Pin;

pub type DataStream<'a> = Box<dyn Stream<Item = DataValue<'a>> + Unpin + 'a>;

pub struct SourceStream<I> {
  source: I,
}

impl<'a, I> SourceStream<I>
where
  I: Iterator<Item = &'a DataValue<'a>>,
{
  pub fn new(source: I) -> SourceStream<I> {
    SourceStream { source }
  }
}

impl<I> Unpin for SourceStream<I> {}

impl<'a, I> Stream for SourceStream<I>
where
  I: Iterator<Item = &'a DataValue<'a>>,
{
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(self.source.next().cloned())
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}
