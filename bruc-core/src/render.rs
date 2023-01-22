use crate::scene::Scenegraph;

pub trait Renderer {
  fn render(&self, scene: Scenegraph) -> String;
}

pub struct DebugRenderer;

impl Renderer for DebugRenderer {
  fn render(&self, scene: Scenegraph) -> String {
    format!("{scene:?}")
  }
}
