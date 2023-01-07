#![cfg(feature = "serde")]

use expression::data::DataItem;
use expression::expr::{Cons, Expression};
use expression::symbols::{Operator, Symbol};
use expression::vars::Variables;

#[test]
fn deserializes_variables() {
  let json = r#"{"a": 1.0, "b": 2, "c": -1, "d": true}"#;
  let vars: Variables = serde_json::from_str(json).unwrap();

  assert_eq!(vars.find("a"), Some(&DataItem::Number(1.0)));
  assert_eq!(vars.find("b"), Some(&DataItem::Number(2.0)));
  assert_eq!(vars.find("c"), Some(&DataItem::Number(-1.0)));
  assert_eq!(vars.find("d"), Some(&DataItem::Bool(true)));
}

#[test]
fn deserializes_expression() {
  let json = r#""a > 2""#;
  let expression: Expression = serde_json::from_str(json).unwrap();

  assert_eq!(
    expression,
    Expression::from(Cons::Binary(
      Operator::Greater,
      (
        Expression::Atom(Symbol::Variable("a".to_string())),
        Expression::Atom(Symbol::Number(2.0))
      ),
    ))
  );
}
