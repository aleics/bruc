use std::collections::HashMap;
use std::iter::FromIterator;

use crate::data::DataValue;
use crate::mark::base::{
  HEIGHT_FIELD_NAME, WIDTH_FIELD_NAME, X_AXIS_FIELD_NAME, Y_AXIS_FIELD_NAME,
};
use crate::mark::line::LineMark;
use crate::mark::{DataSource, Mark, MarkKind};
use crate::scale::Scale;
use crate::{
  data::DataEntry,
  graph::{node::Operator, Graph},
  Specification,
};

struct Parser;

impl Parser {
  pub fn parse(&self, specification: Specification) -> Graph {
    let mut graph = Graph::new();

    let data_nodes = self.parse_data(specification.data, &mut graph);
    let mark_nodes = self.parse_marks(
      specification.marks,
      specification.scales,
      data_nodes,
      &mut graph,
    );

    graph
  }

  fn parse_data(&self, data: Vec<DataEntry>, graph: &mut Graph) -> HashMap<String, usize> {
    data.into_iter().fold(HashMap::new(), |mut acc, entry| {
      let (identifier, node) = self.parse_data_entry(entry, graph);
      acc.insert(identifier, node);
      acc
    })
  }

  fn parse_data_entry(&self, entry: DataEntry, graph: &mut Graph) -> (String, usize) {
    let data_node = graph.add_node(Operator::data(entry.values));

    let node = entry.transform.into_iter().fold(data_node, |acc, pipe| {
      graph.add(Operator::transform(pipe), vec![acc])
    });

    (entry.name, node)
  }
  fn parse_marks(
    &self,
    marks: Vec<Mark>,
    scales: Vec<Scale>,
    data_nodes: HashMap<String, usize>,
    graph: &mut Graph,
  ) -> Vec<usize> {
    let scales_map = HashMap::<String, Scale>::from_iter(
      scales
        .iter()
        .map(|scale| (scale.name.clone(), scale.clone())),
    );

    let mut nodes = Vec::with_capacity(marks.len());
    for mark in marks {
      if let Some(data_node) = data_nodes.get(&mark.from) {
        nodes.push(self.parse_mark(mark, &scales_map, *data_node, graph));
      }
    }
    nodes
  }

  fn parse_mark(
    &self,
    mark: Mark,
    scales: &HashMap<String, Scale>,
    data_node: usize,
    graph: &mut Graph,
  ) -> usize {
    match mark.kind {
      MarkKind::Line(line_mark) => self.parse_line_mark(line_mark, scales, data_node, graph),
    }
  }

  fn parse_line_mark(
    &self,
    mark: LineMark,
    scales: &HashMap<String, Scale>,
    data_node: usize,
    graph: &mut Graph,
  ) -> usize {
    let x_scale_node = mark
      .on
      .update
      .props
      .base
      .x
      .as_ref()
      .map(|x| self.parse_scale(x, X_AXIS_FIELD_NAME, scales, data_node, graph));
    let y_scale_node = mark
      .on
      .update
      .props
      .base
      .y
      .as_ref()
      .map(|y| self.parse_scale(y, Y_AXIS_FIELD_NAME, scales, data_node, graph));
    let width_scale_node = mark
      .on
      .update
      .props
      .base
      .width
      .as_ref()
      .map(|width| self.parse_scale(width, WIDTH_FIELD_NAME, scales, data_node, graph));
    let height_scale_node = mark
      .on
      .update
      .props
      .base
      .height
      .as_ref()
      .map(|height| self.parse_scale(height, HEIGHT_FIELD_NAME, scales, data_node, graph));

    let node = graph.add_node(Operator::line(mark));

    if let Some(x) = x_scale_node {
      graph.add_edge(x, node)
    }
    if let Some(y) = y_scale_node {
      graph.add_edge(y, node)
    }
    if let Some(width) = width_scale_node {
      graph.add_edge(width, node)
    }
    if let Some(height) = height_scale_node {
      graph.add_edge(height, node)
    }

    node
  }

  fn parse_scale(
    &self,
    data_source: &DataSource,
    output: &str,
    scales: &HashMap<String, Scale>,
    data_node: usize,
    graph: &mut Graph,
  ) -> usize {
    let operator = match data_source {
      DataSource::FieldSource { field, scale } => {
        let operator = scale
          .as_ref()
          .and_then(|scale_name| scales.get(scale_name))
          .cloned()
          .map(|scale| Operator::scale(scale, field, output))
          // If no scale is defined, then we copy the field's value in the output field.
          .unwrap_or(Operator::identity(field, output));

        operator
      }
      DataSource::ValueSource(value) => {
        Operator::data(vec![DataValue::from_pairs(vec![(output, *value)])])
      }
    };

    graph.add(operator, vec![data_node])
  }
}

#[cfg(test)]
mod tests {
  use super::Parser;
  use crate::scale::ScaleKind;
  use crate::transform::map::MapPipe;
  use crate::{
    data::{DataEntry, DataValue},
    mark::{
      line::{Interpolate, LineMark, LineMarkProperties},
      DataSource, Mark,
    },
    scale::{domain::Domain, linear::LinearScale, range::Range, Scale},
    transform::{filter::FilterPipe, pipe::Pipe},
    Specification,
  };

  #[test]
  fn parses_simple() {
    let spec: Specification = Specification {
      data: vec![DataEntry::new(
        "primary",
        vec![DataValue::from_pairs(vec![("a", 10.0.into())])],
        vec![
          Pipe::Map(MapPipe::new("a - 2", "b").unwrap()),
          Pipe::Filter(FilterPipe::new("b > 2").unwrap()),
        ],
      )],
      scales: vec![Scale::new(
        "horizontal",
        ScaleKind::Linear(LinearScale::new(
          Domain::Literal(0.0, 100.0),
          Range::Literal(0.0, 20.0),
        )),
      )],
      marks: vec![Mark::line(
        "primary",
        LineMark::new(LineMarkProperties::new(
          Some(DataSource::field("x", Some("horizontal"))),
          None,
          None,
          None,
          Interpolate::Linear,
        )),
      )],
    };

    let parser = Parser {};

    let graph = parser.parse(spec);
  }
}
