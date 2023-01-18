use crate::graph::{Evaluation, MultiPulse, Pulse, PulseValue, SinglePulse};
use crate::mark::line::LineMark;

#[derive(Debug, PartialEq)]
pub struct LineOperator {
  mark: LineMark,
}

impl LineOperator {
  pub fn new(mark: LineMark) -> Self {
    LineOperator { mark }
  }

  pub fn apply(&self, values: &[PulseValue]) -> Vec<PulseValue> {
    let result = values.to_vec();

    result
  }
}

impl Evaluation for LineOperator {
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

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::graph::node::mark::LineOperator;
  use crate::graph::{Evaluation, Pulse, PulseValue};
  use crate::mark::line::{Interpolate, LineMark, LineMarkProperties};

  #[tokio::test]
  async fn computes_line() {
    let series = vec![
      PulseValue::Data(DataValue::from_pairs(vec![("x", (2.0).into()), ("y", 1.0.into()), ("width", 100.0.into()), ("height", 50.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("x", 5.0.into()), ("y", 1.0.into()), ("width", 100.0.into()), ("height", 50.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("x", 10.0.into()), ("y", 1.0.into()), ("width", 100.0.into()), ("height", 50.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("x", 15.0.into()), ("y", 1.0.into()), ("width", 100.0.into()), ("height", 50.0.into())])),
    ];

    let mut operator = LineOperator::new(
      LineMark::new(
        LineMarkProperties::new(
          None,
          None,
          None,
          None,
          Interpolate::Linear
        )
      )
    );

    let pulse = operator.evaluate(Pulse::single(series)).await;
  }
}
