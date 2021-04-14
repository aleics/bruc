use expression::expr::{Expression, Interpretable};
use expression::PredicateParser;

use crate::data::DataValue;
use crate::transform::error::Error;
use crate::transform::pipe::Predicate;

#[derive(PartialEq, Debug)]
pub struct FilterPipe<'a> {
  pub(crate) predicate: FilterPredicate<'a>,
}

impl<'a> FilterPipe<'a> {
  #[inline]
  pub fn new(predicate: &'a str) -> Result<FilterPipe<'a>, Error> {
    let predicate = FilterPredicate::new(predicate)?;
    Ok(FilterPipe { predicate })
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

#[cfg(feature = "serde")]
pub mod serde {
  use crate::transform::filter::FilterPipe;
  use serde::de::{MapAccess, Visitor};
  use serde::{de, Deserialize, Deserializer};
  use std::fmt;

  impl<'de: 'a, 'a> Deserialize<'de> for FilterPipe<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
      struct FilterPipeVisitor;

      impl<'a> Visitor<'a> for FilterPipeVisitor {
        type Value = FilterPipe<'a>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          formatter.write_str("any valid predicate (string)")
        }

        fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
          let mut predicate = None;

          while let Some((key, value)) = map.next_entry::<&str, &str>()? {
            if key == "fn" && predicate.is_some() {
              return Err(de::Error::duplicate_field("fn"));
            }
            predicate = Some(value);
          }

          let predicate = predicate.ok_or_else(|| de::Error::missing_field("fn"))?;
          FilterPipe::new(predicate).map_err(|err| de::Error::custom(err.to_string()))
        }
      }

      deserializer.deserialize_any(FilterPipeVisitor)
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::transform::filter::{FilterPipe, FilterPredicate};

  #[test]
  fn deserialize_filter() {
    let filter = serde_json::from_str::<FilterPipe>(r#"{ "fn": "a > 2.0" }"#).unwrap();
    assert_eq!(filter.predicate, FilterPredicate::new("a > 2.0").unwrap());
  }
}
