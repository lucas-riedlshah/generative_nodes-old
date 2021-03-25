use druid::{
  im::{HashMap, Vector},
  Data,
};

use crate::node_data::NodeData;

#[derive(Clone)]
pub struct GraphData {
  nodes: Vector<NodeData>,
  edges: Vector<(usize, &'static str, usize, &'static str)>,
  // values: HashMap<Port, Packet>, // This is the ECS for all "values" of all the nodes.
  factories: HashMap<&'static str, fn() -> NodeData>,
}

impl Data for GraphData {
  fn same(&self, other: &Self) -> bool {
    self.nodes.same(&other.nodes) && self.edges.same(&other.edges)
  }
}

impl GraphData {
  pub fn new() -> Self {
    GraphData {
      nodes: Vector::new(),
      edges: Vector::new(),
      factories: HashMap::new(),
    }
  }

  pub fn get_nodes(&self) -> &Vector<NodeData> {
    &self.nodes
  }

  pub fn get_edges(&self) -> &Vector<(usize, &'static str, usize, &'static str)> {
    &self.edges
  }

  pub fn get_nodes_mut(&mut self) -> &mut Vector<NodeData> {
    &mut self.nodes
  }

  pub fn get_edges_mut(&mut self) -> &mut Vector<(usize, &'static str, usize, &'static str)> {
    &mut self.edges
  }
}