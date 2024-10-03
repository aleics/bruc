use std::iter::Peekable;

use strizer::{StringTokenizer, TokenKind};

use crate::symbols::{
    Operator, Symbol, AND, CLOSE, DIV, EQUAL, FALSE, GREATER, LESS, MUL, NOT, OPEN, OR, SUB, SUM,
    TRUE,
};

pub(crate) struct Lexer<'a> {
    tokenizer: Peekable<StringTokenizer<'a>>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(text: &'a str) -> Lexer<'a> {
        Lexer {
            tokenizer: StringTokenizer::new(
                text,
                &[
                    AND, OR, NOT, GREATER, LESS, EQUAL, OPEN, CLOSE, SUM, SUB, MUL, DIV,
                ],
            )
            .peekable(),
        }
    }

    #[inline]
    fn symbol_from_word(word: &str) -> Symbol {
        match word {
            TRUE => Symbol::Boolean(true),
            FALSE => Symbol::Boolean(false),
            _ => Symbol::Variable(word.to_string()),
        }
    }

    #[inline]
    fn symbol_from_character(&mut self, character: char) -> Option<Symbol> {
        match character {
            AND => self.eat_and(),
            OR => self.eat_or(),
            NOT => self.eat_ne(),
            GREATER => self.eat_ge(),
            LESS => self.eat_le(),
            EQUAL => self.eat_eq(),
            SUM => Some(Lexer::eat_sum()),
            SUB => Some(Lexer::eat_sub()),
            MUL => Some(Lexer::eat_mul()),
            DIV => Some(Lexer::eat_div()),
            OPEN => Some(Symbol::Open),
            CLOSE => Some(Symbol::Close),
            _ => None,
        }
    }

    fn symbol_from_number(number: f32) -> Symbol {
        Symbol::Number(number)
    }

    #[inline]
    fn eat_and(&mut self) -> Option<Symbol> {
        let (token, _, _) = self.tokenizer.peek()?;

        if token.is_character_equal(AND) {
            self.tokenizer.next();
            Some(Symbol::Operator(Operator::And))
        } else {
            None
        }
    }

    #[inline]
    fn eat_or(&mut self) -> Option<Symbol> {
        let (token, _, _) = self.tokenizer.peek()?;

        if token.is_character_equal(OR) {
            self.tokenizer.next();
            Some(Symbol::Operator(Operator::Or))
        } else {
            None
        }
    }

    #[inline]
    fn eat_ne(&mut self) -> Option<Symbol> {
        let (token, _, _) = self.tokenizer.peek()?;

        if token.is_character_equal(EQUAL) {
            self.tokenizer.next();
            Some(Symbol::Operator(Operator::NotEqual))
        } else {
            Some(Symbol::Operator(Operator::Not))
        }
    }

    #[inline]
    fn eat_eq(&mut self) -> Option<Symbol> {
        let (token, _, _) = self.tokenizer.peek()?;

        if token.is_character_equal(EQUAL) {
            self.tokenizer.next();
            Some(Symbol::Operator(Operator::Equal))
        } else {
            None
        }
    }

    #[inline]
    fn eat_ge(&mut self) -> Option<Symbol> {
        let (token, _, _) = self.tokenizer.peek()?;

        if token.is_character_equal(EQUAL) {
            self.tokenizer.next();
            Some(Symbol::Operator(Operator::GreaterOrEqual))
        } else {
            Some(Symbol::Operator(Operator::Greater))
        }
    }

    #[inline]
    fn eat_le(&mut self) -> Option<Symbol> {
        let (token, _, _) = self.tokenizer.peek()?;

        if token.is_character_equal(EQUAL) {
            self.tokenizer.next();
            Some(Symbol::Operator(Operator::LessOrEqual))
        } else {
            Some(Symbol::Operator(Operator::Less))
        }
    }

    fn eat_sum() -> Symbol {
        Symbol::Operator(Operator::Sum)
    }

    fn eat_sub() -> Symbol {
        Symbol::Operator(Operator::Sub)
    }

    fn eat_mul() -> Symbol {
        Symbol::Operator(Operator::Mul)
    }

    fn eat_div() -> Symbol {
        Symbol::Operator(Operator::Div)
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Symbol;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (token, _, slice) = self.tokenizer.next()?;

        match token.kind() {
            TokenKind::Character(character) => self.symbol_from_character(*character),
            TokenKind::Word => slice.parse().map_or_else(
                |_| Some(Lexer::symbol_from_word(slice)),
                |number| Some(Lexer::symbol_from_number(number)),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::symbols::{Operator, Symbol};

    #[test]
    fn finds_all_boolean_symbols() {
        let symbols: Vec<Symbol> = Lexer::new("true || false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::Or),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("true||false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::Or),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("true && false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::And),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("true&&false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::And),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("true == false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::Equal),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("true==false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::Equal),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("true != false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::NotEqual),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("true!=false").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Boolean(true),
                Symbol::Operator(Operator::NotEqual),
                Symbol::Boolean(false)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("!true").collect();
        assert_eq!(
            symbols,
            vec![Symbol::Operator(Operator::Not), Symbol::Boolean(true)]
        );

        let symbols: Vec<Symbol> = Lexer::new("! true").collect();
        assert_eq!(
            symbols,
            vec![Symbol::Operator(Operator::Not), Symbol::Boolean(true)]
        );
    }

    #[test]
    fn finds_variable_in_boolean_expression() {
        let symbols: Vec<Symbol> = Lexer::new("(foo && false)").collect();

        assert_eq!(
            symbols,
            vec![
                Symbol::Open,
                Symbol::Variable("foo".to_string()),
                Symbol::Operator(Operator::And),
                Symbol::Boolean(false),
                Symbol::Close
            ]
        );
    }

    #[test]
    fn finds_all_number_symbols() {
        let symbols: Vec<Symbol> = Lexer::new("3 > 1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::Greater),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3>1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::Greater),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3 >= 1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::GreaterOrEqual),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3>=1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::GreaterOrEqual),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3 < 1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::Less),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3<1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::Less),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3 <= 1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::LessOrEqual),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3<=1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::LessOrEqual),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3 == 1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::Equal),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3==1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::Equal),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3 != 1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::NotEqual),
                Symbol::Number(1.0)
            ]
        );

        let symbols: Vec<Symbol> = Lexer::new("3!=1").collect();
        assert_eq!(
            symbols,
            vec![
                Symbol::Number(3.0),
                Symbol::Operator(Operator::NotEqual),
                Symbol::Number(1.0)
            ]
        );
    }
}
