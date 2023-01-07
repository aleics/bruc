use crate::{
  data::Series,
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
  transform::{filter::FilterPipe, map::MapPipe},
};

#[derive(Debug)]
pub struct MapOperator {
  pipe: MapPipe,
}

impl MapOperator {
  pub fn new(pipe: MapPipe) -> Self {
    MapOperator { pipe }
  }

  fn apply(&self, values: &Series) -> Series {
    let mut result = values.clone();

    for value in &mut result {
      self.pipe.apply(value);
    }

    result
  }
}

impl Evaluation for MapOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::single(self.apply(&single.values))
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let values = multi.pulses.iter().fold(Vec::new(), |mut acc, pulse| {
      acc.extend(self.apply(&pulse.values));
      acc
    });

    Pulse::single(values)
  }
}

#[derive(Debug)]
pub struct FilterOperator {
  pipe: FilterPipe,
}

impl FilterOperator {
  pub fn new(pipe: FilterPipe) -> Self {
    FilterOperator { pipe }
  }

  fn apply(&self, values: &Series) -> Series {
    let mut result = Vec::with_capacity(values.len());

    for value in values {
      if self.pipe.apply(value) {
        result.push(value.clone())
      }
    }

    result
  }
}

impl Evaluation for FilterOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::single(self.apply(&single.values))
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    let values = multi.pulses.iter().fold(Vec::new(), |mut acc, pulse| {
      acc.extend(self.apply(&pulse.values));
      acc
    });

    Pulse::single(values)
  }
}
