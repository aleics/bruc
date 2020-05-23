use std::fmt;

use ebooler::expr::{Expression, Interpretable};
use ebooler::vars::Variables;
use ebooler::PredicateParser;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

use crate::error::Error;
use crate::pipe::{Pipable, Predicate};

#[derive(PartialEq, Debug)]
pub struct FilterPipe<'a> {
  predicate: FilterPredicate<'a>,
}

impl<'de: 'a, 'a> Deserialize<'de> for FilterPipe<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct FilterPipeVisitor;

    impl<'a> Visitor<'a> for FilterPipeVisitor {
      type Value = FilterPipe<'a>;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid predicate (string)")
      }

      #[inline]
      fn visit_borrowed_str<E: serde::de::Error>(self, value: &'a str) -> Result<Self::Value, E> {
        FilterPipe::new(value).map_err(|error| serde::de::Error::custom(error.to_string()))
      }
    }

    deserializer.deserialize_any(FilterPipeVisitor)
  }
}

impl<'a> FilterPipe<'a> {
  #[inline]
  pub fn new(predicate: &'a str) -> Result<FilterPipe<'a>, Error> {
    let predicate = FilterPredicate::new(predicate)?;
    Ok(FilterPipe { predicate })
  }

  #[inline]
  pub fn apply(&self, item: &Variables<'a>) -> Option<Variables<'a>> {
    let result = self.predicate.interpret(item).unwrap();
    if result {
      Some(item.clone())
    } else {
      None
    }
  }
}

impl<'a> Pipable<'a> for FilterPipe<'a> {
  #[inline]
  fn transform(&self, data: &[Variables<'a>]) -> Vec<Variables<'a>> {
    data
      .iter()
      .filter_map(|item| self.apply(item))
      .collect::<Vec<Variables>>()
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

  fn interpret(&self, vars: &Variables) -> Result<Self::Value, Error> {
    self
      .expression
      .interpret(vars)
      .map_err(|error| error.into())
  }
}

#[cfg(test)]
mod tests {
  use ebooler::vars::Variables;

  use crate::filter::FilterPipe;
  use crate::pipe::Pipable;

  #[test]
  fn apply() {
    let filter = FilterPipe::new("a > 3").unwrap();
    let data = vec![
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("a", 4.0.into())]),
    ];
    let result = filter.transform(&data);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].find("a").unwrap(), &4.0.into());
  }

  #[test]
  fn deserialize() {
    let filter = serde_json::from_str::<FilterPipe>(r#""a > 2.0""#);
    assert!(filter.is_ok());
  }
}
