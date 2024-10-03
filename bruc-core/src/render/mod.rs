use crate::scene::{SceneDimensions, Scenegraph};

pub mod svg;

pub trait SceneRenderer {
    fn render(&self, scene: &Scenegraph) -> String;
}

trait ItemRenderer {
    type RenderResult;

    fn render(&self, dimensions: &SceneDimensions) -> Self::RenderResult;
}

pub struct DebugRenderer;

impl SceneRenderer for DebugRenderer {
    fn render(&self, scene: &Scenegraph) -> String {
        format!("{scene:?}")
    }
}
