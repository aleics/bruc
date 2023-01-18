struct Scenegraph {
  items: Vec<SceneGroup>
}

impl Scenegraph {
  pub fn new() -> Self {
    Scenegraph { items: Vec::new() }
  }

  pub fn add(&mut self, item: SceneGroup) {
    self.items.push(item)
  }
}

enum SceneItem {
  Group(Box<SceneGroup>),
  Line(Box<SceneLine>)
}

impl SceneItem {
  pub fn group(group: SceneGroup) -> Self {
    SceneItem::Group(Box::new(group))
  }

  pub fn line(line: SceneLine) -> Self {
    SceneItem::Line(Box::new(line))
  }
}

struct SceneGroup {
  items: Vec<SceneItem>
}

impl SceneGroup {
  pub fn new() -> Self {
    SceneGroup { items: Vec::new() }
  }

  pub fn with_items(items: Vec<SceneItem>) -> Self {
    SceneGroup { items }
  }
}

struct SceneLine {
  base: SceneItemBase,
  stroke: String,
  stroke_width: f32,
  x2: f32,
  y2: f32
}

impl SceneLine {
  pub fn new(base: SceneItemBase, stroke: String, stroke_width: f32, x2: f32, y2: f32) -> Self {
    SceneLine { base, stroke, stroke_width, x2, y2 }
  }
}

struct SceneItemBase {
  x: f32,
  y: f32,
}

impl SceneItemBase {
  pub fn new(x: f32, y: f32) -> Self {
    SceneItemBase { x, y }
  }
}
