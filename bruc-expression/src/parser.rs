use std::iter::Peekable;

use crate::error::{Error, ParseError, Result};
use crate::expr::{Cons, Expression};
use crate::lexer::Lexer;
use crate::symbols::{Operator, Symbol};

fn prefix_binding_power(operator: Operator) -> Result<u8> {
  match operator {
    Operator::Sum | Operator::Sub => Ok(11),
    Operator::Not => Ok(12),
    _ => Err(Error::Parse(ParseError::BindingPowerMissing)),
  }
}

fn infix_binding_power(operator: Operator) -> Result<(u8, u8)> {
  match operator {
    Operator::Or => Ok((1, 2)),
    Operator::And => Ok((3, 4)),
    Operator::Equal
    | Operator::NotEqual
    | Operator::Greater
    | Operator::GreaterOrEqual
    | Operator::Less
    | Operator::LessOrEqual => Ok((5, 6)),
    Operator::Sum | Operator::Sub => Ok((7, 8)),
    Operator::Mul | Operator::Div => Ok((9, 10)),
    Operator::Not => Err(Error::Parse(ParseError::BindingPowerMissing)),
  }
}

pub(crate) struct Parser<'a> {
  lexer: Peekable<Lexer<'a>>,
}

impl<'a> Parser<'a> {
  pub(crate) fn new(input: &'a str) -> Parser<'a> {
    Parser {
      lexer: Lexer::new(input).peekable(),
    }
  }

  pub(crate) fn parse(&mut self) -> Result<Expression> {
    self.expression(0)
  }

  #[inline]
  fn expression(&mut self, min_binding_power: u8) -> Result<Expression> {
    let mut left = self.factor()?;

    loop {
      let infix: Option<(Operator, u8)> =
        self
          .lexer
          .peek()
          .and_then(Symbol::operator)
          .and_then(|operator| {
            if let Ok((left_bp, right_bp)) = infix_binding_power(operator) {
              if left_bp < min_binding_power {
                None
              } else {
                Some((operator, right_bp))
              }
            } else {
              None
            }
          });

      if let Some((operator, bp)) = infix {
        self.lexer.next();
        let right = self.expression(bp)?;

        left = Expression::from(Cons::Binary(operator, (left, right)));
      } else {
        break;
      }
    }

    Ok(left)
  }

  #[inline]
  fn factor(&mut self) -> Result<Expression> {
    let symbol = self
      .lexer
      .next()
      .ok_or(Error::Parse(ParseError::InvalidExpression))?;

    match symbol {
      Symbol::Boolean(_) | Symbol::Number(_) | Symbol::Variable(_) => Ok(Expression::Atom(symbol)),
      Symbol::Open => {
        let expression = self.parse()?;
        self.lexer.next();

        Ok(expression)
      }
      Symbol::Operator(operator) => {
        let bp = prefix_binding_power(operator)?;
        let root = self.expression(bp)?;

        Ok(Expression::from(Cons::Unary(operator, root)))
      }
      Symbol::Close => Err(Error::Parse(ParseError::InvalidExpression)),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::parser::Parser;

  #[test]
  fn parse_atoms() {
    let expression = Parser::new("1").parse().unwrap();
    assert_eq!(expression.to_string(), "1");

    let expression = Parser::new("a").parse().unwrap();
    assert_eq!(expression.to_string(), "a");
  }

  #[test]
  fn parse_arithmetic() {
    let expression = Parser::new("a + b + c").parse().unwrap();
    assert_eq!(expression.to_string(), "(+ (+ a b) c)");

    let expression = Parser::new("1 + 2 * 3").parse().unwrap();
    assert_eq!(expression.to_string(), "(+ 1 (* 2 3))");

    let expression = Parser::new("1 + 2 * 3 / 4 + 5").parse().unwrap();
    assert_eq!(expression.to_string(), "(+ (+ 1 (/ (* 2 3) 4)) 5)");

    let expression = Parser::new("a + b * c / d + e").parse().unwrap();
    assert_eq!(expression.to_string(), "(+ (+ a (/ (* b c) d)) e)");
  }

  #[test]
  fn parse_negation() {
    let expression = Parser::new("--1 + 2").parse().unwrap();
    assert_eq!(expression.to_string(), "(+ (- (- 1)) 2)");

    let expression = Parser::new("--f * g").parse().unwrap();
    assert_eq!(expression.to_string(), "(* (- (- f)) g)");
  }

  #[test]
  fn parse_equality() {
    let expression = Parser::new("a == b").parse().unwrap();
    assert_eq!(expression.to_string(), "(== a b)");

    let expression = Parser::new("a != b").parse().unwrap();
    assert_eq!(expression.to_string(), "(!= a b)");

    let expression = Parser::new("a > b").parse().unwrap();
    assert_eq!(expression.to_string(), "(> a b)");

    let expression = Parser::new("a >= b").parse().unwrap();
    assert_eq!(expression.to_string(), "(>= a b)");

    let expression = Parser::new("a < b").parse().unwrap();
    assert_eq!(expression.to_string(), "(< a b)");

    let expression = Parser::new("a <= b").parse().unwrap();
    assert_eq!(expression.to_string(), "(<= a b)");

    let expression = Parser::new("a == b != c").parse().unwrap();
    assert_eq!(expression.to_string(), "(!= (== a b) c)");

    let expression = Parser::new("a == b + c").parse().unwrap();
    assert_eq!(expression.to_string(), "(== a (+ b c))");

    let expression = Parser::new("a + b == c").parse().unwrap();
    assert_eq!(expression.to_string(), "(== (+ a b) c)");

    let expression = Parser::new("a + b <= c").parse().unwrap();
    assert_eq!(expression.to_string(), "(<= (+ a b) c)");
  }

  #[test]
  fn parse_not() {
    let expression = Parser::new("!a").parse().unwrap();
    assert_eq!(expression.to_string(), "(! a)");

    let expression = Parser::new("!a + b == c").parse().unwrap();
    assert_eq!(expression.to_string(), "(== (+ (! a) b) c)");
  }

  #[test]
  fn parse_boolean() {
    let expression = Parser::new("a && b || c").parse().unwrap();
    assert_eq!(expression.to_string(), "(|| (&& a b) c)");

    let expression = Parser::new("a || b && c").parse().unwrap();
    assert_eq!(expression.to_string(), "(|| a (&& b c))");

    let expression = Parser::new("a + b == c && d").parse().unwrap();
    assert_eq!(expression.to_string(), "(&& (== (+ a b) c) d)")
  }

  #[test]
  fn parse_open() {
    let expression = Parser::new("(a)").parse().unwrap();
    assert_eq!(expression.to_string(), "a");

    let expression = Parser::new("!(a)").parse().unwrap();
    assert_eq!(expression.to_string(), "(! a)");

    let expression = Parser::new("(1 + 2) * 3").parse().unwrap();
    assert_eq!(expression.to_string(), "(* (+ 1 2) 3)");

    let expression = Parser::new("!(a != c)").parse().unwrap();
    assert_eq!(expression.to_string(), "(! (!= a c))");
  }
}
