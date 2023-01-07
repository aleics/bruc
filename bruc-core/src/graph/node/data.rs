use crate::{
  data::Series,
  graph::{MultiPulse, Pulse, SinglePulse, Evaluation},
};

#[derive(Debug)]
pub struct SourceOperator {
  data: Series,
}

impl SourceOperator {
  pub fn new(data: Series) -> Self {
    SourceOperator { data }
  }
}

impl Evaluation for SourceOperator {
  fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    Pulse::single(self.data.clone())
  }

  fn evaluate_multi(&self, _multi: MultiPulse) -> Pulse {
    Pulse::single(self.data.clone())
  }
}
