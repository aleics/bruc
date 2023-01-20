use crate::scene::Scenegraph;

pub(crate) trait Renderer {
  fn render(&self, scene: Scenegraph) -> String;
}

pub(crate) struct DebugRenderer;

impl Renderer for DebugRenderer {
  fn render(&self, scene: Scenegraph) -> String {
    format!("{:?}", scene)
  }
}
