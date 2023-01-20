use crate::graph::{Evaluation, MultiPulse, Pulse, PulseValue, SinglePulse};
use crate::mark::base::{X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME};
use crate::mark::line::LineMark;
use crate::scene::{SceneItem, SceneLine};

#[derive(Debug, PartialEq)]
pub struct LineOperator {
  mark: LineMark,
}

impl LineOperator {
  pub fn new(mark: LineMark) -> Self {
    LineOperator { mark }
  }

  pub fn apply(&self, values: &[PulseValue]) -> Vec<PulseValue> {
    let mut points = Vec::new();

    for value in values {
      if let PulseValue::Data(data_value) = value {
        let x = data_value
          .instance
          .get(X_AXIS_FIELD_NAME)
          .and_then(|item| item.get_number())
          .copied()
          .unwrap_or(0.0);
        
        let y = data_value
          .instance
          .get(Y_AXIS_FIELD_NAME)
          .and_then(|item| item.get_number())
          .copied()
          .unwrap_or(0.0);

        points.push((x, y));
      }
    }

    vec![PulseValue::Marks(SceneItem::line(SceneLine::new(
      points,
      "black".to_string(),
      1.0,
    )))]
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
  use crate::scene::{SceneItem, SceneLine};

  #[tokio::test]
  async fn computes_line() {
    let series = vec![
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", (2.0).into()),
        ("y", 1.0.into()),
        ("width", 100.0.into()),
        ("height", 50.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 5.0.into()),
        ("y", 1.0.into()),
        ("width", 100.0.into()),
        ("height", 50.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 10.0.into()),
        ("y", 1.0.into()),
        ("width", 100.0.into()),
        ("height", 50.0.into()),
      ])),
      PulseValue::Data(DataValue::from_pairs(vec![
        ("x", 15.0.into()),
        ("y", 1.0.into()),
        ("width", 100.0.into()),
        ("height", 50.0.into()),
      ])),
    ];

    let operator = LineOperator::new(LineMark::new(LineMarkProperties::new(
      None,
      None,
      None,
      None,
      Interpolate::Linear,
    )));

    let pulse = operator.evaluate(Pulse::single(series)).await;

    assert_eq!(
      pulse,
      Pulse::single(vec![PulseValue::Marks(SceneItem::line(SceneLine::new(
        vec![(2.0, 1.0), (5.0, 1.0), (10.0, 1.0), (15.0, 1.0)],
        "black".to_string(),
        1.0
      )))])
    );
  }
}
