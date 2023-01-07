use crate::data::Series;
use crate::graph::{Evaluation, MultiPulse, Pulse, SinglePulse};
use crate::transform::filter::FilterPipe;
use crate::transform::map::MapPipe;

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

#[derive(Debug)]
pub struct SourceOperator {
  data: Series,
}

impl SourceOperator {
  fn new(data: Series) -> Self {
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

#[derive(Debug)]
pub struct MapOperator {
  pipe: MapPipe,
}

impl MapOperator {
  fn new(pipe: MapPipe) -> Self {
    MapOperator { pipe }
  }

  fn apply(&self, values: &Series) -> Series {
    let mut result = values.clone();

    for mut value in &mut result {
      self.pipe.apply(&mut value);
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
  fn new(pipe: FilterPipe) -> Self {
    FilterOperator { pipe }
  }

  fn apply(&self, values: &Series) -> Series {
    let mut result = Vec::with_capacity(values.len());

    for value in values {
      if self.pipe.apply(&value) {
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
