use std::pin::Pin;

use ebooler::expr::{Expression, Interpretable};
use ebooler::PredicateParser;
use futures::task::{Context, Poll};
use futures::Stream;

use crate::data::DataValue;
use crate::error::Error;
use crate::pipe::{DataStream, PipeStream, Predicate};

#[derive(PartialEq, Debug)]
pub struct MapPipe<'a> {
  predicate: MapPredicate<'a>,
  output: &'a str,
}

impl<'a> MapPipe<'a> {
  #[inline]
  pub fn new(predicate: &'a str, output: &'a str) -> Result<MapPipe<'a>, Error> {
    let predicate = MapPredicate::new(predicate)?;
    Ok(MapPipe { predicate, output })
  }

  #[inline]
  pub fn apply(&self, item: &mut DataValue<'a>) {
    let var = self.predicate.interpret(&item).unwrap();
    item.insert(self.output, var.into());
  }

  #[inline]
  pub fn predicate(&self) -> &'_ MapPredicate<'a> {
    &self.predicate
  }

  #[inline]
  pub fn output(&self) -> &'_ str {
    &self.output
  }
}

#[derive(PartialEq, Debug)]
pub struct MapPredicate<'a> {
  expression: Expression<'a>,
}

impl<'a> MapPredicate<'a> {
  pub fn new(input: &'a str) -> Result<MapPredicate<'a>, Error> {
    let expression = PredicateParser::new(input).parse()?;
    Ok(MapPredicate { expression })
  }
}

impl<'a> Predicate for MapPredicate<'a> {
  type Value = f32;

  fn interpret(&self, vars: &DataValue) -> Result<Self::Value, Error> {
    self
      .expression
      .interpret(vars)
      .map_err(|error| error.into())
  }
}

pub struct MapStream<'a> {
  source: DataStream<'a>,
  pipe: &'a MapPipe<'a>,
}

impl<'a> MapStream<'a> {
  pub fn new(source: DataStream<'a>, pipe: &'a MapPipe<'a>) -> MapStream<'a> {
    MapStream { source, pipe }
  }

  #[inline]
  pub fn chain(source: PipeStream<'a>, pipe: &'a MapPipe<'a>) -> PipeStream<'a> {
    let stream = MapStream::new(Box::new(source), pipe);
    PipeStream::new(Box::new(stream))
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
  use crate::map::MapPipe;
  use crate::map::MapStream;
  use crate::pipe::PipeStream;

  #[test]
  fn applies() {
    let map = MapPipe::new("a + 3", "b").unwrap();
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = PipeStream::source(&data);

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
