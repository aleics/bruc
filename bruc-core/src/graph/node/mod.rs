use crate::{
  data::Series,
  transform::{filter::FilterPipe, map::MapPipe},
};

use self::data::DataOperator;
use self::transform::{FilterOperator, MapOperator};

use super::{Evaluation, Pulse};

pub mod data;
pub mod transform;
pub mod scale;
pub mod render;

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
  Data(DataOperator),
  Map(MapOperator),
  Filter(FilterOperator),
}

impl Operator {
  pub fn data(data: Series) -> Self {
    Operator::Data(DataOperator::new(data))
  }

  pub fn map(pipe: MapPipe) -> Self {
    Operator::Map(MapOperator::new(pipe))
  }

  pub fn filter(pipe: FilterPipe) -> Self {
    Operator::Filter(FilterOperator::new(pipe))
  }

  pub async fn evaluate(&self, pulse: Pulse) -> Pulse {
    match self {
      Operator::Data(data) => data.evaluate(pulse).await,
      Operator::Map(map) => map.evaluate(pulse).await,
      Operator::Filter(filter) => filter.evaluate(pulse).await,
    }
  }
}
