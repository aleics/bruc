use std::collections::HashMap;

use crate::data::DataValue;
use crate::graph::node::mark::SceneWindow;
use crate::spec::mark::base::{
  BaseMarkProperties, HEIGHT_FIELD_NAME, WIDTH_FIELD_NAME, X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME,
};
use crate::spec::mark::line::LineMark;
use crate::spec::mark::{DataSource, Mark, MarkKind};
use crate::spec::scale::Scale;
use crate::spec::Dimensions;
use crate::{
  graph::{node::Operator, Graph},
  spec::data::DataEntry,
  Specification,
};

/// `Parser` allows to parse a certain `Specification` into a `Graph` representation, where
/// nodes are generated from the different specification parts, and inter-connected accordingly.
pub(crate) struct Parser;

impl Parser {
  /// Parse a specification instance into a new graph.
  pub(crate) fn parse(&self, specification: Specification) -> Graph {
    let mark_parser = MarkParser::new(&specification.scales, specification.dimensions);

    let mut graph = Graph::new();
    let data_nodes = DataParser::parse(specification.data, &mut graph);
    mark_parser.parse_marks(specification.marks, &data_nodes, &mut graph);

    graph
  }
}

struct DataParser;

impl DataParser {
  /// Parse the specification data into the graph by adding data and transform nodes.
  /// Return the leave indices of the nodes in the graph, identified by the data specification name.
  fn parse(data: Vec<DataEntry>, graph: &mut Graph) -> HashMap<String, usize> {
    data.into_iter().fold(HashMap::new(), |mut acc, entry| {
      let (identifier, node) = DataParser::parse_entry(entry, graph);
      acc.insert(identifier, node);
      acc
    })
  }

  fn parse_entry(entry: DataEntry, graph: &mut Graph) -> (String, usize) {
    let data_node = graph.add_node(Operator::data(entry.values));

    let node = entry.transform.into_iter().fold(data_node, |acc, pipe| {
      graph.add(Operator::transform(pipe), vec![acc])
    });

    (entry.name, node)
  }
}

struct MarkParser {
  scales: HashMap<String, Scale>,
  dimensions: Dimensions,
}

impl MarkParser {
  fn new(scales: &[Scale], dimensions: Dimensions) -> Self {
    let scales = scales
      .iter()
      .map(|scale| (scale.name.clone(), scale.clone()))
      .collect::<HashMap<String, Scale>>();

    MarkParser { scales, dimensions }
  }

  /// Parse the specification marks into the graph by creating mark and scale nodes, which are
  /// properly connected within each other and the incoming data node.
  fn parse_marks(
    &self,
    marks: Vec<Mark>,
    data_nodes: &HashMap<String, usize>,
    graph: &mut Graph,
  ) -> Vec<usize> {
    marks
      .into_iter()
      .filter_map(|mark| {
        data_nodes
          .get(&mark.from)
          .map(|data_node| self.parse_mark(mark, *data_node, graph))
      })
      .collect()
  }

  /// Parse a single mark by creating the referenced scale nodes and the needed graph edges with
  /// the data node.
  fn parse_mark(&self, mark: Mark, data_node: usize, graph: &mut Graph) -> usize {
    match mark.kind {
      MarkKind::Line(line_mark) => self.parse_line_mark(line_mark, data_node, graph),
    }
  }

  fn parse_line_mark(&self, mark: LineMark, data_node: usize, graph: &mut Graph) -> usize {
    let input_nodes = self.parse_mark_base_props(&mark.on.update.props.base, data_node, graph);

    let node = graph.add_node(Operator::line(
      mark,
      SceneWindow::new(self.dimensions.width, self.dimensions.height),
    ));

    for input_node in input_nodes {
      graph.add_edge(input_node, node)
    }

    node
  }

  fn parse_mark_base_props(
    &self,
    base: &BaseMarkProperties,
    data_node: usize,
    graph: &mut Graph,
  ) -> Vec<usize> {
    let mut scale_nodes = Vec::new();

    // Parse scale node for the "x" field
    if let Some(x_scale_node) = base
      .x
      .as_ref()
      .map(|x| self.parse_scale(x, X_AXIS_FIELD_NAME, data_node, graph))
    {
      scale_nodes.push(x_scale_node);
    }

    // Parse scale node for the "y" field
    if let Some(y_scale_node) = base
      .y
      .as_ref()
      .map(|y| self.parse_scale(y, Y_AXIS_FIELD_NAME, data_node, graph))
    {
      scale_nodes.push(y_scale_node);
    }

    // Parse scale node for the "width" field
    if let Some(width_scale_node) = base
      .width
      .as_ref()
      .map(|width| self.parse_scale(width, WIDTH_FIELD_NAME, data_node, graph))
    {
      scale_nodes.push(width_scale_node);
    }

    // Parse scale node for the "height" field
    if let Some(height_scale_node) = base
      .height
      .as_ref()
      .map(|height| self.parse_scale(height, HEIGHT_FIELD_NAME, data_node, graph))
    {
      scale_nodes.push(height_scale_node);
    }

    scale_nodes
  }

  /// Parse a scale for a certain mark's data source by creating a new scale node in the graph
  /// and connecting it to the incoming data node.
  fn parse_scale(
    &self,
    data_source: &DataSource,
    output: &str,
    data_node: usize,
    graph: &mut Graph,
  ) -> usize {
    let operator = match data_source {
      // Create a scale operator, if the mark's data source is from a data field
      DataSource::FieldSource { field, scale } => {
        let operator = scale
          .as_ref()
          .and_then(|scale_name| self.scales.get(scale_name))
          .cloned()
          .map(|scale| Operator::scale(scale, field, output))
          // If no scale is defined, then we copy the field's value in the output field.
          .unwrap_or(Operator::identity(field, output));

        operator
      }
      // Create a data operator if the mark's data source is plain data value
      DataSource::ValueSource(value) => {
        Operator::data(vec![DataValue::from_pairs(vec![(output, value.clone())])])
      }
    };

    // Create node in the graph and connect it to the incoming data node.
    graph.add(operator, vec![data_node])
  }
}

#[cfg(test)]
mod tests {
  use std::collections::{BTreeMap, HashSet};

  use crate::graph::node::mark::SceneWindow;
  use crate::graph::node::{Node, Operator};
  use crate::graph::Edge;
  use crate::spec::mark::line::LinePropertiesBuilder;
  use crate::spec::scale::ScaleKind;
  use crate::spec::transform::map::MapPipe;
  use crate::spec::Dimensions;
  use crate::{
    data::DataValue,
    spec::data::DataEntry,
    spec::mark::{line::LineMark, DataSource, Mark},
    spec::scale::{domain::Domain, linear::LinearScale, range::Range, Scale},
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
      vec![Mark::line(
        "primary",
        LineMark::new(
          LinePropertiesBuilder::new()
            .with_x(DataSource::field("a", Some("horizontal")))
            .with_y(DataSource::field("b", Some("vertical")))
            .build(),
        ),
      )],
    );
    let parser = Parser;

    // when
    let graph = parser.parse(spec);

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
        Node::init(Operator::linear(
          LinearScale::new(Domain::Literal(0.0, 100.0), Range::Literal(0.0, 20.0),),
          "a",
          "x"
        )),
        Node::init(Operator::linear(
          LinearScale::new(Domain::Literal(0.0, 100.0), Range::Literal(0.0, 10.0),),
          "b",
          "y"
        )),
        Node::init(Operator::line(
          LineMark::new(
            LinePropertiesBuilder::new()
              .with_x(DataSource::field("a", Some("horizontal")))
              .with_y(DataSource::field("b", Some("vertical")))
              .build()
          ),
          SceneWindow::new(500, 200),
        ))
      ]
    );
    assert_eq!(
      graph.edges,
      vec![
        Edge::new(0, 1),
        Edge::new(1, 2),
        Edge::new(2, 3),
        Edge::new(2, 4),
        Edge::new(3, 5),
        Edge::new(4, 5),
      ]
    );
    assert_eq!(
      graph.targets,
      BTreeMap::from([
        (0, HashSet::from([1])),
        (1, HashSet::from([2])),
        (2, HashSet::from([3, 4])),
        (3, HashSet::from([5])),
        (4, HashSet::from([5]))
      ])
    );
    assert_eq!(
      graph.sources,
      BTreeMap::from([
        (1, HashSet::from([0])),
        (2, HashSet::from([1])),
        (3, HashSet::from([2])),
        (4, HashSet::from([2])),
        (5, HashSet::from([3, 4]))
      ])
    );
    assert_eq!(
      graph.degrees,
      BTreeMap::from([(0, 0), (1, 1), (2, 1), (3, 1), (4, 1), (5, 2)])
    );
    assert_eq!(
      graph.nodes_in_degree,
      BTreeMap::from([
        (0, HashSet::from([0])),
        (1, HashSet::from([1, 2, 3, 4])),
        (2, HashSet::from([5]))
      ])
    );
    assert_eq!(graph.order, vec![0, 1, 2, 3, 4]);
  }
}
