use std::{error, fmt};

#[derive(Debug)]
pub enum Error {
  Pipe(PipeError),
}

#[derive(Debug)]
pub enum PipeError {
  Expression(bruc_expreter::error::Error),
}

impl fmt::Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Error::Pipe(error) => match error {
        PipeError::Expression(error) => write!(f, "PipeError::Expression: {}", error.to_string()),
      },
    }
  }
}

impl error::Error for Error {}

impl From<bruc_expreter::error::Error> for Error {
  fn from(error: bruc_expreter::error::Error) -> Self {
    Error::Pipe(PipeError::Expression(error))
  }
}
