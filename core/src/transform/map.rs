use crate::data::DataValue;
use crate::transform::error::Error;
use crate::transform::pipe::Predicate;
use expression::expr::{Expression, Interpretable};
use expression::PredicateParser;

#[derive(PartialEq, Debug)]
pub struct MapPipe<'a> {
  pub(crate) predicate: MapPredicate<'a>,
  pub(crate) output: &'a str,
}

impl<'a> MapPipe<'a> {
  #[inline]
  pub fn new(predicate: &'a str, output: &'a str) -> Result<MapPipe<'a>, Error> {
    let predicate = MapPredicate::new(predicate)?;
    Ok(MapPipe { predicate, output })
  }

  #[inline]
  pub fn apply(&self, item: &mut DataValue) {
    let var = self.predicate.interpret(&item).unwrap();
    item.insert(self.output, var.into());
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

#[cfg(feature = "serde")]
pub mod serde {
  use crate::transform::map::MapPipe;
  use serde::de::{MapAccess, Visitor};
  use serde::{de, Deserialize, Deserializer};
  use std::fmt;

  impl<'de: 'a, 'a> Deserialize<'de> for MapPipe<'a> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
      struct MapPipeVisitor;

      impl<'a> Visitor<'a> for MapPipeVisitor {
        type Value = MapPipe<'a>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
          formatter.write_str("struct MapPipe")
        }

        fn visit_map<A: MapAccess<'a>>(self, mut map: A) -> Result<Self::Value, A::Error> {
          let mut predicate = None;
          let mut output = None;

          while let Some((key, value)) = map.next_entry()? {
            match key {
              "fn" => {
                if predicate.is_some() {
                  return Err(de::Error::duplicate_field("fn"));
                }
                predicate = value;
              }
              "output" => {
                if output.is_some() {
                  return Err(de::Error::duplicate_field("output"));
                }
                output = value;
              }
              _ => {}
            }
          }

          let predicate = predicate.ok_or_else(|| de::Error::missing_field("fn"))?;
          let output = output.ok_or_else(|| de::Error::missing_field("output"))?;

          MapPipe::new(predicate, output).map_err(|err| de::Error::custom(err.to_string()))
        }
      }

      deserializer.deserialize_any(MapPipeVisitor)
    }
  }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
  use crate::transform::map::{MapPipe, MapPredicate};

  #[test]
  fn deserialize_map() {
    let map = serde_json::from_str::<MapPipe>(r#"{ "fn": "a + 2.0", "output": "b" }"#).unwrap();

    assert_eq!(map.predicate, MapPredicate::new("a + 2.0").unwrap());
    assert_eq!(map.output, "b");
  }
}
