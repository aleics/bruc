use ebooler::expr::{Expression, Interpretable};
use ebooler::PredicateParser;

use crate::data::DataValue;
use crate::error::Error;
use crate::pipe::{DataIterator, PipeIterator, Predicate};

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

pub struct FilterIterator<'a> {
  source: Box<DataIterator<'a>>,
  pipe: &'a FilterPipe<'a>,
}

impl<'a> FilterIterator<'a> {
  pub fn new(source: Box<DataIterator<'a>>, pipe: &'a FilterPipe<'a>) -> FilterIterator<'a> {
    FilterIterator { source, pipe }
  }

  #[inline]
  pub fn chain(source: PipeIterator<'a>, pipe: &'a FilterPipe<'a>) -> PipeIterator<'a> {
    let iterator = FilterIterator::new(Box::new(source), pipe);
    PipeIterator::new(Box::new(iterator))
  }
}

impl<'a> Iterator for FilterIterator<'a> {
  type Item = DataValue<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let current = self.source.next()?;
    self.pipe.apply(current).or_else(|| self.next())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::filter::{FilterIterator, FilterPipe};
  use crate::pipe::PipeIterator;

  #[test]
  fn apply() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = [
      DataValue::from_pairs(vec![("a", 2.0.into())]),
      DataValue::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = PipeIterator::source(&data);

    let iterator = FilterIterator::chain(source, &filter);
    let result = iterator.collect::<Vec<DataValue>>();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].find("a").unwrap(), &4.0.into());
  }
}
