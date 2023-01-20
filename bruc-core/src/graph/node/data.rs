use crate::graph::PulseValue;
use crate::{
  data::DataValue,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
};

#[derive(Debug)]
pub struct DataOperator {
  data: Vec<DataValue>,
}

impl DataOperator {
  pub fn new(data: Vec<DataValue>) -> Self {
    DataOperator { data }
  }

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
