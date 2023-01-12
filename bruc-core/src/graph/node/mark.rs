use crate::graph::{Evaluation, MultiPulse, Pulse, SinglePulse};
use crate::mark::line::LineMark;

#[derive(Debug, PartialEq)]
pub struct Line {
  points: Vec<(f32, f32)>,
  width: f32,
  height: f32,
}

impl Line {
  pub fn new(points: Vec<(f32, f32)>, width: f32, height: f32) -> Line {
    Line {
      points,
      width,
      height,
    }
  }

  pub fn points(&self) -> &Vec<(f32, f32)> {
    &self.points
  }

  pub fn width(&self) -> &f32 {
    &self.width
  }

  pub fn height(&self) -> &f32 {
    &self.height
  }
}

#[derive(Debug, PartialEq)]
pub struct LineOperator {
  mark: LineMark,
}

impl LineOperator {
  pub fn new(mark: LineMark) -> Self {
    LineOperator { mark }
  }
}

impl Evaluation for LineOperator {
  fn evaluate_single(&self, single: SinglePulse) -> Pulse {
    todo!()
  }

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse {
    todo!()
  }
}
