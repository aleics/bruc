use crate::graph::{Evaluation, MultiPulse, Pulse, PulseValue, SinglePulse};
use crate::scene::SceneItem;
use crate::spec::mark::base::{X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME};
use crate::spec::mark::line::LineMark;

/// `LineOperator` represents an operator of the graph, which generates a `LineMark` instance from
/// the incoming `Pulse` instance.
#[derive(Debug, PartialEq)]
pub struct LineOperator {
  mark: LineMark,
}

impl LineOperator {
  /// Create a new `LineOperator` instance with a certain line mark.
  pub fn new(mark: LineMark) -> Self {
    LineOperator { mark }
  }

  /// Apply the operator's logic by generating line marks from the incoming already encoded pulse.
  /// values.
  fn apply(values: &[PulseValue]) -> Vec<PulseValue> {
    let mut lines = Vec::new();

    // Iterate in chunks of 2 consisting of the begin and end of the line
    for i in 0..(values.len() - 1) {
      let begin = values.get(i).and_then(LineOperator::read_point);
      let end = values.get(i + 1).and_then(LineOperator::read_point);

      match (begin, end) {
        (Some(begin), Some(end)) => {
          let line = PulseValue::Marks(SceneItem::line(begin, end, "black", 1.0));
          lines.push(line)
        }
        (Some(begin), None) => {
          let line = PulseValue::Marks(SceneItem::line(begin, begin, "black", 1.0));
          lines.push(line)
        }
        _ => {}
      };
    }

    lines
  }

  /// Read a point out of a data pulse value
  fn read_point(value: &PulseValue) -> Option<(f32, f32)> {
    if let PulseValue::Data(data_value) = value {
      // Read "x" field
      let x = data_value
        .get_number(X_AXIS_FIELD_NAME)
        .copied()
        .unwrap_or(0.0);

      // Read "y" field
      let y = data_value
        .get_number(Y_AXIS_FIELD_NAME)
        .copied()
        .unwrap_or(0.0);

      return Some((x, y));
    }

    None
  }
}

impl Evaluation for LineOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::single(LineOperator::apply(&single.values))
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    self.evaluate_single(multi.aggregate())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::graph::node::mark::LineOperator;
  use crate::graph::{Evaluation, Pulse, PulseValue, SinglePulse};
  use crate::scene::SceneItem;
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
      Pulse::single(vec![
        PulseValue::Marks(SceneItem::line((2.0, 1.0), (5.0, 1.0), "black", 1.0)),
        PulseValue::Marks(SceneItem::line((5.0, 1.0), (10.0, 1.0), "black", 1.0)),
        PulseValue::Marks(SceneItem::line((10.0, 1.0), (15.0, 1.0), "black", 1.0))
      ])
    );
  }
}
