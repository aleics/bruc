use crate::graph::node::Node;
use crate::graph::pulse::{Pulse, SinglePulse};
use crate::spec::axis::AxisOrientation;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct SceneDimensions {
  pub(crate) width: usize,
  pub(crate) height: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Scenegraph {
  pub(crate) root: SceneRoot,
}

impl Scenegraph {
  pub fn new(root: SceneRoot) -> Self {
    Scenegraph { root }
  }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SceneRoot {
  pub(crate) items: Vec<SceneItem>,
  pub(crate) dimensions: SceneDimensions,
}

impl SceneRoot {
  pub fn new(items: Vec<SceneItem>, dimensions: SceneDimensions) -> Self {
    SceneRoot { items, dimensions }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneItem {
  Group(Box<SceneGroup>),
  Line(Box<SceneLine>),
  Rect(Box<SceneRect>),
  Axis(Box<SceneAxis>),
}

impl SceneItem {
  pub fn group(items: Vec<SceneItem>) -> Self {
    SceneItem::Group(Box::new(SceneGroup::with_items(items)))
  }

  pub fn line(points: Vec<(f32, f32)>, stroke: String, stroke_width: f32) -> Self {
    SceneItem::Line(Box::new(SceneLine::new(points, stroke, stroke_width)))
  }

  pub fn rect(width: f32, height: f32, x: f32, y: f32, fill: String) -> Self {
    SceneItem::Rect(Box::new(SceneRect {
      width,
      height,
      x,
      y,
      fill,
    }))
  }

  pub fn axis(
    rule: SceneAxisRule,
    ticks: Vec<SceneAxisTick>,
    orientation: AxisOrientation,
  ) -> Self {
    SceneItem::Axis(Box::new(SceneAxis::new(rule, ticks, orientation)))
  }

  pub(crate) fn build(node: &Node) -> Option<Self> {
    if let Pulse::Single(single) = &node.pulse {
      let SinglePulse::Shapes(shapes) = single else {
        return None;
      };

      if shapes.len() > 1 {
        Some(SceneItem::group(shapes.clone()))
      } else {
        shapes.first().cloned()
      }
    } else {
      None
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct SceneGroup {
  pub(crate) items: Vec<SceneItem>,
}

impl SceneGroup {
  pub fn new() -> Self {
    SceneGroup::default()
  }

  pub fn with_items(items: Vec<SceneItem>) -> Self {
    SceneGroup { items }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneLine {
  pub(crate) stroke: String,
  pub(crate) stroke_width: f32,
  pub(crate) points: Vec<(f32, f32)>,
}

impl SceneLine {
  pub fn new(points: Vec<(f32, f32)>, stroke: String, stroke_width: f32) -> Self {
    SceneLine {
      points,
      stroke,
      stroke_width,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneRect {
  pub(crate) width: f32,
  pub(crate) height: f32,
  pub(crate) x: f32,
  pub(crate) y: f32,
  pub(crate) fill: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneAxis {
  pub(crate) rule: SceneAxisRule,
  pub(crate) ticks: Vec<SceneAxisTick>,
  pub(crate) orientation: AxisOrientation,
}

impl SceneAxis {
  pub(crate) fn new(
    rule: SceneAxisRule,
    ticks: Vec<SceneAxisTick>,
    orientation: AxisOrientation,
  ) -> Self {
    SceneAxis {
      rule,
      ticks,
      orientation,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneAxisRule {
  pub(crate) from: (f32, f32),
  pub(crate) to: (f32, f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneAxisTick {
  pub(crate) position: (f32, f32),
  pub(crate) label: String,
}
