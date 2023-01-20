#![feature(async_fn_in_trait)]

use crate::parser::Parser;
use crate::scene::Scenegraph;
use crate::spec::Specification;

mod data;
mod graph;
mod parser;
mod render;
mod scene;
pub mod spec;

pub struct View {
  scene: Scenegraph,
}

impl View {
  pub fn new(scene: Scenegraph) -> View { View { scene } }

  async fn build(spec: Specification) -> View {
    let scene = Parser
      .parse(spec)
      .build()
      .await;

    View::new(scene)
  }
}
