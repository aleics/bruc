use std::collections::HashMap;

use crate::data::DataValue;
use crate::graph::node::shape::SceneWindow;
use crate::spec::axis::Axis;
use crate::spec::scale::linear::LinearScale;
use crate::spec::scale::range::Range;
use crate::spec::scale::{Scale, ScaleKind};
use crate::spec::shape::base::{
  BaseShapeProperties, HEIGHT_FIELD_NAME, WIDTH_FIELD_NAME, X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME,
};
use crate::spec::shape::line::LineShape;
use crate::spec::shape::{DataSource, Shape, ShapeKind};
use crate::spec::Dimensions;
use crate::{
  graph::{node::Operator, Graph},
  spec::data::DataEntry,
  Specification,
};

/// `ParseResult` collects all the data needed after parsing the `Specification`
pub(crate) struct ParseResult {
  pub(crate) graph: Graph,
  pub(crate) collection: ParsedNodeCollection,
}

impl ParseResult {
  fn new() -> Self {
    ParseResult {
      graph: Graph::new(),
      collection: ParsedNodeCollection::default(),
    }
  }
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct ParsedNodeCollection {
  pub(crate) data: HashMap<String, usize>,
  pub(crate) domain: HashMap<String, usize>,
  pub(crate) scales: HashMap<String, usize>,
  pub(crate) axis: HashMap<String, usize>,
  pub(crate) shapes: Vec<usize>,
}

/// `Parser` allows to parse a certain `Specification` into a `Graph` representation, where
/// nodes are generated from the different specification parts, and inter-connected accordingly.
pub(crate) struct Parser;

impl Parser {
  /// Parse a specification instance into a new graph.
  pub(crate) fn parse(&self, specification: Specification) -> ParseResult {
    let mut result = ParseResult::new();

    self.walk_spec(specification, &mut result);

    result
  }

  fn walk_spec(&self, specification: Specification, result: &mut ParseResult) {
    let visitor = Visitor::new(specification.dimensions, &specification.scales);

    for entry in specification.data {
      visitor.visit_data(entry, result);
    }

    for shape in specification.visual.shapes {
      visitor.visit_shape(shape, result);
    }

    for axis in specification.visual.axes {
      visitor.visit_axis(axis, result);
    }
  }
}

struct Visitor {
  dimensions: Dimensions,
  scales: HashMap<String, Scale>,
}

impl Visitor {
  fn new(dimensions: Dimensions, scales: &[Scale]) -> Self {
    let scales = scales
      .iter()
      .map(|scale| (scale.name.clone(), scale.clone()))
      .collect::<HashMap<String, Scale>>();

    Visitor { dimensions, scales }
  }

  fn visit_data(&self, data: DataEntry, result: &mut ParseResult) {
    let data_node = result.graph.add_node(Operator::data(data.values));

    let node = data.transform.into_iter().fold(data_node, |acc, pipe| {
      result.graph.add(Operator::transform(pipe), vec![acc])
    });

    result.collection.data.insert(data.name, node);
  }

  fn visit_shape(&self, shape: Shape, result: &mut ParseResult) {
    let Some(data_node) = result.collection.data.get(&shape.from) else {
      return;
    };

    match shape.kind {
      ShapeKind::Line(line) => self.visit_line_shape(line, *data_node, result),
    };
  }

  fn visit_line_shape(&self, line: LineShape, data_node: usize, result: &mut ParseResult) {
    let scale_nodes = self.visit_shape_props(&line.props.base, data_node, result);

    let node = result.graph.add_node(Operator::line(
      line,
      SceneWindow::new(self.dimensions.width, self.dimensions.height),
    ));
    result.collection.shapes.push(node);

    for scale_node in scale_nodes {
      result.graph.add_edge(scale_node, node);
    }
  }

  fn visit_shape_props(
    &self,
    base: &BaseShapeProperties,
    data_node: usize,
    result: &mut ParseResult,
  ) -> Vec<usize> {
    let mut nodes = Vec::new();

    // Parse scale node for the "x" field
    if let Some(x) = base.x.as_ref() {
      nodes.push(self.visit_data_source(x, X_AXIS_FIELD_NAME, data_node, result));
    }

    // Parse scale node for the "y" field
    if let Some(y) = base.y.as_ref() {
      nodes.push(self.visit_data_source(y, Y_AXIS_FIELD_NAME, data_node, result));
    }

    // Parse scale node for the "width" field
    if let Some(width) = base.width.as_ref() {
      nodes.push(self.visit_data_source(width, WIDTH_FIELD_NAME, data_node, result));
    }

    // Parse scale node for the "height" field
    if let Some(height) = base.height.as_ref() {
      nodes.push(self.visit_data_source(height, HEIGHT_FIELD_NAME, data_node, result));
    }

    nodes
  }

  fn visit_data_source(
    &self,
    data_source: &DataSource,
    output: &str,
    data_node: usize,
    result: &mut ParseResult,
  ) -> usize {
    match data_source {
      // Create a scale operator, if the shape's data source is from a data field
      DataSource::FieldSource { field, scale } => {
        if let Some(scale) = scale
          .as_ref()
          .and_then(|scale_name| self.scales.get(scale_name))
        {
          self.visit_scale(scale.clone(), field, output, data_node, result)
        } else {
          // If no scale is defined, then we copy the field's value in the output field.
          let operator = Operator::identity(field, output);

          result.graph.add(operator, vec![data_node])
        }
      }
      // Create a data operator if the shape's data source is plain data value
      DataSource::ValueSource(value) => {
        let operator = Operator::data(vec![DataValue::from_pairs(vec![(output, value.clone())])]);
        result.graph.add(operator, vec![data_node])
      }
    }
  }

  fn visit_scale(
    &self,
    scale: Scale,
    field: &str,
    output: &str,
    data_node: usize,
    result: &mut ParseResult,
  ) -> usize {
    match scale.kind {
      ScaleKind::Linear(linear) => self.visit_linear(
        linear,
        scale.name.to_string(),
        field,
        output,
        data_node,
        result,
      ),
    }
  }

  fn visit_linear(
    &self,
    linear: LinearScale,
    name: String,
    field: &str,
    output: &str,
    data_node: usize,
    result: &mut ParseResult,
  ) -> usize {
    let domain_operator = Operator::domain(linear.domain.clone());
    let domain_node = result.graph.add_node(domain_operator);

    result.graph.add_edge(data_node, domain_node);
    result.collection.domain.insert(name.clone(), domain_node);

    let Range::Literal(range_min, range_max) = linear.range;
    let linear_operator = Operator::linear((range_min, range_max), field, output);
    let linear_node = result.graph.add_node(linear_operator);

    result.graph.add_edge(domain_node, linear_node);
    result.graph.add_edge(data_node, linear_node);
    result.collection.scales.insert(name, linear_node);

    linear_node
  }

  fn visit_axis(&self, axis: Axis, result: &mut ParseResult) {
    let scale_name = axis.scale.clone();

    let Some(scale) = self.scales.get(&scale_name) else {
      return;
    };
    let Some(domain) = result.collection.domain.get(&scale_name) else {
      return;
    };

    let operator = Operator::axis(
      axis,
      scale.clone(),
      SceneWindow::new(self.dimensions.width, self.dimensions.height),
    );

    let node = result.graph.add(operator, vec![*domain]);
    result.collection.axis.insert(scale_name, node);
  }
}

#[cfg(test)]
mod tests {
  use std::collections::{BTreeMap, BTreeSet, HashMap};

  use crate::graph::node::shape::SceneWindow;
  use crate::graph::node::{Node, Operator};
  use crate::graph::Edge;
  use crate::parser::{ParseResult, ParsedNodeCollection};
  use crate::spec::axis::{Axis, AxisOrientation};
  use crate::spec::scale::ScaleKind;
  use crate::spec::shape::line::LinePropertiesBuilder;
  use crate::spec::transform::map::MapPipe;
  use crate::spec::{Dimensions, Visual};
  use crate::{
    data::DataValue,
    spec::data::DataEntry,
    spec::scale::{domain::Domain, linear::LinearScale, range::Range, Scale},
    spec::shape::{line::LineShape, DataSource, Shape},
    spec::transform::{filter::FilterPipe, pipe::Pipe},
    Specification,
  };

  use super::Parser;

  #[test]
  fn parses_simple() {
    // given
    let spec: Specification = Specification::new(
      Dimensions::default(),
      vec![DataEntry::new(
        "primary",
        vec![
          DataValue::from_pairs(vec![("a", 10.0.into())]),
          DataValue::from_pairs(vec![("a", 5.0.into())]),
        ],
        vec![
          Pipe::Map(MapPipe::new("a - 2", "b").unwrap()),
          Pipe::Filter(FilterPipe::new("b > 2").unwrap()),
        ],
      )],
      vec![
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
    );
    let parser = Parser;

    // when
    let ParseResult { graph, collection } = parser.parse(spec);

    // then
    assert_eq!(
      graph.nodes,
      vec![
        Node::init(Operator::data(vec![
          DataValue::from_pairs(vec![("a", 10.0.into())]),
          DataValue::from_pairs(vec![("a", 5.0.into())]),
        ])),
        Node::init(Operator::map(MapPipe::new("a - 2", "b").unwrap())),
        Node::init(Operator::filter(FilterPipe::new("b > 2").unwrap())),
        Node::init(Operator::domain(Domain::Literal(0.0, 100.0))),
        Node::init(Operator::linear((0.0, 20.0), "a", "x")),
        Node::init(Operator::domain(Domain::Literal(0.0, 100.0))),
        Node::init(Operator::linear((0.0, 10.0), "b", "y")),
        Node::init(Operator::line(
          LineShape::new(
            LinePropertiesBuilder::new()
              .with_x(DataSource::field("a", Some("horizontal")))
              .with_y(DataSource::field("b", Some("vertical")))
              .build()
          ),
          SceneWindow::new(500, 200),
        )),
        Node::init(Operator::axis(
          Axis::new("horizontal", AxisOrientation::Bottom),
          Scale::new(
            "horizontal",
            ScaleKind::Linear(LinearScale::new(
              Domain::Literal(0.0, 100.0),
              Range::Literal(0.0, 20.0)
            ))
          ),
          SceneWindow::new(500, 200)
        )),
        Node::init(Operator::axis(
          Axis::new("vertical", AxisOrientation::Left),
          Scale::new(
            "vertical",
            ScaleKind::Linear(LinearScale::new(
              Domain::Literal(0.0, 100.0),
              Range::Literal(0.0, 10.0)
            ))
          ),
          SceneWindow::new(500, 200)
        ))
      ]
    );
    assert_eq!(
      graph.edges,
      vec![
        Edge::new(0, 1),
        Edge::new(1, 2),
        Edge::new(2, 3),
        Edge::new(3, 4),
        Edge::new(2, 4),
        Edge::new(2, 5),
        Edge::new(5, 6),
        Edge::new(2, 6),
        Edge::new(4, 7),
        Edge::new(6, 7),
        Edge::new(3, 8),
        Edge::new(5, 9)
      ]
    );
    assert_eq!(
      graph.targets,
      BTreeMap::from([
        (0, BTreeSet::from([1])),
        (1, BTreeSet::from([2])),
        (2, BTreeSet::from([3, 4, 5, 6])),
        (3, BTreeSet::from([4, 8])),
        (4, BTreeSet::from([7])),
        (5, BTreeSet::from([6, 9])),
        (6, BTreeSet::from([7])),
        (7, BTreeSet::new()),
        (8, BTreeSet::new()),
        (9, BTreeSet::new())
      ])
    );
    assert_eq!(
      graph.sources,
      BTreeMap::from([
        (0, BTreeSet::new()),
        (1, BTreeSet::from([0])),
        (2, BTreeSet::from([1])),
        (3, BTreeSet::from([2])),
        (4, BTreeSet::from([2, 3])),
        (5, BTreeSet::from([2])),
        (6, BTreeSet::from([2, 5])),
        (7, BTreeSet::from([4, 6])),
        (8, BTreeSet::from([3])),
        (9, BTreeSet::from([5]))
      ])
    );
    assert_eq!(
      graph.degrees,
      BTreeMap::from([
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 1),
        (4, 2),
        (5, 1),
        (6, 2),
        (7, 2),
        (8, 1),
        (9, 1)
      ])
    );
    assert_eq!(
      graph.nodes_in_degree,
      BTreeMap::from([
        (0, BTreeSet::from([0])),
        (1, BTreeSet::from([1, 2, 3, 5, 8, 9])),
        (2, BTreeSet::from([4, 6, 7]))
      ])
    );
    assert_eq!(graph.order, vec![0, 1, 2, 3, 5, 4, 8, 6, 9, 7]);
    assert_eq!(
      collection,
      ParsedNodeCollection {
        data: HashMap::from([("primary".to_string(), 2)]),
        domain: HashMap::from([("vertical".to_string(), 5), ("horizontal".to_string(), 3)]),
        scales: HashMap::from([("vertical".to_string(), 6), ("horizontal".to_string(), 4)]),
        axis: HashMap::from([("vertical".to_string(), 9), ("horizontal".to_string(), 8)]),
        shapes: vec![7]
      }
    )
  }
}
