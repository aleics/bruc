#![feature(async_fn_in_trait)]

use crate::spec::Specification;

pub mod data;
pub mod graph;
pub mod parser;
pub mod scene;
pub mod spec;

#[derive(Debug, PartialEq)]
pub struct Engine {
  spec: Specification,
}

impl Engine {
  pub fn new(spec: Specification) -> Engine {
    Engine { spec }
  }
}
