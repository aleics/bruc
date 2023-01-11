use crate::{
  data::Series,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
};

#[derive(Debug)]
pub struct DataOperator {
  data: Series,
}

impl DataOperator {
  pub fn new(data: Series) -> Self {
    DataOperator { data }
  }
}

impl Evaluation for DataOperator {
  fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    Pulse::single(self.data.clone())
  }

  fn evaluate_multi(&self, _multi: MultiPulse) -> Pulse {
    Pulse::single(self.data.clone())
  }
}
