#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use crate::parser::Parser;
use crate::render::Renderer;
use crate::scene::{SceneRoot, Scenegraph};
use crate::spec::Specification;

pub mod data;
pub mod graph;
mod parser;
pub mod render;
mod scene;
pub mod spec;

#[derive(Debug, PartialEq)]
pub struct View {
  scene: Scenegraph,
}

impl View {
  pub fn new(scene: Scenegraph) -> View {
    View { scene }
  }

  pub async fn build(spec: Specification) -> View {
    let width = spec.dimensions.width;
    let height = spec.dimensions.height;
    let items = Parser.parse(spec).build().await;

    let scenegraph = Scenegraph::new(SceneRoot::new(items, width, height));

    View::new(scenegraph)
  }

  pub fn render<R: Renderer>(self, renderer: R) -> String {
    renderer.render(&self.scene)
  }
}

#[cfg(test)]
mod tests {
  use crate::data::DataValue;
  use crate::render::DebugRenderer;
  use crate::scene::{SceneItem, SceneRoot, Scenegraph};
  use crate::spec::data::DataEntry;
  use crate::spec::mark::line::{LineMark, LinePropertiesBuilder};
  use crate::spec::mark::{DataSource, Mark};
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::range::Range;
  use crate::spec::scale::{Scale, ScaleKind};
  use crate::spec::transform::filter::FilterPipe;
  use crate::spec::transform::map::MapPipe;
  use crate::spec::transform::pipe::Pipe;
  use crate::spec::{Dimensions, Specification};
  use crate::View;

  fn specification() -> Specification {
    Specification::new(
      Dimensions::new(40, 20),
      vec![DataEntry::new(
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
      vec![
        Scale::new(
          "horizontal",
          ScaleKind::Linear(LinearScale::new(
            Domain::Literal(0.0, 20.0),
            Range::Literal(0.0, 40.0),
          )),
        ),
        Scale::new(
          "vertical",
          ScaleKind::Linear(LinearScale::new(
            Domain::Literal(0.0, 20.0),
            Range::Literal(0.0, 20.0),
          )),
        ),
      ],
      vec![Mark::line(
        "primary",
        LineMark::new(
          LinePropertiesBuilder::new()
            .with_x(DataSource::field("a", Some("horizontal")))
            .with_y(DataSource::field("b", Some("vertical")))
            .build(),
        ),
      )],
    )
  }

  #[tokio::test]
  async fn builds_specification() {
    // when
    let view = View::build(specification()).await;

    // then
    assert_eq!(
      view,
      View::new(Scenegraph::new(SceneRoot::new(
        vec![SceneItem::group(vec![SceneItem::line(
          vec![(10.0, 13.0), (26.0, 5.0)],
          "black".to_string(),
          1.0
        )])],
        40,
        20
      )))
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
      "Scenegraph { root: SceneRoot { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(10.0, 13.0), (26.0, 5.0)] })] })], width: 40, height: 20 } }"
    )
  }
}
