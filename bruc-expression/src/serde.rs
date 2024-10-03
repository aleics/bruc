use std::fmt;

use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer};

use crate::expr::Expression;
use crate::PredicateParser;

impl<'de, 'a> Deserialize<'de> for Expression
where
    'de: 'a,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ExpressionVisitor;

        impl<'a> Visitor<'a> for ExpressionVisitor {
            type Value = Expression;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("valid predicate")
            }

            #[inline]
            fn visit_borrowed_str<E: serde::de::Error>(
                self,
                value: &'a str,
            ) -> Result<Self::Value, E> {
                PredicateParser::new(value)
                    .parse()
                    .map_err(|_| E::invalid_value(Unexpected::Other(value), &"valid predicate"))
            }
        }

        deserializer.deserialize_any(ExpressionVisitor)
    }
}
