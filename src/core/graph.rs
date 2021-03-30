use std::sync::Arc;

use druid::{
  Data,
};

use crate::core::node::Node;

#[derive(Clone, Data)]
pub struct Graph {
  nodes: Arc<Vec<Node>>,
  edges: Arc<Vec<(usize, &'static str, usize, &'static str)>>
}

impl Graph {
  pub fn new() -> Self {
    Graph {
      nodes: Arc::new(Vec::new()),
      edges: Arc::new(Vec::new()),
    }
  }

  pub fn get_nodes(&self) -> &Vec<Node> {
    &self.nodes
  }

  pub fn get_edges(&self) -> &Vec<(usize, &'static str, usize, &'static str)> {
    &self.edges
  }

  pub fn get_nodes_mut(&mut self) -> &mut Vec<Node> {
    Arc::make_mut(&mut self.nodes)
  }

  pub fn get_edges_mut(&mut self) -> &mut Vec<(usize, &'static str, usize, &'static str)> {
    Arc::make_mut(&mut self.edges)
  }
}