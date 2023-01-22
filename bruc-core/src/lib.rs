#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use crate::parser::Parser;
use crate::render::Renderer;
use crate::scene::Scenegraph;
use crate::spec::Specification;

mod data;
mod graph;
mod parser;
mod render;
mod scene;
pub mod spec;

#[derive(Debug, PartialEq)]
pub struct View {
  scene: Scenegraph,
}

impl View {
  pub fn new(scene: Scenegraph) -> View { View { scene } }

  pub async fn build(spec: Specification) -> View {
    let scene = Parser
      .parse(spec)
      .build()
      .await;

    View::new(scene)
  }

  pub fn render<R: Renderer>(self, renderer: R) -> String {
    renderer.render(self.scene)
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::render::DebugRenderer;
  use crate::scene::{Scenegraph, SceneGroup, SceneItem, SceneLine};
  use crate::spec::data::DataEntry;
  use crate::spec::mark::line::{Interpolate, LineMark, LineMarkProperties};
  use crate::spec::mark::{DataSource, Mark};
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::range::Range;
  use crate::spec::scale::{Scale, ScaleKind};
  use crate::spec::Specification;
  use crate::spec::transform::filter::FilterPipe;
  use crate::spec::transform::map::MapPipe;
  use crate::spec::transform::pipe::Pipe;
  use crate::View;

  fn specification() -> Specification {
    Specification {
      data: vec![DataEntry::new(
        "primary",
        vec![
          DataValue::from_pairs(vec![("a", 5.0.into())]),
          DataValue::from_pairs(vec![("a", 13.0.into())]),
        ],
        vec![
          Pipe::Map(MapPipe::new("a + 2", "b").unwrap()),
          Pipe::Filter(FilterPipe::new("b > 2").unwrap()),
        ],
      )],
      scales: vec![
        Scale::new(
          "horizontal",
          ScaleKind::Linear(LinearScale::new(
            Domain::Literal(0.0, 100.0),
            Range::Literal(0.0, 20.0),
          )),
        ),
        Scale::new(
          "vertical",
          ScaleKind::Linear(LinearScale::new(
            Domain::Literal(0.0, 100.0),
            Range::Literal(0.0, 10.0),
          )),
        ),
      ],
      marks: vec![Mark::line(
        "primary",
        LineMark::new(LineMarkProperties::new(
          Some(DataSource::field("a", Some("horizontal"))),
          Some(DataSource::field("b", Some("vertical"))),
          None,
          None,
          Interpolate::Linear,
        )),
      )],
    }
  }

  #[tokio::test]
  async fn builds_specification() {
    // when
    let view = View::build(specification()).await;

    // then
    assert_eq!(
      view,
      View::new(
        Scenegraph::new(SceneGroup::with_items(vec![
          SceneItem::group(vec![SceneItem::line(SceneLine::new(vec![(1.0, 0.7), (2.6, 1.5)], "black", 1.0))])
        ]))
      )
    );
  }

  #[tokio::test]
  async fn renders() {
    // given
    let view = View::build(specification()).await;

    // when
    let result = view.render(DebugRenderer);

    // then
    assert_eq!(
      result,
      "Scenegraph { item: SceneGroup { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(1.0, 0.7), (2.6, 1.5)] })] })] } }"
    )
  }
}
