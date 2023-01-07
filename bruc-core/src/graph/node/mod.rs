use crate::{
  data::Series,
  transform::{filter::FilterPipe, map::MapPipe},
};

use self::data::SourceOperator;
use self::transform::{FilterOperator, MapOperator};

use super::{Evaluation, Pulse};

pub mod data;
pub mod transform;

#[derive(Debug)]
pub struct Node {
  pub(crate) id: usize,
  pub(crate) operator: Operator,
  pub(crate) pulse: Pulse,
}

impl Node {
  pub fn new(id: usize, operator: Operator) -> Self {
    Node {
      id,
      operator,
      pulse: Pulse::init(),
    }
  }

  pub async fn execute(&mut self, pulse: Pulse) {
    self.pulse = self.operator.evaluate(pulse).await;
  }
}

#[derive(Debug)]
pub enum Operator {
  Source(SourceOperator),
  Map(MapOperator),
  Filter(FilterOperator),
}

impl Operator {
  pub fn source(data: Series) -> Self {
    Operator::Source(SourceOperator::new(data))
  }

  pub fn map(pipe: MapPipe) -> Self {
    Operator::Map(MapOperator::new(pipe))
  }

  pub fn filter(pipe: FilterPipe) -> Self {
    Operator::Filter(FilterOperator::new(pipe))
  }

  pub async fn evaluate(&self, pulse: Pulse) -> Pulse {
    match self {
      Operator::Source(source) => source.evaluate(pulse).await,
      Operator::Map(map) => map.evaluate(pulse).await,
      Operator::Filter(filter) => filter.evaluate(pulse).await,
    }
  }
}
