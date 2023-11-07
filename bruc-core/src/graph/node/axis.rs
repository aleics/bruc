use bruc_expression::data::DataItem;

use crate::{
  graph::{Evaluation, MultiPulse, Pulse, PulseValue, SinglePulse},
  scene::{SceneAxisRule, SceneAxisTick, SceneItem},
  spec::{
    axis::{Axis, AxisOrientation},
    scale::{domain::Domain, linear::LinearScale, range::Range, Scale, ScaleKind, Scaler},
  },
};

use super::shape::SceneWindow;

const TICK_COUNT: usize = 10;

#[derive(Debug, PartialEq)]
pub struct AxisOperator {
  axis: Axis,
  scale: Scale,
  window: SceneWindow,
}

impl AxisOperator {
  pub(crate) fn new(axis: Axis, scale: Scale, window: SceneWindow) -> Self {
    AxisOperator {
      axis,
      scale,
      window,
    }
  }

  fn apply(&self) -> Vec<PulseValue> {
    let scene_item = match &self.scale.kind {
      ScaleKind::Linear(linear) => self.linear_axis(linear),
    };

    vec![PulseValue::Shapes(scene_item)]
  }

  fn linear_axis(&self, linear: &LinearScale) -> SceneItem {
    SceneItem::axis(
      self.create_ruler(&linear.range),
      self.create_ticks(linear),
      self.axis.orientation,
    )
  }

  fn create_ticks(&self, linear: &LinearScale) -> Vec<SceneAxisTick> {
    AxisOperator::create_tick_relative_positions(TICK_COUNT, &linear.domain)
      .into_iter()
      .filter_map(|value| {
        linear
          .scale(&DataItem::Number(value))
          .map(|position| SceneAxisTick {
            position: self.orientation_position(position),
            label: format!("{:.2}", value),
          })
      })
      .collect()
  }

  fn create_tick_relative_positions(count: usize, domain: &Domain) -> Vec<f32> {
    match domain {
      Domain::Literal(from, to) => {
        let step = (to - from) / (count as f32);
        (0..count + 1).map(|i| step * (i as f32)).collect()
      }
    }
  }

  fn create_ruler(&self, range: &Range) -> SceneAxisRule {
    SceneAxisRule {
      from: self.orientation_position(range.from()),
      to: self.orientation_position(range.to()),
    }
  }

  fn orientation_position(&self, position: f32) -> (f32, f32) {
    match self.axis.orientation {
      AxisOrientation::Top => (position, self.window.height),
      AxisOrientation::Bottom => (position, 0.0),
      AxisOrientation::Left => (0.0, position),
      AxisOrientation::Right => (self.window.width, position),
    }
  }
}

impl Evaluation for AxisOperator {
  fn evaluate_single(&self, _single: SinglePulse) -> Pulse {
    Pulse::single(self.apply())
  }

  fn evaluate_multi(&self, _multi: MultiPulse) -> Pulse {
    Pulse::single(self.apply())
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    graph::{
      node::{axis::AxisOperator, shape::SceneWindow},
      Evaluation, Pulse, PulseValue,
    },
    scene::{SceneAxisRule, SceneAxisTick, SceneItem},
    spec::{
      axis::{Axis, AxisOrientation},
      scale::{domain::Domain, linear::LinearScale, range::Range, Scale, ScaleKind},
    },
  };

  #[tokio::test]
  async fn creates_top_axis() {
    let operator = AxisOperator::new(
      Axis::new("horizontal", AxisOrientation::Top),
      Scale::new(
        "horizontal",
        ScaleKind::Linear(LinearScale::new(
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 200.0),
        )),
      ),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::single(vec![])).await;

    assert_eq!(
      pulse,
      Pulse::single(vec![PulseValue::Shapes(SceneItem::axis(
        SceneAxisRule {
          from: (0.0, 100.0),
          to: (200.0, 100.0)
        },
        vec![
          SceneAxisTick {
            position: (0.0, 100.0),
            label: "0.00".to_string()
          },
          SceneAxisTick {
            position: (20.0, 100.0),
            label: "10.00".to_string()
          },
          SceneAxisTick {
            position: (40.0, 100.0),
            label: "20.00".to_string()
          },
          SceneAxisTick {
            position: (60.000004, 100.0),
            label: "30.00".to_string()
          },
          SceneAxisTick {
            position: (80.0, 100.0),
            label: "40.00".to_string()
          },
          SceneAxisTick {
            position: (100.0, 100.0),
            label: "50.00".to_string()
          },
          SceneAxisTick {
            position: (120.00001, 100.0),
            label: "60.00".to_string()
          },
          SceneAxisTick {
            position: (140.0, 100.0),
            label: "70.00".to_string()
          },
          SceneAxisTick {
            position: (160.0, 100.0),
            label: "80.00".to_string()
          },
          SceneAxisTick {
            position: (180.0, 100.0),
            label: "90.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 100.0),
            label: "100.00".to_string()
          }
        ],
        AxisOrientation::Top
      ))])
    )
  }

  #[tokio::test]
  async fn creates_bottom_axis() {
    let operator = AxisOperator::new(
      Axis::new("horizontal", AxisOrientation::Bottom),
      Scale::new(
        "horizontal",
        ScaleKind::Linear(LinearScale::new(
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 200.0),
        )),
      ),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::single(vec![])).await;

    assert_eq!(
      pulse,
      Pulse::single(vec![PulseValue::Shapes(SceneItem::axis(
        SceneAxisRule {
          from: (0.0, 0.0),
          to: (200.0, 0.0)
        },
        vec![
          SceneAxisTick {
            position: (0.0, 0.0),
            label: "0.00".to_string()
          },
          SceneAxisTick {
            position: (20.0, 0.0),
            label: "10.00".to_string()
          },
          SceneAxisTick {
            position: (40.0, 0.0),
            label: "20.00".to_string()
          },
          SceneAxisTick {
            position: (60.000004, 0.0),
            label: "30.00".to_string()
          },
          SceneAxisTick {
            position: (80.0, 0.0),
            label: "40.00".to_string()
          },
          SceneAxisTick {
            position: (100.0, 0.0),
            label: "50.00".to_string()
          },
          SceneAxisTick {
            position: (120.00001, 0.0),
            label: "60.00".to_string()
          },
          SceneAxisTick {
            position: (140.0, 0.0),
            label: "70.00".to_string()
          },
          SceneAxisTick {
            position: (160.0, 0.0),
            label: "80.00".to_string()
          },
          SceneAxisTick {
            position: (180.0, 0.0),
            label: "90.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 0.0),
            label: "100.00".to_string()
          }
        ],
        AxisOrientation::Bottom
      ))])
    )
  }

  #[tokio::test]
  async fn creates_left_axis() {
    let operator = AxisOperator::new(
      Axis::new("vertical", AxisOrientation::Left),
      Scale::new(
        "vertical",
        ScaleKind::Linear(LinearScale::new(
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 200.0),
        )),
      ),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::single(vec![])).await;

    assert_eq!(
      pulse,
      Pulse::single(vec![PulseValue::Shapes(SceneItem::axis(
        SceneAxisRule {
          from: (0.0, 0.0),
          to: (0.0, 200.0)
        },
        vec![
          SceneAxisTick {
            position: (0.0, 0.0),
            label: "0.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 20.0),
            label: "10.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 40.0),
            label: "20.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 60.000004),
            label: "30.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 80.0),
            label: "40.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 100.0),
            label: "50.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 120.00001),
            label: "60.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 140.0),
            label: "70.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 160.0),
            label: "80.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 180.0),
            label: "90.00".to_string()
          },
          SceneAxisTick {
            position: (0.0, 200.0),
            label: "100.00".to_string()
          }
        ],
        AxisOrientation::Left
      ))])
    )
  }

  #[tokio::test]
  async fn creates_right_axis() {
    let operator = AxisOperator::new(
      Axis::new("vertical", AxisOrientation::Right),
      Scale::new(
        "vertical",
        ScaleKind::Linear(LinearScale::new(
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 200.0),
        )),
      ),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::single(vec![])).await;

    assert_eq!(
      pulse,
      Pulse::single(vec![PulseValue::Shapes(SceneItem::axis(
        SceneAxisRule {
          from: (200.0, 0.0),
          to: (200.0, 200.0)
        },
        vec![
          SceneAxisTick {
            position: (200.0, 0.0),
            label: "0.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 20.0),
            label: "10.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 40.0),
            label: "20.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 60.000004),
            label: "30.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 80.0),
            label: "40.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 100.0),
            label: "50.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 120.00001),
            label: "60.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 140.0),
            label: "70.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 160.0),
            label: "80.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 180.0),
            label: "90.00".to_string()
          },
          SceneAxisTick {
            position: (200.0, 200.0),
            label: "100.00".to_string()
          }
        ],
        AxisOrientation::Right
      ))])
    )
  }
}
