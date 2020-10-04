use ebooler::expr::{Expression, Interpretable};
use ebooler::PredicateParser;

use crate::data::DataValue;
use crate::error::Error;
use crate::pipe::{DataIterator, PipeIterator, Predicate};

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

pub struct MapIterator<'a> {
  source: Box<DataIterator<'a>>,
  pipe: &'a MapPipe<'a>,
}

impl<'a> MapIterator<'a> {
  pub fn new(source: Box<DataIterator<'a>>, pipe: &'a MapPipe<'a>) -> MapIterator<'a> {
    MapIterator { source, pipe }
  }

  #[inline]
  pub fn chain(source: PipeIterator<'a>, pipe: &'a MapPipe<'a>) -> PipeIterator<'a> {
    let iterator = MapIterator::new(Box::new(source), pipe);
    PipeIterator::new(Box::new(iterator))
  }
}

impl<'a> Iterator for MapIterator<'a> {
  type Item = DataValue<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.source.next().map(|mut item| {
      self.pipe.apply(&mut item);
      item
    })
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

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::map::{MapIterator, MapPipe};
  use crate::pipe::PipeIterator;

  #[test]
  fn apply() {
    let map = MapPipe::new("a + 3", "b").unwrap();
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = PipeIterator::source(&data);

    let iterator = MapIterator::chain(source, &map);
    let result = iterator.collect::<Vec<DataValue>>();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].find("b").unwrap(), &5.0.into());
    assert_eq!(result[1].find("b").unwrap(), &7.0.into());
  }
}
