use crate::data::DataValue;
use crate::graph::node::scale::SCALE_BAND_BANDWIDTH_FIELD_NAME;
use crate::graph::{Evaluation, MultiPulse, Pulse, SinglePulse};
use crate::scene::SceneItem;
use crate::spec::shape::bar::BarShape;
use crate::spec::shape::base::{
  HEIGHT_FIELD_NAME, WIDTH_FIELD_NAME, X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME,
};
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
  async fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::shapes(self.apply(&single))
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    self.evaluate_single(multi.aggregate()).await
  }
}

#[derive(Debug, PartialEq)]
pub struct BarOperator {
  shape: BarShape,
  window: SceneWindow,
}

impl BarOperator {
  /// Create a new `BarOperator` instance with a certain bar shape.
  pub(crate) fn new(shape: BarShape, window: SceneWindow) -> Self {
    BarOperator { shape, window }
  }

  /// Apply the operator's logic by generating bar shapes from the incoming already encoded pulse.
  /// values.
  fn apply(&self, pulse: &SinglePulse) -> Vec<SceneItem> {
    let SinglePulse::Data(values) = pulse else {
      return Vec::new();
    };

    values.iter().map(|value| self.read_rect(value)).collect()
  }

  fn read_rect(&self, value: &DataValue) -> SceneItem {
    let x = value.get_number(X_AXIS_FIELD_NAME).copied().unwrap_or(0.0);
    let y = value.get_number(Y_AXIS_FIELD_NAME).copied().unwrap_or(0.0);
    let width = value.get_number(WIDTH_FIELD_NAME).copied();
    let height = value.get_number(HEIGHT_FIELD_NAME).copied();
    let fill = self.shape.props.fill.clone();

    let horizontal_bandwidth_name =
      format!("{}_{}", X_AXIS_FIELD_NAME, SCALE_BAND_BANDWIDTH_FIELD_NAME);
    let x_bandwidth = value.get_number(&horizontal_bandwidth_name).copied();

    let vertical_bandwidth_name =
      format!("{}_{}", Y_AXIS_FIELD_NAME, SCALE_BAND_BANDWIDTH_FIELD_NAME);
    let y_bandwidth = value.get_number(&vertical_bandwidth_name).copied();

    let width = Self::calculate_dimension_with_bandwidth(width, x_bandwidth);
    let height = Self::calculate_dimension_with_bandwidth(height, y_bandwidth);
    let y = (self.window.height - y - height).max(0.0);

    SceneItem::rect(width, height, x, y, fill)
  }

  fn calculate_dimension_with_bandwidth(dimension: Option<f32>, bandwidth: Option<f32>) -> f32 {
    match (dimension, bandwidth) {
      (Some(dimension), Some(bandwidth)) => dimension.min(bandwidth),
      (Some(dimension), None) => dimension,
      (None, Some(bandwidth)) => bandwidth,
      (None, None) => 0.0,
    }
  }
}

impl Evaluation for BarOperator {
  async fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    Pulse::shapes(self.apply(&single))
  }

  async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    self.evaluate_single(multi.aggregate()).await
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::graph::node::shape::{BarOperator, LineOperator, SceneWindow};
  use crate::graph::{Evaluation, Pulse, SinglePulse};
  use crate::scene::SceneItem;
  use crate::spec::shape::bar::{BarPropertiesBuilder, BarShape};
  use crate::spec::shape::line::{LinePropertiesBuilder, LineShape};
  use crate::spec::shape::DataSource;

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

  #[tokio::test]
  async fn computes_bar() {
    let pulse = SinglePulse::Data(vec![
      DataValue::from_pairs(vec![
        ("x", 0.0.into()),
        ("y", 0.0.into()),
        ("width", 5.0.into()),
        ("height", 3.0.into()),
      ]),
      DataValue::from_pairs(vec![
        ("x", 20.0.into()),
        ("y", 0.0.into()),
        ("width", 5.0.into()),
        ("height", 7.0.into()),
      ]),
    ]);

    let operator = BarOperator::new(
      BarShape::new(
        BarPropertiesBuilder::new()
          .with_width(DataSource::field("x", Some("xscale")))
          .with_height(DataSource::field("y", Some("yscale")))
          .with_x(DataSource::value(5.0.into()))
          .with_y(DataSource::value(0.0.into()))
          .with_fill("red")
          .build(),
      ),
      SceneWindow::new(20, 2),
    );

    let result = operator.evaluate(Pulse::Single(pulse)).await;

    assert_eq!(
      result,
      Pulse::shapes(vec![
        SceneItem::rect(5.0, 3.0, 0.0, 0.0, "red".to_string()),
        SceneItem::rect(5.0, 7.0, 20.0, 0.0, "red".to_string())
      ])
    )
  }
}
