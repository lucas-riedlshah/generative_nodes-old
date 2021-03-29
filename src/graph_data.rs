use std::collections::{HashMap};
use std::sync::Arc;

use druid::{
  // im::{HashMap, Vector},
  Data,
};

use crate::node_data::NodeData;

#[derive(Clone, Data)]
pub struct GraphData {
  nodes: Arc<Vec<NodeData>>,
  edges: Arc<Vec<(usize, &'static str, usize, &'static str)>>
}

impl GraphData {
  pub fn new() -> Self {
    GraphData {
      nodes: Arc::new(Vec::new()),
      edges: Arc::new(Vec::new()),
    }
  }

  pub fn get_nodes(&self) -> &Vec<NodeData> {
    &self.nodes
  }

  pub fn get_edges(&self) -> &Vec<(usize, &'static str, usize, &'static str)> {
    &self.edges
  }

  pub fn get_nodes_mut(&mut self) -> &mut Vec<NodeData> {
    Arc::make_mut(&mut self.nodes)
  }

  pub fn get_edges_mut(&mut self) -> &mut Vec<(usize, &'static str, usize, &'static str)> {
    Arc::make_mut(&mut self.edges)
  }
}