use std::collections::HashMap;

use async_std::channel::{bounded, Sender};
use async_std::stream::{Stream, StreamExt};

use data::DataValue;
use graph::node::{Node, Operator};
use graph::Graph;

use crate::parser::Parser;
use crate::render::Renderer;
use crate::scene::{SceneDimensions, SceneRoot, Scenegraph};
use crate::spec::Specification;

pub mod data;
pub mod graph;
mod parser;
pub mod render;
mod scene;
pub mod spec;

#[derive(Debug)]
struct ViewState {
  graph: Graph,
  data_nodes: HashMap<String, usize>,
  dimensions: SceneDimensions,
}

#[derive(Debug)]
pub struct View {
  state: ViewState,
  listeners: Vec<Sender<Scenegraph>>,
}

impl View {
  pub fn build(spec: Specification) -> View {
    let dimensions = SceneDimensions {
      width: spec.dimensions.width,
      height: spec.dimensions.height,
    };
    let parse_result = Parser.parse(spec);

    View {
      state: ViewState {
        graph: parse_result.graph,
        data_nodes: parse_result.data_nodes,
        dimensions,
      },
      listeners: Vec::new(),
    }
  }

  pub async fn set_data(&mut self, name: &str, values: Vec<DataValue>) {
    if let Some(node) = self.state.data_nodes.get(name).copied() {
      self
        .state
        .graph
        .replace_node(node, Node::init(Operator::data(values)));

      let items = self.state.graph.build_tree(node).await;
      let scene = Scenegraph::new(SceneRoot::new(items, self.state.dimensions));

      self.notify_listeners(scene).await;
    }
  }

  pub async fn render<R: Renderer>(&mut self, renderer: R) -> impl Stream<Item = String> {
    let (sender, recv) = bounded(5);

    let items = self.state.graph.build().await;
    let scene = Scenegraph::new(SceneRoot::new(items, self.state.dimensions));

    sender.send(scene).await.unwrap();
    self.listeners.push(sender);

    recv.map(move |scene| renderer.render(&scene))
  }

  async fn notify_listeners(&self, scene: Scenegraph) {
    for listener in &self.listeners {
      listener.send(scene.clone()).await.unwrap();
    }
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use async_std::stream::StreamExt;

  use crate::data::DataValue;
  use crate::render::DebugRenderer;
  use crate::scene::SceneDimensions;
  use crate::spec::axis::{Axis, AxisOrientation};
  use crate::spec::data::DataEntry;
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::range::Range;
  use crate::spec::scale::{Scale, ScaleKind};
  use crate::spec::shape::line::{LinePropertiesBuilder, LineShape};
  use crate::spec::shape::{DataSource, Shape};
  use crate::spec::transform::filter::FilterPipe;
  use crate::spec::transform::map::MapPipe;
  use crate::spec::transform::pipe::Pipe;
  use crate::spec::{Dimensions, Specification, Visual};
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
      Visual::new(
        vec![Shape::line(
          "primary",
          LineShape::new(
            LinePropertiesBuilder::new()
              .with_x(DataSource::field("a", Some("horizontal")))
              .with_y(DataSource::field("b", Some("vertical")))
              .build(),
          ),
        )],
        vec![
          Axis::new("horizontal", AxisOrientation::Bottom),
          Axis::new("vertical", AxisOrientation::Left),
        ],
      ),
    )
  }

  #[test]
  fn builds_specification() {
    // when
    let view = View::build(specification());

    // then
    assert_eq!(
      view.state.dimensions,
      SceneDimensions {
        width: 40,
        height: 20
      }
    );
    assert_eq!(
      view.state.data_nodes,
      HashMap::from([("primary".to_string(), 2)])
    );
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
      "Scenegraph { root: SceneRoot { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(10.0, 13.0), (26.0, 5.0)] })] })], dimensions: SceneDimensions { width: 40, height: 20 } } }"
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
      "Scenegraph { root: SceneRoot { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(10.0, 13.0), (26.0, 5.0)] })] })], dimensions: SceneDimensions { width: 40, height: 20 } } }"
    );
    assert_eq!(
      second.unwrap(),
      "Scenegraph { root: SceneRoot { items: [Group(SceneGroup { items: [Line(SceneLine { stroke: \"black\", stroke_width: 1.0, points: [(20.0, 20.0), (16.0, 20.0)] })] })], dimensions: SceneDimensions { width: 40, height: 20 } } }"
    );
  }
}
