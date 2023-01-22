use crate::graph::node::mark::LineOperator;
use crate::graph::node::scale::{IdentityOperator, LinearOperator};
use crate::spec::mark::line::LineMark;
use crate::spec::scale::linear::LinearScale;
use crate::spec::scale::{Scale, ScaleKind};
use crate::{
  data::DataValue,
  spec::transform::{filter::FilterPipe, group::GroupPipe, map::MapPipe, pipe::Pipe},
};

use self::transform::{FilterOperator, MapOperator};
use self::{data::DataOperator, transform::GroupOperator};

use super::{Evaluation, Pulse};

pub(crate) mod data;
pub(crate) mod mark;
pub(crate) mod scale;
pub(crate) mod transform;

/// `Node` represents a node in the `Graph` with a certain operator and a `Pulse` instance.
#[derive(Debug, PartialEq)]
pub struct Node {
  pub(crate) operator: Operator,
  pub(crate) pulse: Pulse,
}

impl Node {
  /// Initialize a new `Node` instance with a certain `Operator`. The associated pulse includes no
  /// data.
  pub(crate) fn init(operator: Operator) -> Self {
    Node {
      operator,
      pulse: Pulse::init(),
    }
  }

  /// Evaluate a `Pulse` instance passed to the node from its source. The resulting pulse is stored.
  pub(crate) async fn execute(&mut self, pulse: Pulse) {
    self.pulse = self.operator.evaluate(pulse).await;
  }
}

/// `Operator` collects all possible operators that can be used in a graph `Node`.
#[derive(Debug, PartialEq)]
pub enum Operator {
  Data(DataOperator),
  Map(MapOperator),
  Filter(FilterOperator),
  Group(GroupOperator),
  Line(LineOperator),
  Linear(LinearOperator),
  Identity(IdentityOperator),
}

impl Operator {
  /// Create a new data `Operator` instance.
  pub fn data(data: Vec<DataValue>) -> Self {
    Operator::Data(DataOperator::new(data))
  }

  /// Create a new `Operator` instance, given a transform `Pipe` definition.
  pub(crate) fn transform(pipe: Pipe) -> Self {
    match pipe {
      Pipe::Map(map) => Operator::map(map),
      Pipe::Filter(filter) => Operator::filter(filter),
      Pipe::Group(group) => Operator::group(group),
    }
  }

  /// Create a new map `Operator` instance.
  pub fn map(pipe: MapPipe) -> Self {
    Operator::Map(MapOperator::new(pipe))
  }

  /// Create a new filter `Operator` instance.
  pub fn filter(pipe: FilterPipe) -> Self {
    Operator::Filter(FilterOperator::new(pipe))
  }

  /// Create a new group `Operator` instance.
  pub fn group(pipe: GroupPipe) -> Self {
    Operator::Group(GroupOperator::new(pipe))
  }

  /// Create a new scale `Operator` instance, for a given `Scale`, data `field` reference and an
  /// `output` field name.
  pub(crate) fn scale(scale: Scale, field: &str, output: &str) -> Self {
    match scale.kind {
      ScaleKind::Linear(linear) => Operator::linear(linear, field, output),
    }
  }

  /// Create a new line `Operator` instance.
  pub(crate) fn line(mark: LineMark) -> Self {
    Operator::Line(LineOperator::new(mark))
  }

  /// Create a new identity `Operator` instance.
  pub(crate) fn identity(field: &str, output: &str) -> Self {
    Operator::Identity(IdentityOperator::new(field, output))
  }

  /// Create a new linear scale `Operator` instance, with a given `field` reference and an `output`
  /// field name.
  pub(crate) fn linear(scale: LinearScale, field: &str, output: &str) -> Self {
    Operator::Linear(LinearOperator::new(scale, field, output))
  }

  /// Evaluate the operator for a certain `Pulse`.
  pub async fn evaluate(&self, pulse: Pulse) -> Pulse {
    match self {
      Operator::Data(data) => data.evaluate(pulse).await,
      Operator::Map(map) => map.evaluate(pulse).await,
      Operator::Filter(filter) => filter.evaluate(pulse).await,
      Operator::Group(group) => group.evaluate(pulse).await,
      Operator::Line(line) => line.evaluate(pulse).await,
      Operator::Linear(linear) => linear.evaluate(pulse).await,
      Operator::Identity(identity) => identity.evaluate(pulse).await,
    }
  }
}
