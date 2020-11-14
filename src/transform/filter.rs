use std::pin::Pin;

use ebooler::expr::{Expression, Interpretable};
use ebooler::PredicateParser;
use futures::task::{Context, Poll};
use futures::Stream;

use crate::transform::data::DataValue;
use crate::transform::error::Error;
use crate::transform::pipe::{DataStream, PipeStream, Predicate};

#[derive(PartialEq, Debug)]
pub struct FilterPipe<'a> {
  predicate: FilterPredicate<'a>,
}

impl<'a> FilterPipe<'a> {
  #[inline]
  pub fn new(predicate: &'a str) -> Result<FilterPipe<'a>, Error> {
    let predicate = FilterPredicate::new(predicate)?;
    Ok(FilterPipe { predicate })
  }

  #[inline]
  pub fn predicate(&self) -> &'_ FilterPredicate<'a> {
    &self.predicate
  }

  #[inline]
  pub fn apply(&self, item: DataValue<'a>) -> Option<DataValue<'a>> {
    let result = self.predicate.interpret(&item).unwrap();
    if result {
      Some(item)
    } else {
      None
    }
  }
}

#[derive(PartialEq, Debug)]
pub struct FilterPredicate<'a> {
  expression: Expression<'a>,
}

impl<'a> FilterPredicate<'a> {
  pub fn new(input: &'a str) -> Result<FilterPredicate<'a>, Error> {
    let expression = PredicateParser::new(input).parse()?;
    Ok(FilterPredicate { expression })
  }
}

impl<'a> Predicate for FilterPredicate<'a> {
  type Value = bool;

  fn interpret(&self, vars: &DataValue) -> Result<Self::Value, Error> {
    self
      .expression
      .interpret(vars)
      .map_err(|error| error.into())
  }
}

pub struct FilterStream<'a> {
  source: DataStream<'a>,
  pipe: &'a FilterPipe<'a>,
}

impl<'a> FilterStream<'a> {
  pub fn new(source: DataStream<'a>, pipe: &'a FilterPipe<'a>) -> FilterStream<'a> {
    FilterStream { source, pipe }
  }

  #[inline]
  pub fn chain(source: PipeStream<'a>, pipe: &'a FilterPipe<'a>) -> PipeStream<'a> {
    let stream = FilterStream::new(Box::new(source), pipe);
    PipeStream::new(Box::new(stream))
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

  use crate::transform::data::DataValue;
  use crate::transform::filter::FilterPipe;
  use crate::transform::filter::FilterStream;
  use crate::transform::pipe::PipeStream;

  #[test]
  fn applies() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = PipeStream::source(&data);
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
