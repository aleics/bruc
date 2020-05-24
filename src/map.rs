use std::fmt;

use ebooler::expr::{Expression, Interpretable};
use ebooler::vars::Variables;
use ebooler::PredicateParser;
use serde::de::Visitor;
use serde::{Deserialize, Deserializer};

use crate::error::Error;
use crate::pipe::{DataIterator, Predicate};

#[derive(Deserialize, PartialEq, Debug)]
pub struct MapPipe<'a> {
  #[serde(rename = "fn", borrow)]
  predicate: MapPredicate<'a>,
  #[serde(borrow)]
  output: &'a str,
}

impl<'a> MapPipe<'a> {
  #[inline]
  pub fn new(predicate: &'a str, output: &'a str) -> Result<MapPipe<'a>, Error> {
    let predicate = MapPredicate::new(predicate)?;
    Ok(MapPipe { predicate, output })
  }

  #[inline]
  pub fn apply(&self, item: &Variables<'a>) -> Variables<'a> {
    let var = self.predicate.interpret(item).unwrap();

    let mut result = item.clone();
    result.insert(self.output, var.into());

    result
  }
}

pub struct MapIterator<'a> {
  source: Box<DataIterator<'a>>,
  pipe: &'a MapPipe<'a>,
}

impl<'a> MapIterator<'a> {
  pub fn chain(source: Box<DataIterator<'a>>, pipe: &'a MapPipe<'a>) -> MapIterator<'a> {
    MapIterator { source, pipe }
  }
}

impl<'a> Iterator for MapIterator<'a> {
  type Item = Variables<'a>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.source.next().map(|current| self.pipe.apply(&current))
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

  fn interpret(&self, vars: &Variables) -> Result<Self::Value, Error> {
    self
      .expression
      .interpret(vars)
      .map_err(|error| error.into())
  }
}

impl<'de: 'a, 'a> Deserialize<'de> for MapPredicate<'a> {
  fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
    struct PredicateVisitor;

    impl<'a> Visitor<'a> for PredicateVisitor {
      type Value = MapPredicate<'a>;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("any valid predicate (string)")
      }

      #[inline]
      fn visit_borrowed_str<E: serde::de::Error>(self, value: &'a str) -> Result<Self::Value, E> {
        MapPredicate::new(value).map_err(|error| serde::de::Error::custom(error.to_string()))
      }
    }

    deserializer.deserialize_any(PredicateVisitor)
  }
}

#[cfg(test)]
mod tests {
  use ebooler::vars::Variables;

  use crate::map::{MapIterator, MapPipe};
  use crate::pipe::PipeIterator;

  #[test]
  fn apply() {
    let map = MapPipe::new("a + 3", "b").unwrap();
    let data = vec![
      Variables::from_pairs(vec![("a", 2.0.into())]),
      Variables::from_pairs(vec![("a", 4.0.into())]),
    ];
    let source = PipeIterator::source(data.iter());

    let iterator = MapIterator::chain(Box::new(source), &map);
    let result = iterator.collect::<Vec<Variables>>();

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].find("b").unwrap(), &5.0.into());
    assert_eq!(result[1].find("b").unwrap(), &7.0.into());
  }

  #[test]
  fn deserialize() {
    let map = serde_json::from_str::<MapPipe>(r#"{ "fn": "a + 2.0", "output": "b" }"#);
    assert!(map.is_ok());
  }
}
