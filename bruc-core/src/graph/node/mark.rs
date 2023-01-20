use crate::data::DataValue;
use crate::graph::{Evaluation, MultiPulse, Pulse, PulseValue, SinglePulse};
use crate::scene::{SceneItem, SceneLine};
use crate::spec::mark::base::{X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME};
use crate::spec::mark::line::LineMark;
use bruc_expression::data::DataItem;

#[derive(Debug, PartialEq)]
pub struct LineOperator {
  mark: LineMark,
}

impl LineOperator {
  pub fn new(mark: LineMark) -> Self {
    LineOperator { mark }
  }

  fn encode(&self, multi: MultiPulse) -> SinglePulse {
    let mut pulse_pairs: Vec<Vec<(&str, DataItem)>> = Vec::new();

    for i in 0..multi.pulses.len() {
      let single = multi.pulses.get(i).unwrap();

      let data_values: Vec<Vec<(&str, DataItem)>> = single
        .values
        .iter()
        .flat_map(|value| value.get_data())
        .map(|data| data.pairs())
        .collect();

      if pulse_pairs.is_empty() {
        pulse_pairs = data_values;
      } else {
        for j in 0..data_values.len() {
          if let Some(pairs) = pulse_pairs.get_mut(j) {
            let data_value = data_values.get(j).unwrap();
            pairs.extend(data_value);
          }
        }
      }
    }

    let values = pulse_pairs
      .into_iter()
      .map(|pairs| PulseValue::Data(DataValue::from_pairs(pairs)))
      .collect();

    SinglePulse::new(values)
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
    self.evaluate_single(self.encode(multi))
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::graph::node::mark::LineOperator;
  use crate::graph::{Evaluation, Pulse, PulseValue, SinglePulse};
  use crate::scene::{SceneItem, SceneLine};
  use crate::spec::mark::line::{Interpolate, LineMark, LineMarkProperties};

  #[tokio::test]
  async fn computes_line() {
    let x_pulse = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![("x", 2.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("x", 5.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("x", 10.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("x", 15.0.into())])),
    ]);
    let y_pulse = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![("y", 1.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("y", 1.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("y", 1.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("y", 1.0.into())])),
    ]);
    let width_pulse = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![("width", 100.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("width", 100.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("width", 100.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("width", 100.0.into())])),
    ]);
    let height_pulse = SinglePulse::new(vec![
      PulseValue::Data(DataValue::from_pairs(vec![("height", 100.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("height", 100.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("height", 100.0.into())])),
      PulseValue::Data(DataValue::from_pairs(vec![("height", 100.0.into())])),
    ]);

    let operator = LineOperator::new(LineMark::new(LineMarkProperties::new(
      None,
      None,
      None,
      None,
      Interpolate::Linear,
    )));

    let pulse = operator
      .evaluate(Pulse::multi(vec![
        x_pulse,
        y_pulse,
        width_pulse,
        height_pulse,
      ]))
      .await;

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
