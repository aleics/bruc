use crate::graph::node::Node;
use crate::graph::{Pulse, PulseValue};

#[derive(Debug, PartialEq)]
pub struct Scenegraph {
  pub(crate) root: SceneGroup,
}

impl Scenegraph {
  pub fn new(root: SceneGroup) -> Self {
    Scenegraph { root }
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

  pub fn line(begin: (f32, f32), end: (f32, f32), stroke: &str, stroke_width: f32) -> Self {
    SceneItem::Line(Box::new(SceneLine::new(begin, end, stroke, stroke_width)))
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
  pub(crate) begin: (f32, f32),
  pub(crate) end: (f32, f32),
}

impl SceneLine {
  pub fn new(begin: (f32, f32), end: (f32, f32), stroke: &str, stroke_width: f32) -> Self {
    SceneLine {
      begin,
      end,
      stroke: stroke.to_string(),
      stroke_width,
    }
  }
}
