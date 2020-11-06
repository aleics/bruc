use std::pin::Pin;

use futures::task::{Context, Poll};
use futures::Stream;

use crate::data::DataValue;
use crate::filter_async::FilterStream;
use crate::group_async::GroupStream;
use crate::map_async::MapStream;
use crate::pipe::Pipe;

#[inline]
pub async fn chain_async<'a>(data: &'a [DataValue<'a>], pipes: &'a [Pipe<'a>]) -> PipeStream<'a> {
  pipes
    .iter()
    .fold(PipeStream::source(data), |mut acc, pipe| {
      acc = PipeStream::chain(acc, pipe);
      acc
    })
}

pub type DataStream<'a> = Box<dyn Stream<Item = DataValue<'a>> + Unpin + 'a>;

pub struct PipeStream<'a> {
  source: DataStream<'a>,
}

impl<'a> PipeStream<'a> {
  pub fn new(source: DataStream<'a>) -> PipeStream<'a> {
    PipeStream { source }
  }

  pub fn chain(source: PipeStream<'a>, pipe: &'a Pipe<'a>) -> PipeStream<'a> {
    match pipe {
      Pipe::Filter(pipe) => FilterStream::chain(source, pipe),
      Pipe::Map(pipe) => MapStream::chain(source, pipe),
      Pipe::Group(pipe) => GroupStream::chain(source, pipe),
    }
  }

  pub fn source<I: 'a>(input: I) -> PipeStream<'a>
  where
    I: IntoIterator<Item = &'a DataValue<'a>>,
  {
    let stream = SourceStream::new(input.into_iter());
    PipeStream {
      source: Box::new(stream),
    }
  }
}

impl<'a> Unpin for PipeStream<'a> {}

impl<'a> Stream for PipeStream<'a> {
  type Item = DataValue<'a>;

  fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Pin::new(&mut self.source).poll_next(cx)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.source.size_hint()
  }
}

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
