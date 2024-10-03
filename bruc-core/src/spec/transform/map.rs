use crate::data::DataValue;
use crate::spec::transform::error::Error;
use crate::spec::transform::pipe::Predicate;
use bruc_expression::expr::{Expression, Interpretable};
use bruc_expression::PredicateParser;

#[derive(PartialEq, Debug, Clone)]
pub struct MapPipe {
    pub(crate) predicate: MapPredicate,
    pub(crate) output: String,
}

impl MapPipe {
    #[inline]
    pub fn new(predicate: &str, output: &str) -> Result<MapPipe, Error> {
        let predicate = MapPredicate::new(predicate)?;
        Ok(MapPipe {
            predicate,
            output: output.to_string(),
        })
    }

    #[inline]
    pub fn apply(&self, item: &mut DataValue) {
        let var = self.predicate.interpret(item).unwrap();
        item.insert(&self.output, var.into());
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct MapPredicate {
    expression: Expression,
}

impl MapPredicate {
    pub fn new(input: &str) -> Result<MapPredicate, Error> {
        let expression = PredicateParser::new(input).parse()?;
        Ok(MapPredicate { expression })
    }
}

impl Predicate for MapPredicate {
    type Value = f32;

    fn interpret(&self, vars: &DataValue) -> Result<Self::Value, Error> {
        self.expression.interpret(vars).map_err(Into::into)
    }
}

#[cfg(feature = "serde")]
pub mod serde {
    use crate::spec::transform::map::MapPipe;
    use serde::de::{MapAccess, Visitor};
    use serde::{de, Deserialize, Deserializer};
    use std::fmt;

    impl<'de: 'a, 'a> Deserialize<'de> for MapPipe {
        fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            struct MapPipeVisitor;

            impl<'a> Visitor<'a> for MapPipeVisitor {
                type Value = MapPipe;

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

                    MapPipe::new(predicate, output)
                        .map_err(|err| de::Error::custom(err.to_string()))
                }
            }

            deserializer.deserialize_any(MapPipeVisitor)
        }
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod serde_tests {
    use crate::spec::transform::map::{MapPipe, MapPredicate};

    #[test]
    fn deserialize_map() {
        let map = serde_json::from_str::<MapPipe>(r#"{ "fn": "a + 2.0", "output": "b" }"#).unwrap();

        assert_eq!(map.predicate, MapPredicate::new("a + 2.0").unwrap());
        assert_eq!(map.output, "b");
    }
}
