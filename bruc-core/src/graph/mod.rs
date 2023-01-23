use crate::data::DataValue;
use crate::graph::node::{Node, Operator};
use crate::scene::{SceneGroup, SceneItem, Scenegraph};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::iter::FromIterator;

pub mod node;

/// `Graph` represent a distribution of nodes connected with each other. Nodes are sorted
/// in topological order. The graph can be evaluated by passing data from the roots into the leaves,
/// where a `Pulse` instance is being used to collect the data being passed between nodes.
#[derive(Debug, Default, PartialEq)]
pub struct Graph {
  /// List of nodes of the graph.
  pub(crate) nodes: Vec<Node>,

  /// List of edges of the graph
  pub(crate) edges: Vec<Edge>,

  /// Map associating the node index in the graph and their target nodes.
  pub(crate) targets: BTreeMap<usize, HashSet<usize>>,

  /// Map associating the node index in the graph and their source nodes.
  pub(crate) sources: BTreeMap<usize, HashSet<usize>>,

  /// Node's degree distribution, defining the degree of a node as the amount of incoming
  /// edges connected to a certain node. Key is the node index in the graph. Value is
  /// the degree count of the node.
  pub(crate) degrees: BTreeMap<usize, usize>,

  /// Map associating a node degree with the respective node indices in the graph.
  pub(crate) nodes_in_degree: BTreeMap<usize, HashSet<usize>>,

  /// The topological order of the graph nodes.
  pub(crate) order: Vec<usize>,
}

impl Graph {
  /// Create a new `Graph` instance with no nodes.
  pub fn new() -> Self {
    Graph::default()
  }

  /// Add node with connections to existing source nodes
  pub fn add(&mut self, operator: Operator, sources: Vec<usize>) -> usize {
    let id = self.add_node(operator);

    sources
      .into_iter()
      .for_each(|source| self.add_edge(source, id));

    self.order = self.sort_order();

    id
  }

  /// Add single node without any connections
  pub fn add_node(&mut self, operator: Operator) -> usize {
    let index = self.nodes.len();

    self.nodes.push(Node::init(operator));
    self.degrees.insert(index, 0);
    self.nodes_in_degree.insert(0, HashSet::from_iter([index]));

    index
  }

  // Add edge between to nodes existent in the graph
  pub fn add_edge(&mut self, from: usize, to: usize) {
    let previous_degree = self.degrees.entry(to).or_insert(0);
    *previous_degree += 1;

    self.nodes_in_degree = self
      .degrees
      .iter()
      .fold(BTreeMap::new(), |mut acc, (node, degree)| {
        acc.entry(*degree).or_insert(HashSet::new()).insert(*node);
        acc
      });

    let target = self.targets.entry(from).or_insert(HashSet::new());
    target.insert(to);

    let source = self.sources.entry(to).or_insert(HashSet::new());
    source.insert(from);

    self.edges.push(Edge::new(from, to));
  }

  /// Sort the graph in topological order and return the order list of the nodes.
  fn sort_order(&self) -> Vec<usize> {
    // Copy the current graph's node degree distribution
    let mut current_degrees = self.degrees.clone();

    // Start sorting the graph from the root nodes (no incoming edge).
    let mut queue = current_degrees
      .iter()
      .filter_map(|(node, degree)| (*degree == 0).then_some(*node))
      .collect::<VecDeque<_>>();

    let mut order = Vec::new();

    // For a given root node in the queue, read all its connected nodes. Add an node
    // if no more incoming edges are found.
    while let Some(node) = queue.pop_front() {
      order.push(node);

      // Decrease degree of the nodes with incoming connections to the root node
      for edge in &self.edges {
        if edge.from == node {
          let target_degree = current_degrees.get_mut(&edge.to).unwrap();
          *target_degree -= 1;
          if *target_degree == 0 {
            queue.push_back(edge.to);
          }
        }
      }
    }

    order
  }

  /// Get node indices that have a certain degree.
  fn get_nodes_in_degree(&self, degree: usize) -> HashSet<usize> {
    self
      .nodes_in_degree
      .get(&degree)
      .cloned()
      .unwrap_or(HashSet::new())
  }

  /// Return the leave nodes in the graph.
  fn leaves(&self) -> Vec<&Node> {
    self
      .sources
      .keys()
      .filter(|node| !self.targets.contains_key(node))
      .filter_map(|node| self.nodes.get(*node))
      .collect()
  }

  /// Builds the `Scenegraph` instance by evaluating the full tree once, and building the scene
  /// items out of the pulse value of the outputs.
  pub async fn build(&mut self) -> Scenegraph {
    let outputs = self.evaluate().await;

    let items = outputs.into_iter().filter_map(SceneItem::build).collect();

    Scenegraph::new(SceneGroup::with_items(items))
  }

  /// Evaluates the current graph iterating through all the edges of the graph in topological
  /// order, and keeps track of the values by using `Pulse` instances. Once the evaluation
  /// has completed, it returns the leave nodes.
  pub async fn evaluate(&mut self) -> Vec<&Node> {
    // Start evaluating the graph from the root nodes
    let mut queue = VecDeque::from_iter(self.get_nodes_in_degree(0));

    while let Some(index) = queue.pop_front() {
      self.evaluate_node(index).await;

      // Append the targets to the queue
      if let Some(targets) = self.targets.get(&index) {
        queue.extend(targets.iter());
      }
    }

    self.leaves()
  }

  /// Evaluate a single node of a given index in the graph.
  async fn evaluate_node(&mut self, index: usize) {
    let pulse = self.get_pulse(index).unwrap_or(Pulse::init());

    // Run the pulse against the node
    let node = self.nodes.get_mut(index).unwrap();
    node.execute(pulse).await;
  }

  /// Find the pulse instance of a given node index by finding the source node's current pulse.
  fn get_pulse(&self, index: usize) -> Option<Pulse> {
    let source_indices = self.sources.get(&index)?;

    // Collect the pulses from all the source nodes and merge them together into a multi-pulse
    let source_pulses: Vec<Pulse> = source_indices
      .iter()
      .filter_map(|source_index| self.nodes.get(*source_index))
      .map(|node| node.pulse.clone())
      .collect();

    Some(Pulse::merge(source_pulses))
  }
}

/// `Evaluation` represents the ability to evaluate a certain `Pulse` instance, by returning a new
/// one. All nodes in the `Graph` are required to be evaluated, so that changes in the input nodes
/// are propagated through the graph.
trait Evaluation {
  /// Evaluates a `Pulse` instance.
  async fn evaluate(&self, pulse: Pulse) -> Pulse {
    match pulse {
      Pulse::Single(single) => self.evaluate_single(single),
      Pulse::Multi(multi) => self.evaluate_multi(multi),
    }
  }

  /// Evaluates a single `Pulse` instance
  fn evaluate_single(&self, single: SinglePulse) -> Pulse;

  /// Evaluates a multi `Pulse` instance.
  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse;
}

/// `Edge` represents an edge between two nodes in the graph
#[derive(Debug, PartialEq)]
pub(crate) struct Edge {
  pub(crate) from: usize,
  pub(crate) to: usize,
}

impl Edge {
  /// Create a new `Edge`
  pub(crate) fn new(from: usize, to: usize) -> Self {
    Edge { from, to }
  }
}

/// `Pulse` represents the current state of a node in the graph for a certain evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum Pulse {
  /// A single `Pulse` represents a single state.
  Single(SinglePulse),

  /// A multi `Pulse` represents multiple instances of a `Pulse` collected together.
  /// Multi pulses occur while evaluating nodes that have multiple source nodes connected to.
  Multi(MultiPulse),
}

impl Pulse {
  /// Create a new multi `Pulse` given a collection of single pulses
  pub(crate) fn multi(pulses: Vec<SinglePulse>) -> Self {
    Pulse::Multi(MultiPulse::new(pulses))
  }

  /// Create a new single `Pulse` with certain values.
  pub fn single(values: Vec<PulseValue>) -> Self {
    Pulse::Single(SinglePulse::new(values))
  }

  /// Initialize an empty single `Pulse` instance.
  pub(crate) fn init() -> Self {
    Pulse::Single(SinglePulse::new(Vec::new()))
  }

  /// Merge a collection of pulses together so that a multi `Pulse` instance is returned merging
  /// all internal single pulses together
  pub(crate) fn merge(pulses: Vec<Pulse>) -> Self {
    let value = pulses
      .into_iter()
      .flat_map(|pulse| match pulse {
        Pulse::Multi(multi) => multi.pulses,
        Pulse::Single(single) => vec![single],
      })
      .collect();

    Pulse::multi(value)
  }
}

/// `SinglePulse` represents a type of `Pulse` with a single state instance, represented by a list
/// of values.
#[derive(Debug, Clone, PartialEq)]
pub struct SinglePulse {
  pub(crate) values: Vec<PulseValue>,
}

impl SinglePulse {
  /// Create a new `SinglePulse` instance.
  pub(crate) fn new(values: Vec<PulseValue>) -> SinglePulse {
    SinglePulse { values }
  }
}

/// `MultiPulse` represents a type of `Pulse` with a number of `SinglePulse` instances.
#[derive(Debug, Clone, PartialEq)]
pub struct MultiPulse {
  pub(crate) pulses: Vec<SinglePulse>,
}

impl MultiPulse {
  /// Create a new `MultiPulse` instance.
  pub fn new(pulses: Vec<SinglePulse>) -> Self {
    MultiPulse { pulses }
  }
}

/// `PulseValue` describes the type of values that are expected to be propagated through the graph
/// during evaluation.
#[derive(Debug, Clone, PartialEq)]
pub enum PulseValue {
  /// `DataValue` pulse value.
  Data(DataValue),
  /// `SceneItem` pulse value.
  Marks(SceneItem),
}

impl PulseValue {
  /// Get the pulse value as `DataValue`
  pub(crate) fn get_data(&self) -> Option<&DataValue> {
    if let PulseValue::Data(data) = &self {
      Some(data)
    } else {
      None
    }
  }

  /// Get the pulse value as `SceneItem`
  pub(crate) fn get_marks(&self) -> Option<&SceneItem> {
    if let PulseValue::Marks(scene) = &self {
      Some(scene)
    } else {
      None
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::spec::mark::line::{Interpolate, LineMark, LineMarkProperties};
  use crate::spec::mark::DataSource;
  use crate::spec::scale::domain::Domain;
  use crate::spec::scale::linear::LinearScale;
  use crate::spec::scale::range::Range;
  use crate::{
    data::DataValue,
    spec::transform::{filter::FilterPipe, map::MapPipe},
  };

  use super::*;

  fn graph() -> Graph {
    let mut graph = Graph::new();

    let first_data = graph.add_node(Operator::data(vec![
      DataValue::from_pairs(vec![("a", 5.0.into())]),
      DataValue::from_pairs(vec![("a", 13.0.into())]),
    ]));

    let second_data = graph.add_node(Operator::data(vec![DataValue::from_pairs(vec![(
      "a",
      2.0.into(),
    )])]));

    let map = graph.add(
      Operator::map(MapPipe::new("a + 2", "b").unwrap()),
      vec![first_data, second_data],
    );

    let filter = graph.add(
      Operator::filter(FilterPipe::new("b > 4").unwrap()),
      vec![map],
    );

    let x_scale = graph.add(
      Operator::linear(
        LinearScale::new(Domain::Literal(0.0, 100.0), Range::Literal(0.0, 20.0)),
        "a",
        "x",
      ),
      vec![filter],
    );

    let y_scale = graph.add(
      Operator::linear(
        LinearScale::new(Domain::Literal(0.0, 100.0), Range::Literal(0.0, 10.0)),
        "b",
        "y",
      ),
      vec![filter],
    );

    graph.add(
      Operator::line(LineMark::new(LineMarkProperties::new(
        Some(DataSource::field("a", Some("horizontal"))),
        Some(DataSource::field("b", Some("vertical"))),
        None,
        None,
        Interpolate::Linear,
      ))),
      vec![x_scale, y_scale],
    );

    graph
  }

  #[tokio::test]
  async fn evaluates_in_topological_sort() {
    let mut graph = graph();

    let outputs = graph.evaluate().await;

    assert_eq!(outputs.len(), 1);
    assert_eq!(
      outputs[0].pulse,
      Pulse::single(vec![PulseValue::Marks(SceneItem::line(
        (1.0, 0.7),
        (2.6, 1.5),
        "black",
        1.0
      ))])
    );
  }

  #[tokio::test]
  async fn builds_scenegraph() {
    let mut graph = graph();

    let scenegraph = graph.build().await;

    assert_eq!(
      scenegraph,
      Scenegraph::new(SceneGroup::with_items(vec![SceneItem::group(vec![
        SceneItem::line((1.0, 0.7), (2.6, 1.5), "black", 1.0)
      ])]))
    );
  }
}
