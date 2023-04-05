#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]

use std::collections::HashMap;

use async_std::channel::{bounded, Sender};
use async_std::stream::{Stream, StreamExt};
use data::DataValue;
use graph::node::{Node, Operator};
use graph::Graph;

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

#[derive(Debug)]
pub struct View {
  width: usize,
  height: usize,
  graph: Graph,
  data_nodes: HashMap<String, usize>,
  jobs: Vec<Sender<Scenegraph>>,
}

impl View {
  pub fn build(spec: Specification) -> View {
    let width = spec.dimensions.width;
    let height = spec.dimensions.height;
    let parse_result = Parser.parse(spec);

    View {
      width,
      height,
      graph: parse_result.graph,
      data_nodes: parse_result.data_nodes,
      jobs: Vec::new(),
    }
  }

  pub async fn set_data(&mut self, name: &str, values: Vec<DataValue>) {
    if let Some(node) = self.data_nodes.get(name).copied() {
      self
        .graph
        .replace_node(node, Node::init(Operator::data(values)));

      let items = self.graph.build_tree(node).await;
      let scene = Scenegraph::new(SceneRoot::new(items, self.width, self.height));

      for job in &self.jobs {
        job.send(scene.clone()).await.unwrap();
      }
    }
  }

  pub async fn render<R: Renderer>(&mut self, renderer: R) -> impl Stream<Item = String> {
    let (sender, recv) = bounded(5);

    let items = self.graph.build().await;
    let scene = Scenegraph::new(SceneRoot::new(items, self.width, self.height));

    sender.send(scene).await.unwrap();
    self.jobs.push(sender);

    recv.map(move |scene| renderer.render(&scene))
  }
}

#[cfg(test)]
mod tests {
  use async_std::stream::StreamExt;
  use std::collections::HashMap;

  use crate::data::DataValue;
  use crate::render::DebugRenderer;
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

  #[test]
  fn builds_specification() {
    // when
    let view = View::build(specification());

    // then
    assert_eq!(view.width, 40);
    assert_eq!(view.height, 20);
    assert_eq!(view.data_nodes, HashMap::from([("primary".to_string(), 2)]));
  }

  #[tokio::test]
  async fn renders() {
    // given
    let mut view = View::build(specification());

    // when
    let mut result = view.render(DebugRenderer).await;
    let content = result.next().await;

    // then
    assert_eq!(
      content.unwrap(),
      "Scenegraph { root: SceneRoot { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(10.0, 13.0), (26.0, 5.0)] })] })], width: 40, height: 20 } }"
    )
  }

  #[tokio::test]
  async fn renders_after_set_data() {
    // given
    let mut view = View::build(specification());

    // when
    let mut result = view.render(DebugRenderer).await;
    let first = result.next().await;

    view
      .set_data(
        "primary",
        vec![
          DataValue::from_pairs(vec![("a", 10.0.into())]),
          DataValue::from_pairs(vec![("a", 8.0.into())]),
        ],
      )
      .await;

    let second = result.next().await;

    // then
    assert_eq!(
      first.unwrap(),
      "Scenegraph { root: SceneRoot { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(10.0, 13.0), (26.0, 5.0)] })] })], width: 40, height: 20 } }"
    );
    assert_eq!(
      second.unwrap(),
      "Scenegraph { root: SceneRoot { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(20.0, 20.0), (16.0, 20.0)] })] })], width: 40, height: 20 } }"
    );
  }
}
