use std::{error, fmt, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
  Interpretation(InterpretationError),
  Construction(ConstructionError),
  Parse(ParseError),
}

#[derive(Debug)]
pub enum InterpretationError {
  InvalidBinaryExpression,
  InvalidBooleanExpression,
  InvalidNumericExpression,
}

#[derive(Debug)]
pub enum ConstructionError {
  InvalidBooleanConstruction,
  InvalidNumericConstruction,
}

#[derive(Debug)]
pub enum ParseError {
  InvalidExpression,
  BindingPowerMissing,
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Interpretation(error) => match error {
        InterpretationError::InvalidBinaryExpression => {
          write!(f, "InterpretationError::InvalidBinaryExpression")
        }
        InterpretationError::InvalidBooleanExpression => {
          write!(f, "InterpretationError::InvalidBooleanExpression")
        }
        InterpretationError::InvalidNumericExpression => {
          write!(f, "InterpretationError::InvalidNumericExpression")
        }
      },
      Error::Construction(error) => match error {
        ConstructionError::InvalidBooleanConstruction => {
          write!(f, "ConstructionError::InvalidBooleanConstruction")
        }
        ConstructionError::InvalidNumericConstruction => {
          write!(f, "ConstructionError::InvalidNumericConstruction")
        }
      },
      Error::Parse(error) => match error {
        ParseError::InvalidExpression => write!(f, "ParseError::InvalidExpression"),
        ParseError::BindingPowerMissing => write!(f, "ParseError::BindingPowerMissing"),
      },
    }
  }
}

impl error::Error for Error {}
