use std::collections::HashMap;

use crate::{
  data::DataEntry,
  graph::{node::Operator, Graph},
  Specification,
};
use crate::scale::Scale;

struct Parser {}

impl Parser {
  pub fn parse(&self, specification: Specification) -> Graph {
    let mut graph = Graph::new();

    let data_identifiers = self.parse_data(specification.data, &mut graph);

    graph
  }

  fn parse_data(&self, data: Vec<DataEntry>, graph: &mut Graph) -> HashMap<String, usize> {
    data.into_iter().fold(HashMap::new(), |mut acc, entry| {
      let (identifier, node) = self.parse_entry(entry, graph);
      acc.insert(identifier, node);
      acc
    })
  }

  fn parse_entry(&self, entry: DataEntry, graph: &mut Graph) -> (String, usize) {
    let data_node = graph.add_node(Operator::data(entry.values));

    let node = entry.transform.into_iter().fold(data_node, |acc, pipe| {
      graph.add(Operator::transform(pipe), vec![acc])
    });

    (entry.name.to_string(), node)
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
        "valid",
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
