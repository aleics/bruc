use crate::error::Result;
use crate::expr::Expression;
use crate::parser::Parser;

pub mod data;
pub mod error;
pub mod expr;
mod lexer;
mod parser;
pub mod symbols;
pub mod vars;

#[cfg(feature = "serde")]
pub mod serde;

pub struct PredicateParser<'a> {
    parser: Parser<'a>,
}

impl<'a> PredicateParser<'a> {
    pub fn new(text: &'a str) -> PredicateParser<'a> {
        PredicateParser {
            parser: Parser::new(text),
        }
    }

    pub fn parse(&mut self) -> Result<Expression> {
        self.parser.parse()
    }
}
