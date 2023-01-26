use crate::graph::PulseValue;
use crate::{
  data::DataValue,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
};

/// `DataOperator` represents an operator of the graph, which has a list of `DataValue` as source.
#[derive(Debug, PartialEq)]
pub(crate) struct DataOperator {
  data: Vec<DataValue>,
}

impl DataOperator {
  /// Create a new `DataOperator` instance with a list of values.
  pub(crate) fn new(data: Vec<DataValue>) -> Self {
    DataOperator { data }
  }

  /// Apply the operator's logic by propagating the internal list of data values into a `Pulse`
  /// instance.
  fn apply(&self) -> Vec<PulseValue> {
    self.data.iter().cloned().map(PulseValue::Data).collect()
  }
}

impl Evaluation for DataOperator {
  fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    Pulse::single(self.apply())
  }

  fn evaluate_multi(&self, _multi: MultiPulse) -> Pulse {
    Pulse::single(self.apply())
  }
}
