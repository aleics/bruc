pub struct Scenegraph {
  items: Vec<SceneGroup>,
}

impl Scenegraph {
  pub fn new() -> Self {
    Default::default()
  }

  pub fn add(&mut self, item: SceneGroup) {
    self.items.push(item)
  }
}

impl Default for Scenegraph {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SceneItem {
  Group(Box<SceneGroup>),
  Line(Box<SceneLine>),
}

impl SceneItem {
  pub fn group(group: SceneGroup) -> Self {
    SceneItem::Group(Box::new(group))
  }

  pub fn line(line: SceneLine) -> Self {
    SceneItem::Line(Box::new(line))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneGroup {
  items: Vec<SceneItem>,
}

impl SceneGroup {
  pub fn new() -> Self {
    Default::default()
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
  pub fn new(points: Vec<(f32, f32)>, stroke: String, stroke_width: f32) -> Self {
    SceneLine {
      points,
      stroke,
      stroke_width,
    }
  }
}
