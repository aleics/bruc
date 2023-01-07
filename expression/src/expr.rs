use std::fmt;

use crate::data::{DataItem, DataSource};
use crate::error::{ConstructionError, Error, InterpretationError, Result};
use crate::symbols::{Operator, Symbol};

pub trait Interpretable<T> {
  fn interpret(&self, variables: &impl DataSource) -> Result<T>;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Cons {
  Binary(Operator, (Expression, Expression)),
  Unary(Operator, Expression),
}

impl fmt::Display for Cons {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Cons::Binary(operator, (left, right)) => write!(
        f,
        "({} {} {})",
        operator.to_string(),
        left.to_string(),
        right.to_string()
      ),
      Cons::Unary(operator, root) => write!(f, "({} {})", operator.to_string(), root.to_string()),
    }
  }
}

impl Interpretable<bool> for Cons {
  #[inline]
  fn interpret(&self, source: &impl DataSource) -> Result<bool> {
    match self {
      Cons::Binary(operator, (left, right)) => match operator {
        Operator::And => {
          let left_value: bool = left.interpret(source)?;
          let right_value: bool = right.interpret(source)?;

          Ok(left_value && right_value)
        }
        Operator::Or => {
          let left_value: bool = left.interpret(source)?;
          let right_value: bool = right.interpret(source)?;

          Ok(left_value || right_value)
        }
        Operator::Equal => {
          let left_bool: Result<bool> = left.interpret(source);
          if let Ok(left_value) = left_bool {
            let right_value: bool = right.interpret(source)?;

            Ok(left_value == right_value)
          } else {
            let left_value: f32 = left.interpret(source)?;
            let right_value: f32 = right.interpret(source)?;

            Ok((left_value - right_value).abs() < f32::EPSILON)
          }
        }
        Operator::NotEqual => {
          let left_bool: Result<bool> = left.interpret(source);
          if let Ok(left_value) = left_bool {
            let right_value: bool = right.interpret(source)?;

            Ok(left_value != right_value)
          } else {
            let left_value: f32 = left.interpret(source)?;
            let right_value: f32 = right.interpret(source)?;

            Ok((left_value - right_value).abs() > f32::EPSILON)
          }
        }
        Operator::Greater => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value > right_value)
        }
        Operator::GreaterOrEqual => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value >= right_value)
        }
        Operator::Less => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value < right_value)
        }
        Operator::LessOrEqual => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value <= right_value)
        }
        _ => Err(Error::Construction(
          ConstructionError::InvalidBooleanConstruction,
        )),
      },
      Cons::Unary(operator, root) => match operator {
        Operator::Not => root.interpret(source).map(|value: bool| !value),
        _ => Err(Error::Construction(
          ConstructionError::InvalidBooleanConstruction,
        )),
      },
    }
  }
}

impl Interpretable<f32> for Cons {
  #[inline]
  fn interpret(&self, source: &impl DataSource) -> Result<f32> {
    match self {
      Cons::Binary(operator, (left, right)) => match operator {
        Operator::Sum => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value + right_value)
        }
        Operator::Sub => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value - right_value)
        }
        Operator::Mul => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value * right_value)
        }
        Operator::Div => {
          let left_value: f32 = left.interpret(source)?;
          let right_value: f32 = right.interpret(source)?;

          Ok(left_value / right_value)
        }
        _ => Err(Error::Construction(
          ConstructionError::InvalidNumericConstruction,
        )),
      },
      Cons::Unary(operator, root) => match operator {
        Operator::Sub => root.interpret(source).map(|value: f32| -value),
        _ => Err(Error::Construction(
          ConstructionError::InvalidNumericConstruction,
        )),
      },
    }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
  Atom(Symbol),
  Cons(Box<Cons>),
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Expression::Atom(symbol) => write!(f, "{}", symbol.to_string()),
      Expression::Cons(cons) => write!(f, "{}", cons.to_string()),
    }
  }
}

impl Interpretable<bool> for Expression {
  #[inline]
  fn interpret(&self, source: &impl DataSource) -> Result<bool> {
    match self {
      Expression::Atom(symbol) => match symbol {
        Symbol::Boolean(boolean) => Ok(*boolean),
        Symbol::Variable(name) => {
          let variable = source.get(name).ok_or(Error::Interpretation(
            InterpretationError::InvalidBooleanExpression,
          ))?;

          match variable {
            DataItem::Bool(boolean) => Ok(*boolean),
            _ => Err(Error::Construction(
              ConstructionError::InvalidBooleanConstruction,
            )),
          }
        }
        _ => Err(Error::Construction(
          ConstructionError::InvalidBooleanConstruction,
        )),
      },
      Expression::Cons(cons) => cons.interpret(source),
    }
  }
}

impl Interpretable<f32> for Expression {
  #[inline]
  fn interpret(&self, source: &impl DataSource) -> Result<f32> {
    match self {
      Expression::Atom(symbol) => match symbol {
        Symbol::Number(number) => Ok(*number),
        Symbol::Variable(name) => {
          let variable = source.get(name).ok_or(Error::Interpretation(
            InterpretationError::InvalidNumericExpression,
          ))?;

          match variable {
            DataItem::Number(number) => Ok(*number),
            _ => Err(Error::Construction(
              ConstructionError::InvalidNumericConstruction,
            )),
          }
        }
        _ => Err(Error::Construction(
          ConstructionError::InvalidNumericConstruction,
        )),
      },
      Expression::Cons(cons) => cons.interpret(source),
    }
  }
}

impl From<Symbol> for Expression {
  fn from(symbol: Symbol) -> Self {
    Expression::Atom(symbol)
  }
}

impl From<bool> for Expression {
  fn from(value: bool) -> Self {
    Expression::Atom(Symbol::Boolean(value))
  }
}

impl From<f32> for Expression {
  fn from(value: f32) -> Self {
    Expression::Atom(Symbol::Number(value))
  }
}

impl From<&str> for Expression {
  fn from(name: &str) -> Self {
    Expression::Atom(Symbol::Variable(name.to_string()))
  }
}

impl From<Cons> for Expression {
  fn from(cons: Cons) -> Self {
    Expression::Cons(Box::new(cons))
  }
}

#[cfg(test)]
mod tests {
  use crate::error::Result;
  use crate::expr::{Cons, Expression, Interpretable};
  use crate::symbols::Operator;
  use crate::vars::Variables;

  fn unary(root: Expression, operator: Operator) -> Expression {
    Expression::from(Cons::Unary(operator, root))
  }

  fn binary<'a>(left: Expression, operator: Operator, right: Expression) -> Expression {
    Expression::from(Cons::Binary(operator, (left, right)))
  }

  #[test]
  fn interprets_boolean_literals() {
    let expression = Expression::from(true);
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(false);
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_boolean_variables() {
    let expression = Expression::from("a");
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![("a", true.into())]))
      .unwrap();
    assert_eq!(result, true);

    let expression = Expression::from("a");
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![("a", false.into())]))
      .unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_not_boolean_literals() {
    let expression = Expression::from(unary(Expression::from(true), Operator::Not));

    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(unary(Expression::from(false), Operator::Not));

    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(unary(Expression::from("a"), Operator::Not));

    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![("a", true.into())]))
      .unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_not_unary_boolean_expressions() {
    let expression = Expression::from(unary(
      Expression::from(unary(Expression::from(false), Operator::Not)),
      Operator::Not,
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(unary(
      Expression::from(unary(Expression::from("a"), Operator::Not)),
      Operator::Not,
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![("a", true.into())]))
      .unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn interprets_not_binary_boolean_expressions() {
    let expression = Expression::from(unary(
      Expression::from(binary(
        Expression::from(true),
        Operator::And,
        Expression::from(true),
      )),
      Operator::Not,
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_binary_and_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::And,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::And,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::And,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::And,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::And,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", true.into()),
        ("b", false.into()),
      ]))
      .unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_binary_or_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::Or,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::Or,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::Or,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::Or,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::Or,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", true.into()),
        ("b", false.into()),
      ]))
      .unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn interprets_binary_eq_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::Equal,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::Equal,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::Equal,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::Equal,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::Equal,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", true.into()),
        ("b", false.into()),
      ]))
      .unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_binary_neq_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::NotEqual,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::NotEqual,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::NotEqual,
      Expression::from(true),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::NotEqual,
      Expression::from(false),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::NotEqual,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", true.into()),
        ("b", false.into()),
      ]))
      .unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn doesn_interpret_gt_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::Greater,
      Expression::from(false),
    ));
    let result: Result<bool> = expression.interpret(&Variables::new());
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn doesn_interpret_ge_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::GreaterOrEqual,
      Expression::from(false),
    ));
    let result: Result<bool> = expression.interpret(&Variables::new());
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn doesn_interpret_lt_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::Less,
      Expression::from(false),
    ));
    let result: Result<bool> = expression.interpret(&Variables::new());
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn doesn_interpret_le_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(false),
      Operator::LessOrEqual,
      Expression::from(false),
    ));
    let result: Result<bool> = expression.interpret(&Variables::new());
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn interprets_numeric_literals() {
    let expression = Expression::from(3.0);
    let result: f32 = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, 3.0);

    let expression = Expression::from(-3.0);
    let result: f32 = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, -3.0);
  }

  #[test]
  fn interprets_numeric_variables() {
    let expression = Expression::from("a");
    let result: f32 = expression
      .interpret(&Variables::from_pairs(vec![("a", 3.0.into())]))
      .unwrap();
    assert_eq!(result, 3.0);

    let expression = Expression::from("a");
    let result: f32 = expression
      .interpret(&Variables::from_pairs(vec![("a", (-3.0).into())]))
      .unwrap();
    assert_eq!(result, -3.0);
  }

  #[test]
  fn interprets_binary_gt_numeric_expressions() {
    let expression = Expression::from(binary(
      Expression::from(3.0),
      Operator::Greater,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(2.0),
      Operator::Greater,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(1.0),
      Operator::Greater,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::Greater,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", 3.0.into()),
        ("b", 2.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn interprets_binary_ge_numeric_expressions() {
    let expression = Expression::from(binary(
      Expression::from(3.0),
      Operator::GreaterOrEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(2.0),
      Operator::GreaterOrEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(1.0),
      Operator::GreaterOrEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::GreaterOrEqual,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", 3.0.into()),
        ("b", 2.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn interprets_binary_lt_numeric_expressions() {
    let expression = Expression::from(binary(
      Expression::from(3.0),
      Operator::Less,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(2.0),
      Operator::Less,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(1.0),
      Operator::Less,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::Less,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", 3.0.into()),
        ("b", 2.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_binary_le_numeric_expressions() {
    let expression = Expression::from(binary(
      Expression::from(3.0),
      Operator::LessOrEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(2.0),
      Operator::LessOrEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(1.0),
      Operator::LessOrEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::LessOrEqual,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", 3.0.into()),
        ("b", 2.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_binary_eq_numeric_expressions() {
    let expression = Expression::from(binary(
      Expression::from(3.0),
      Operator::Equal,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(2.0),
      Operator::Equal,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(1.0),
      Operator::Equal,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::Equal,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", 3.0.into()),
        ("b", 2.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, false);
  }

  #[test]
  fn interprets_binary_neq_numeric_expressions() {
    let expression = Expression::from(binary(
      Expression::from(3.0),
      Operator::NotEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(2.0),
      Operator::NotEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(1.0),
      Operator::NotEqual,
      Expression::from(2.0),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::NotEqual,
      Expression::from("b"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", 3.0.into()),
        ("b", 2.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn interprets_multiple_binary_boolean_expressions() {
    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::And,
      Expression::from(binary(
        Expression::from(false),
        Operator::Or,
        Expression::from(true),
      )),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(true),
      Operator::And,
      Expression::from(binary(
        Expression::from(false),
        Operator::Or,
        Expression::from(false),
      )),
    ));
    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from(true),
        Operator::And,
        Expression::from(false),
      )),
      Operator::Or,
      Expression::from(binary(
        Expression::from(true),
        Operator::And,
        Expression::from(false),
      )),
    ));

    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from(true),
        Operator::And,
        Expression::from(false),
      )),
      Operator::Or,
      Expression::from(binary(
        Expression::from(false),
        Operator::Or,
        Expression::from(true),
      )),
    ));

    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn interprets_multiple_binary_numeric_expressions() {
    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from(3.0),
        Operator::Greater,
        Expression::from(1.0),
      )),
      Operator::And,
      Expression::from(binary(
        Expression::from(3.0),
        Operator::Less,
        Expression::from(1.0),
      )),
    ));

    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from(3.0),
        Operator::Equal,
        Expression::from(1.0),
      )),
      Operator::Or,
      Expression::from(binary(
        Expression::from(2.0),
        Operator::GreaterOrEqual,
        Expression::from(2.0),
      )),
    ));

    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from(3.0),
        Operator::Greater,
        Expression::from(1.0),
      )),
      Operator::And,
      Expression::from(binary(
        Expression::from(3.0),
        Operator::Less,
        Expression::from(1.0),
      )),
    ));

    let result: bool = expression.interpret(&Variables::new()).unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from("a"),
        Operator::Equal,
        Expression::from(1.0),
      )),
      Operator::Or,
      Expression::from(binary(
        Expression::from(2.0),
        Operator::GreaterOrEqual,
        Expression::from("a"),
      )),
    ));

    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![("a", 2.0.into())]))
      .unwrap();
    assert_eq!(result, true);
  }

  #[test]
  fn interprets_multiple_binary_mixed_expressions() {
    let expression = Expression::from(binary(
      Expression::from("a"),
      Operator::And,
      Expression::from(binary(
        Expression::from("b"),
        Operator::Greater,
        Expression::from(3.0),
      )),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", true.into()),
        ("b", 3.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, false);

    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from("b"),
        Operator::Greater,
        Expression::from(3.0),
      )),
      Operator::Or,
      Expression::from("a"),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", true.into()),
        ("b", 3.0.into()),
      ]))
      .unwrap();
    assert_eq!(result, true);

    let expression = Expression::from(binary(
      Expression::from(binary(
        Expression::from("a"),
        Operator::Greater,
        Expression::from(3.0),
      )),
      Operator::And,
      Expression::from(unary(Expression::from("b"), Operator::Not)),
    ));
    let result: bool = expression
      .interpret(&Variables::from_pairs(vec![
        ("a", 4.0.into()),
        ("b", true.into()),
      ]))
      .unwrap();
    assert_eq!(result, false);
  }
}
