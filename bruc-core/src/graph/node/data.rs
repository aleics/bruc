use crate::{
  data::DataValue,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
};

/// `DataOperator` represents an operator of the graph, which has a list of `DataValue` as source.
#[derive(Debug, PartialEq)]
pub struct DataOperator {
  data: Vec<DataValue>,
}

impl DataOperator {
  /// Create a new `DataOperator` instance with a list of values.
  pub(crate) fn new(data: Vec<DataValue>) -> Self {
    DataOperator { data }
  }
}

impl Evaluation for DataOperator {
  fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    Pulse::data(self.data.clone())
  }

  fn evaluate_multi(&self, _multi: MultiPulse) -> Pulse {
    Pulse::data(self.data.clone())
  }
}
