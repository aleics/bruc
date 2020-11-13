use crate::scale::linear::LinearScale;

pub mod domain;
pub mod linear;

#[derive(Debug, PartialEq)]
pub enum Scale<'a> {
  Linear(LinearScale<'a>),
}

pub trait Scaler {
  type Item;

  fn scale(&self, value: Self::Item) -> Self::Item;
}
