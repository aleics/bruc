use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::iter::FromIterator;

use crate::graph::node::{Node, Operator};
use crate::scene::SceneItem;

use self::pulse::{MultiPulse, Pulse, SinglePulse};

pub mod node;
pub mod pulse;

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
    pub(crate) targets: BTreeMap<usize, BTreeSet<usize>>,

    /// Map associating the node index in the graph and their source nodes.
    pub(crate) sources: BTreeMap<usize, BTreeSet<usize>>,

    /// Node's degree distribution, defining the degree of a node as the amount of incoming
    /// edges connected to a certain node. Key is the node index in the graph. Value is
    /// the degree count of the node.
    pub(crate) degrees: BTreeMap<usize, usize>,

    /// Map associating a node degree with the respective node indices in the graph.
    pub(crate) nodes_in_degree: BTreeMap<usize, BTreeSet<usize>>,

    /// The topological order of the graph nodes.
    pub(crate) order: Vec<usize>,
}

impl Graph {
    /// Create a new `Graph` instance with no nodes.
    pub fn new() -> Self {
        Graph::default()
    }

    /// Replaces a node from the specified `index` for another node
    pub(crate) fn replace_node(&mut self, index: usize, new_node: Node) -> Option<Node> {
        if index < self.nodes.len() {
            Some(std::mem::replace(&mut self.nodes[index], new_node))
        } else {
            None
        }
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

        // Add node in degree zero
        self.nodes_in_degree
            .entry(0)
            .and_modify(|nodes| {
                nodes.insert(index);
            })
            .or_insert(BTreeSet::from_iter([index]));

        // Add node in sources & targets with no connection
        self.sources.insert(index, BTreeSet::new());
        self.targets.insert(index, BTreeSet::new());

        index
    }

    // Add edge between to nodes existent in the graph
    pub fn add_edge(&mut self, from: usize, to: usize) {
        let previous_degree = self.degrees.entry(to).or_insert(0);
        *previous_degree += 1;

        self.nodes_in_degree =
            self.degrees
                .iter()
                .fold(BTreeMap::new(), |mut acc, (node, degree)| {
                    acc.entry(*degree).or_default().insert(*node);
                    acc
                });

        let target = self.targets.entry(from).or_default();
        target.insert(to);

        let source = self.sources.entry(to).or_default();
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
    fn get_nodes_in_degree(&self, degree: usize) -> BTreeSet<usize> {
        self.nodes_in_degree
            .get(&degree)
            .cloned()
            .unwrap_or(BTreeSet::new())
    }

    /// Return the leave nodes in the graph.
    fn leaves(&self) -> Vec<&Node> {
        self.sources
            .keys()
            .filter(|node| {
                // Leave nodes are the ones that are not targeting any other node
                self.targets
                    .get(node)
                    .map(|node_targets| node_targets.is_empty())
                    .unwrap_or(false)
            })
            .filter_map(|node| self.nodes.get(*node))
            .collect()
    }

    /// Builds the scene items by evaluating the full tree once, and building the scene
    /// items out of the pulse value of the outputs.
    pub async fn build(&mut self) -> Vec<SceneItem> {
        let outputs = self.evaluate().await;

        outputs.into_iter().filter_map(SceneItem::build).collect()
    }

    /// Builds the scene item by evaluating a sub-tree starting with a `node` index and
    /// building the scene items out of the pulse value of the outputs.
    pub async fn build_tree(&mut self, node: usize) -> Vec<SceneItem> {
        let outputs = self.evaluate_tree(node).await;

        outputs.into_iter().filter_map(SceneItem::build).collect()
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

    /// Evaluates the sub-tree of the current graph starting from `node` index by iterating
    /// through all the edges of the graph in topological order, and keeps track of the values
    /// by using `Pulse` instances. Once the evaluation has completed, it returns the leave
    /// nodes.
    async fn evaluate_tree(&mut self, node: usize) -> Vec<&Node> {
        // Start evaluating the tree from the node
        let mut queue = VecDeque::from_iter([node]);

        while let Some(node) = queue.pop_front() {
            self.evaluate_node(node).await;

            // Append the targets to the queue
            if let Some(targets) = self.targets.get(&node) {
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
            Pulse::Single(single) => self.evaluate_single(single).await,
            Pulse::Multi(multi) => self.evaluate_multi(multi).await,
        }
    }

    /// Evaluates a single `Pulse` instance
    async fn evaluate_single(&self, single: SinglePulse) -> Pulse;

    /// Evaluates a multi `Pulse` instance.
    async fn evaluate_multi(&self, multi: MultiPulse) -> Pulse;
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

#[cfg(test)]
mod tests {
    use crate::graph::node::shape::SceneWindow;
    use crate::scale::Scale;
    use crate::spec::axis::{Axis, AxisOrientation};
    use crate::spec::scale::domain::Domain;

    use crate::spec::shape::line::{LinePropertiesBuilder, LineShape};
    use crate::spec::shape::DataSource;
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

        let x_domain = graph.add(
            Operator::domain_interval(Domain::Literal(vec![0.0, 20.0])),
            vec![filter],
        );

        let x_scale = graph.add(
            Operator::linear((0.0, 20.0), "a", "x"),
            vec![filter, x_domain],
        );

        let y_domain = graph.add(
            Operator::domain_interval(Domain::Literal(vec![0.0, 20.0])),
            vec![filter],
        );

        let y_scale = graph.add(
            Operator::linear((0.0, 20.0), "b", "y"),
            vec![filter, y_domain],
        );

        graph.add(
            Operator::line(
                LineShape::new(
                    LinePropertiesBuilder::new()
                        .with_x(DataSource::field("a", Some("horizontal")))
                        .with_y(DataSource::field("b", Some("vertical")))
                        .build(),
                ),
                SceneWindow::new(20, 20),
            ),
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
            Pulse::shapes(vec![SceneItem::line(
                vec![(5.0, 13.0), (13.0, 5.0)],
                "black".to_string(),
                1.0
            )])
        );
    }

    #[tokio::test]
    async fn evaluates_tree_in_topological_sort() {
        let mut graph = graph();

        graph.evaluate().await;

        let map = Node::init(Operator::map(MapPipe::new("a + 3", "b").unwrap()));
        graph.replace_node(2, map);

        let outputs = graph.evaluate_tree(2).await;

        assert_eq!(outputs.len(), 1);
        assert_eq!(
            outputs[0].pulse,
            Pulse::shapes(vec![SceneItem::line(
                vec![(5.0, 20.0 - 8.0), (13.0, 20.0 - 16.0), (2.0, 20.0 - 5.0)],
                "black".to_string(),
                1.0
            )])
        );
    }

    #[tokio::test]
    async fn builds_scenegraph() {
        let mut graph = graph();

        let scene_items = graph.build().await;

        assert_eq!(
            scene_items,
            vec![SceneItem::line(
                vec![(5.0, 13.0), (13.0, 5.0)],
                "black".to_string(),
                1.0
            )]
        );
    }

    #[tokio::test]
    async fn gets_leave_nodes() {
        let mut graph = Graph::new();

        let data_operator = Operator::data(vec![]);
        let map_operator = Operator::map(MapPipe::new("a + 2", "b").unwrap());
        let scale_operator = Operator::linear((0.0, 20.0), "a", "x");
        let axis_operator = Operator::axis(
            Axis::new("x", AxisOrientation::Left),
            Scale::linear((0.0, 2.0)),
            SceneWindow::new(500, 200),
        );

        // (data) -> (map) -> (scale)
        // (axis)
        let data = graph.add(data_operator, vec![]);
        let map = graph.add(map_operator, vec![data]);
        let _scale = graph.add(scale_operator, vec![map]);
        let _axis = graph.add(axis_operator, vec![]);

        let leave_nodes = graph.leaves();
        assert_eq!(
            leave_nodes,
            vec![
                &Node::init(Operator::linear((0.0, 20.0), "a", "x",)),
                &Node::init(Operator::axis(
                    Axis::new("x", AxisOrientation::Left),
                    Scale::linear((0.0, 2.0)),
                    SceneWindow::new(500, 200)
                ))
            ]
        );
    }
}
