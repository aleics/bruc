use crate::scale::linear::LinearScale;

pub mod domain;
pub mod linear;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub enum Scale<'a> {
  #[cfg_attr(feature = "serde", serde(borrow))]
  Linear(LinearScale<'a>),
}

pub trait Scaler {
  type Item;

  fn scale(&self, value: Self::Item) -> Self::Item;
}
