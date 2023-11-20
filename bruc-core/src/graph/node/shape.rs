use crate::data::DataValue;
use crate::graph::{Evaluation, MultiPulse, Pulse, SinglePulse};
use crate::scene::SceneItem;
use crate::spec::shape::base::{X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME};
use crate::spec::shape::line::LineShape;

#[derive(Debug, PartialEq)]
pub(crate) struct SceneWindow {
  pub(crate) width: f32,
  pub(crate) height: f32,
}

impl SceneWindow {
  pub fn new(width: usize, height: usize) -> Self {
    SceneWindow {
      width: width as f32,
      height: height as f32,
    }
  }
}

/// `LineOperator` represents an operator of the graph, which generates a `LineShape` instance from
/// the incoming `Pulse` instance.
#[derive(Debug, PartialEq)]
pub struct LineOperator {
  shape: LineShape,
  window: SceneWindow,
}

impl LineOperator {
  /// Create a new `LineOperator` instance with a certain line shape.
  pub(crate) fn new(shape: LineShape, window: SceneWindow) -> Self {
    LineOperator { shape, window }
  }

  /// Apply the operator's logic by generating line shapes from the incoming already encoded pulse.
  /// values.
  fn apply(&self, pulse: &SinglePulse) -> Vec<SceneItem> {
    let SinglePulse::Data(values) = pulse else {
      return Vec::new();
    };

    let points = values
      .iter()
      .map(|value| LineOperator::read_point(value, &self.window))
      .collect();

    let stroke = self
      .shape
      .props
      .stroke
      .as_ref()
      .and_then(|stroke| stroke.get_text())
      .cloned()
      .unwrap_or("black".to_string());

    let stroke_width = self
      .shape
      .props
      .stroke_width
      .as_ref()
      .and_then(|stroke_width| stroke_width.get_number().copied())
      .unwrap_or(1.0);

    vec![SceneItem::line(points, stroke, stroke_width)]
  }

  /// Read a point out of a data pulse value
  fn read_point(value: &DataValue, window: &SceneWindow) -> (f32, f32) {
    // Read "x" field
    let x = value.get_number(X_AXIS_FIELD_NAME).copied().unwrap_or(0.0);

    // Read "y" field
    let y = value.get_number(Y_AXIS_FIELD_NAME).copied().unwrap_or(0.0);

    (x, window.height - y)
  }
}

impl Evaluation for LineOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::shapes(self.apply(&single))
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    self.evaluate_single(multi.aggregate())
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::graph::node::shape::{LineOperator, SceneWindow};
  use crate::graph::{Evaluation, Pulse, SinglePulse};
  use crate::scene::SceneItem;
  use crate::spec::shape::line::{LinePropertiesBuilder, LineShape};

  #[tokio::test]
  async fn computes_line() {
    let x_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("x", 2.0.into())]),
      DataValue::from_pairs(vec![("x", 5.0.into())]),
      DataValue::from_pairs(vec![("x", 10.0.into())]),
      DataValue::from_pairs(vec![("x", 15.0.into())]),
    ]);
    let y_pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![("y", 1.0.into())]),
      DataValue::from_pairs(vec![("y", 1.0.into())]),
      DataValue::from_pairs(vec![("y", 1.0.into())]),
      DataValue::from_pairs(vec![("y", 1.0.into())]),
    ]);

    let operator = LineOperator::new(
      LineShape::new(
        LinePropertiesBuilder::new()
          .with_stroke("red")
          .with_stroke_width(2.0)
          .build(),
      ),
      SceneWindow::new(20, 2),
    );

    let pulse = operator
      .evaluate(Pulse::multi(vec![x_pulse, y_pulse]))
      .await;

    assert_eq!(
      pulse,
      Pulse::shapes(vec![SceneItem::line(
        vec![(2.0, 1.0), (5.0, 1.0), (10.0, 1.0), (15.0, 1.0)],
        "red".to_string(),
        2.0
      )])
    );
  }
}
