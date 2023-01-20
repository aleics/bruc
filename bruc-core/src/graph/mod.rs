use crate::data::DataValue;
use crate::graph::node::{Node, Operator};
use crate::scene::{Scenegraph, SceneGroup, SceneItem};
use std::collections::{BTreeMap, HashSet, VecDeque};
use std::iter::FromIterator;

pub mod node;

/// `graph-pulse` is a library to build graphs that can be evaluated in topological order,
/// where a Pulse instance is being used to collect the data being passed between nodes.
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
  pub fn new() -> Self {
    Default::default()
  }

  pub fn get_node(&self, id: usize) -> Option<&Node> {
    self.nodes.iter().find(|node| node.id == id)
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

  // Add root node with connections to existing target nodes
  pub fn add_root(&mut self, operator: Operator, targets: Vec<usize>) -> usize {
    let id = self.add_node(operator);

    targets
      .into_iter()
      .for_each(|target| self.add_edge(id, target));

    self.order = self.sort_order();

    id
  }

  /// Add single node without any connections
  pub fn add_node(&mut self, operator: Operator) -> usize {
    let id = self.nodes.len();

    self.nodes.push(Node::init(id, operator));
    self.degrees.insert(id, 0);
    self.nodes_in_degree.insert(0, HashSet::from_iter([id]));

    id
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

  fn sort_order(&self) -> Vec<usize> {
    // Copy the current graph's node degree distribution
    let mut current_degrees = self.degrees.clone();

    // Start sorting the graph from the root nodes (no incoming edge).
    let mut queue = VecDeque::from_iter(
      current_degrees
        .iter()
        .filter_map(|(node, degree)| (*degree == 0).then_some(*node)),
    );

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

  fn get_nodes_in_degree(&self, degree: usize) -> HashSet<usize> {
    self
      .nodes_in_degree
      .get(&degree)
      .cloned()
      .unwrap_or(HashSet::new())
  }

  /// Return the output nodes indices in the graph.
  fn outputs(&self) -> Vec<&Node> {
    self
      .sources
      .keys()
      .filter(|node| !self.targets.contains_key(node))
      .flat_map(|node| self.get_node(*node))
      .collect()
  }

  /// Builds the `Scenegraph` instance by evaluating the full tree once, and building the scene
  /// items out of the pulse value of the outputs.
  pub async fn build(&mut self) -> Scenegraph {
    let outputs = self.evaluate().await;

    let items = outputs.into_iter()
      .flat_map(|node| SceneItem::build(node))
      .collect();

    Scenegraph::new(SceneGroup::with_items(items))
  }

  /// Evaluates the current graph iterating through all the edges of the graph in topological
  /// order, and keeps track of the values by using `Pulse` instances. Once the evaluation
  /// has completed, it returns the indices of the output nodes.
  async fn evaluate(&mut self) -> Vec<&Node> {
    // Start evaluating the graph from the root nodes
    let mut queue = VecDeque::from_iter(self.get_nodes_in_degree(0));

    while let Some(index) = queue.pop_front() {
      self.evaluate_node(index).await;

      // Append the targets to the queue
      if let Some(targets) = self.targets.get(&index) {
        queue.extend(targets.iter());
      }
    }

    self.outputs()
  }

  async fn evaluate_tree(&mut self, node_id: usize) {
    let mut queue = VecDeque::new();

    let node_index = self
      .nodes
      .iter()
      .position(|node| node.id == node_id)
      .unwrap();

    queue.push_back(node_index);

    while let Some(index) = queue.pop_front() {
      self.evaluate_node(index).await;

      // Append the targets to the queue
      if let Some(targets) = self.targets.get(&index) {
        queue.extend(targets.iter());
      }
    }
  }

  async fn evaluate_node(&mut self, index: usize) {
    let pulse = self.get_pulse(&index).unwrap_or(Pulse::init());

    // Run the pulse against the node
    let node = self.nodes.get_mut(index).unwrap();
    node.execute(pulse).await;
  }

  /// Find the pulse instance of a given node index by finding the source node's current pulse.
  fn get_pulse(&self, index: &usize) -> Option<Pulse> {
    let source_indices = self.sources.get(index)?;

    // Collect the pulses from all the source nodes and merge them together into a multi-pulse
    let source_pulses: Vec<Pulse> = source_indices
      .iter()
      .flat_map(|source_index| self.nodes.get(*source_index))
      .map(|node| node.pulse.clone())
      .collect();

    Some(Pulse::merge(source_pulses))
  }
}

trait Evaluation {
  async fn evaluate(&self, pulse: Pulse) -> Pulse {
    match pulse {
      Pulse::Single(single) => self.evaluate_single(single),
      Pulse::Multi(multi) => self.evaluate_multi(multi),
    }
  }

  fn evaluate_single(&self, single: SinglePulse) -> Pulse;

  fn evaluate_multi(&self, multi: MultiPulse) -> Pulse;
}

#[derive(Debug, PartialEq)]
pub struct Edge {
  from: usize,
  to: usize,
}

impl Edge {
  pub fn new(from: usize, to: usize) -> Self {
    Edge { from, to }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Pulse {
  Single(SinglePulse),
  Multi(MultiPulse),
}

impl Pulse {
  pub fn multi(pulses: Vec<SinglePulse>) -> Self {
    Pulse::Multi(MultiPulse::new(pulses))
  }

  pub fn single(values: Vec<PulseValue>) -> Self {
    Pulse::Single(SinglePulse::new(values))
  }

  pub fn init() -> Self {
    Pulse::Single(SinglePulse::new(Vec::new()))
  }

  pub fn merge(pulses: Vec<Pulse>) -> Self {
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

#[derive(Debug, Clone, PartialEq)]
pub struct SinglePulse {
  pub(crate) values: Vec<PulseValue>,
}

impl SinglePulse {
  pub fn new(values: Vec<PulseValue>) -> SinglePulse {
    SinglePulse { values }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PulseValue {
  Data(DataValue),
  Marks(SceneItem),
}

impl PulseValue {
  pub(crate) fn get_data(&self) -> Option<&DataValue> {
    if let PulseValue::Data(data) = &self {
      Some(data)
    } else {
      None
    }
  }

  pub(crate) fn get_marks(&self) -> Option<&SceneItem> {
    if let PulseValue::Marks(scene) = &self {
      Some(scene)
    } else {
      None
    }
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MultiPulse {
  pub(crate) pulses: Vec<SinglePulse>,
}

impl MultiPulse {
  pub fn new(pulses: Vec<SinglePulse>) -> Self {
    MultiPulse { pulses }
  }
}

#[cfg(test)]
mod tests {
  use crate::{
    data::DataValue,
    spec::transform::{filter::FilterPipe, map::MapPipe},
  };

  use super::*;

  #[tokio::test]
  async fn evaluates_in_topological_sort() {
    let mut graph = Graph::new();

    let first_data = graph.add_node(Operator::data(vec![DataValue::from_pairs(vec![(
      "a",
      2.0.into(),
    )])]));

    let second_data = graph.add_node(Operator::data(vec![DataValue::from_pairs(vec![(
      "a",
      1.0.into(),
    )])]));

    let map = graph.add(
      Operator::map(MapPipe::new("a + 3", "b").unwrap()),
      vec![first_data, second_data],
    );

    let filter = graph.add(
      Operator::filter(FilterPipe::new("b > 4").unwrap()),
      vec![map],
    );

    let outputs = graph.evaluate().await;

    assert_eq!(outputs.len(), 1);
    assert_eq!(
      outputs[0].pulse,
      Pulse::single(vec![PulseValue::Data(DataValue::from_pairs(vec![
        ("a", 2.0.into()),
        ("b", 5.0.into())
      ]))])
    );
  }
}
