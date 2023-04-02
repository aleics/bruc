use crate::graph::node::Node;
use crate::graph::{Pulse, PulseValue};

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
  pub(crate) width: usize,
  pub(crate) height: usize,
}

impl SceneRoot {
  pub fn new(items: Vec<SceneItem>, width: usize, height: usize) -> Self {
    SceneRoot {
      items,
      width,
      height,
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneItem {
  Group(Box<SceneGroup>),
  Line(Box<SceneLine>),
}

impl SceneItem {
  pub fn group(items: Vec<SceneItem>) -> Self {
    SceneItem::Group(Box::new(SceneGroup::with_items(items)))
  }

  pub fn line(points: Vec<(f32, f32)>, stroke: String, stroke_width: f32) -> Self {
    SceneItem::Line(Box::new(SceneLine::new(points, stroke, stroke_width)))
  }

  pub(crate) fn build(node: &Node) -> Option<Self> {
    if let Pulse::Single(single) = &node.pulse {
      let items = single
        .values
        .iter()
        .filter_map(PulseValue::get_marks)
        .cloned()
        .collect();

      Some(SceneItem::group(items))
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
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

impl Default for SceneGroup {
  fn default() -> Self {
    Self::new()
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
