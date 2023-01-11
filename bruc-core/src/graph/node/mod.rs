use crate::{
  data::DataValue,
  transform::{filter::FilterPipe, group::GroupPipe, map::MapPipe, pipe::Pipe},
};

use self::transform::{FilterOperator, MapOperator};
use self::{data::DataOperator, transform::GroupOperator};

use super::{Evaluation, Pulse};

pub mod data;
pub mod render;
pub mod scale;
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
  Data(DataOperator),
  Map(MapOperator),
  Filter(FilterOperator),
  Group(GroupOperator),
}

impl Operator {
  pub fn data(data: Vec<DataValue>) -> Self {
    Operator::Data(DataOperator::new(data))
  }

  pub fn transform(pipe: Pipe) -> Self {
    match pipe {
      Pipe::Map(map) => Operator::map(map),
      Pipe::Filter(filter) => Operator::filter(filter),
      Pipe::Group(group) => Operator::group(group),
    }
  }

  pub fn map(pipe: MapPipe) -> Self {
    Operator::Map(MapOperator::new(pipe))
  }

  pub fn filter(pipe: FilterPipe) -> Self {
    Operator::Filter(FilterOperator::new(pipe))
  }

  pub fn group(pipe: GroupPipe) -> Self {
    Operator::Group(GroupOperator::new(pipe))
  }

  pub async fn evaluate(&self, pulse: Pulse) -> Pulse {
    match self {
      Operator::Data(data) => data.evaluate(pulse).await,
      Operator::Map(map) => map.evaluate(pulse).await,
      Operator::Filter(filter) => filter.evaluate(pulse).await,
      Operator::Group(group) => group.evaluate(pulse).await,
    }
  }
}
