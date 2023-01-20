use bruc_expression::expr::{Expression, Interpretable};
use bruc_expression::PredicateParser;

use crate::data::DataValue;
use crate::spec::transform::error::Error;
use crate::spec::transform::pipe::Predicate;

#[derive(PartialEq, Debug, Clone)]
pub struct FilterPipe {
  pub(crate) predicate: FilterPredicate,
}

impl FilterPipe {
  #[inline]
  pub fn new(predicate: &str) -> Result<FilterPipe, Error> {
    let predicate = FilterPredicate::new(predicate)?;
    Ok(FilterPipe { predicate })
  }

  #[inline]
  pub fn apply(&self, item: &DataValue) -> bool {
    self.predicate.interpret(item).unwrap()
  }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FilterPredicate {
  expression: Expression,
}

impl FilterPredicate {
  pub fn new(input: &str) -> Result<FilterPredicate, Error> {
    let expression = PredicateParser::new(input).parse()?;
    Ok(FilterPredicate { expression })
  }
}

impl Predicate for FilterPredicate {
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
  use crate::spec::transform::filter::FilterPipe;
  use serde::de::{MapAccess, Visitor};
  use serde::{de, Deserialize, Deserializer};
  use std::fmt;

  impl<'de> Deserialize<'de> for FilterPipe {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
      struct FilterPipeVisitor;

      impl<'a> Visitor<'a> for FilterPipeVisitor {
        type Value = FilterPipe;

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
  use crate::spec::transform::filter::{FilterPipe, FilterPredicate};

  #[test]
  fn deserialize_filter() {
    let filter = serde_json::from_str::<FilterPipe>(r#"{ "fn": "a > 2.0" }"#).unwrap();
    assert_eq!(filter.predicate, FilterPredicate::new("a > 2.0").unwrap());
  }
}
