use crate::graph::node::Node;
use crate::graph::{Pulse, PulseValue};

#[derive(Debug, PartialEq)]
pub struct Scenegraph {
  item: SceneGroup,
}

impl Scenegraph {
  pub fn new(item: SceneGroup) -> Self {
    Scenegraph { item }
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

  pub fn line(line: SceneLine) -> Self {
    SceneItem::Line(Box::new(line))
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
  items: Vec<SceneItem>,
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
  stroke: String,
  stroke_width: f32,
  points: Vec<(f32, f32)>,
}

impl SceneLine {
  pub fn new(points: Vec<(f32, f32)>, stroke: &str, stroke_width: f32) -> Self {
    SceneLine {
      points,
      stroke: stroke.to_string(),
      stroke_width,
    }
  }
}
