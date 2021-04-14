use expression::data::DataItem;
use expression::expr::Interpretable;
use expression::vars::Variables;
use expression::PredicateParser;

#[test]
fn interprets_simple_bool() {
  let expression = PredicateParser::new("true || false").parse().unwrap();
  let result: bool = expression.interpret(&Variables::new()).unwrap();
  assert_eq!(result, true);
}

#[test]
fn interprets_simple_numeric_with_vars() {
  let vars = Variables::from_pairs(vec![("a", DataItem::Number(4.0))]);
  let expression = PredicateParser::new("a > 3").parse().unwrap();

  let result: bool = expression.interpret(&vars).unwrap();
  assert_eq!(result, true);
}

#[test]
fn interprets_multiple_with_vars() {
  let vars = Variables::from_pairs(vec![
    ("a", DataItem::Number(4.0)),
    ("b", DataItem::Bool(true)),
  ]);
  let expression = PredicateParser::new("(a > 3) && !b").parse().unwrap();
  let result: bool = expression.interpret(&vars).unwrap();
  assert_eq!(result, false);
}

#[test]
fn interprets_multiple_numeric_expressions() {
  let expression = PredicateParser::new("(a < 4) && ((a > 1) && (a != 3))")
    .parse()
    .unwrap();
  let vars = Variables::from_pairs(vec![("a", 3.0.into())]);

  let result: bool = expression.interpret(&vars).unwrap();
  assert_eq!(result, false);
}

#[test]
fn interprets_multiple_boolean_expressions() {
  let expression = PredicateParser::new("(a || true) && ((a || true) && (a && true))")
    .parse()
    .unwrap();
  let vars = Variables::from_pairs(vec![("a", false.into())]);

  let result: bool = expression.interpret(&vars).unwrap();
  assert_eq!(result, false);
}
