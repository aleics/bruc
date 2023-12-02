use crate::{
  graph::{Evaluation, MultiPulse, Pulse, SinglePulse},
  scene::{SceneAxisRule, SceneAxisTick, SceneItem},
  spec::axis::{Axis, AxisOrientation},
};

use super::{
  shape::SceneWindow,
  util::{interpolate, normalize},
};

const TICK_COUNT: usize = 10;

#[derive(Debug, PartialEq)]
pub struct AxisOperator {
  axis: Axis,
  range: (f32, f32),
  window: SceneWindow,
}

impl AxisOperator {
  pub(crate) fn new(axis: Axis, range: (f32, f32), window: SceneWindow) -> Self {
    AxisOperator {
      axis,
      range,
      window,
    }
  }

  fn apply(&self, domain: (f32, f32)) -> SinglePulse {
    let scene_item = self.linear_axis(self.range, domain);
    SinglePulse::Shapes(vec![scene_item])
  }

  fn linear_axis(&self, range: (f32, f32), domain: (f32, f32)) -> SceneItem {
    SceneItem::axis(
      self.create_ruler(range),
      self.create_ticks(range, domain),
      self.axis.orientation,
    )
  }

  fn create_ticks(&self, range: (f32, f32), domain: (f32, f32)) -> Vec<SceneAxisTick> {
    AxisOperator::create_tick_relative_positions(TICK_COUNT, domain)
      .into_iter()
      .map(|value| {
        let position = interpolate(normalize(value, domain), range);
        SceneAxisTick {
          position: self.orientation_position(position),
          label: format!("{:.2}", value),
        }
      })
      .collect()
  }

  fn create_tick_relative_positions(count: usize, (from, to): (f32, f32)) -> Vec<f32> {
    let step = (to - from) / (count as f32);
    (0..count + 1).map(|i| step * (i as f32)).collect()
  }

  fn create_ruler(&self, (from, to): (f32, f32)) -> SceneAxisRule {
    SceneAxisRule {
      from: self.orientation_position(from),
      to: self.orientation_position(to),
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
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    match single {
      SinglePulse::Domain(min, max) => Pulse::Single(self.apply((min, max))),
      _ => Pulse::shapes(Vec::new()),
    }
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    for pulse in multi.pulses {
      if let SinglePulse::Domain(min, max) = pulse {
        return Pulse::Single(self.apply((min, max)));
      }
    }
    Pulse::shapes(Vec::new())
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    graph::{
      node::{axis::AxisOperator, shape::SceneWindow},
      Evaluation, Pulse,
    },
    scene::{SceneAxisRule, SceneAxisTick, SceneItem},
    spec::axis::{Axis, AxisOrientation},
  };

  #[tokio::test]
  async fn creates_top_axis() {
    let operator = AxisOperator::new(
      Axis::new("horizontal", AxisOrientation::Top),
      (0.0, 200.0),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::domain(0.0, 100.0)).await;

    assert_eq!(
      pulse,
      Pulse::shapes(vec![SceneItem::axis(
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
      )])
    )
  }

  #[tokio::test]
  async fn creates_bottom_axis() {
    let operator = AxisOperator::new(
      Axis::new("horizontal", AxisOrientation::Bottom),
      (0.0, 200.0),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::domain(0.0, 100.0)).await;

    assert_eq!(
      pulse,
      Pulse::shapes(vec![SceneItem::axis(
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
      )])
    )
  }

  #[tokio::test]
  async fn creates_left_axis() {
    let operator = AxisOperator::new(
      Axis::new("vertical", AxisOrientation::Left),
      (0.0, 200.0),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::domain(0.0, 100.0)).await;

    assert_eq!(
      pulse,
      Pulse::shapes(vec![SceneItem::axis(
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
      )])
    )
  }

  #[tokio::test]
  async fn creates_right_axis() {
    let operator = AxisOperator::new(
      Axis::new("vertical", AxisOrientation::Right),
      (0.0, 200.0),
      SceneWindow::new(200, 100),
    );

    let pulse = operator.evaluate(Pulse::domain(0.0, 100.0)).await;

    assert_eq!(
      pulse,
      Pulse::shapes(vec![SceneItem::axis(
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
      )])
    )
  }
}
