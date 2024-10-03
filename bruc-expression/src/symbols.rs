use std::fmt;

pub(crate) const TRUE: &str = "true";
pub(crate) const FALSE: &str = "false";
pub(crate) const AND: char = '&';
pub(crate) const OR: char = '|';
pub(crate) const NOT: char = '!';
pub(crate) const GREATER: char = '>';
pub(crate) const LESS: char = '<';
pub(crate) const EQUAL: char = '=';
pub(crate) const OPEN: char = '(';
pub(crate) const CLOSE: char = ')';
pub(crate) const SUM: char = '+';
pub(crate) const SUB: char = '-';
pub(crate) const MUL: char = '*';
pub(crate) const DIV: char = '/';

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Operator {
    And,
    Or,
    Not,
    Sum,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Sum => write!(f, "+"),
            Operator::Sub => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::And => write!(f, "&&"),
            Operator::Or => write!(f, "||"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::Greater => write!(f, ">"),
            Operator::GreaterOrEqual => write!(f, ">="),
            Operator::Less => write!(f, "<"),
            Operator::LessOrEqual => write!(f, "<="),
            Operator::Not => write!(f, "!"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Operator(Operator),
    Open,
    Close,
    Number(f32),
    Boolean(bool),
    Variable(String),
}

impl Symbol {
    pub fn operator(&self) -> Option<Operator> {
        match self {
            Symbol::Operator(operator) => Some(*operator),
            _ => None,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Symbol::Operator(operator) => match operator {
                Operator::And => write!(f, "{AND}"),
                Operator::Or => write!(f, "{OR}"),
                Operator::Not => write!(f, "{NOT}"),
                Operator::Sum => write!(f, "{SUM}"),
                Operator::Sub => write!(f, "{SUB}"),
                Operator::Mul => write!(f, "{MUL}"),
                Operator::Div => write!(f, "{DIV}"),
                Operator::Equal => write!(f, "{EQUAL}"),
                Operator::NotEqual => write!(f, "{NOT}{EQUAL}"),
                Operator::Greater => write!(f, "{GREATER}"),
                Operator::GreaterOrEqual => write!(f, "{GREATER}{EQUAL}"),
                Operator::Less => write!(f, "{LESS}"),
                Operator::LessOrEqual => write!(f, "{LESS}{EQUAL}"),
            },
            Symbol::Open => write!(f, "{OPEN}"),
            Symbol::Close => write!(f, "{CLOSE}"),
            Symbol::Number(number) => write!(f, "{number}"),
            Symbol::Boolean(boolean) => write!(f, "{boolean}"),
            Symbol::Variable(variable) => write!(f, "{variable}"),
        }
    }
}
