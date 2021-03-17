use druid::{
  im::{HashMap, Vector},
  Data,
};

use crate::vertex_data::VertexData;

#[derive(Clone)]
pub struct GraphData {
  vertices: Vector<VertexData>,
  edges: Vector<(usize, &'static str, usize, &'static str)>,
  factories: HashMap<&'static str, fn() -> VertexData>,
}

impl Data for GraphData {
  fn same(&self, other: &Self) -> bool {
    self.vertices.same(&other.vertices) && self.edges.same(&other.edges)
  }
}

impl GraphData {
  pub fn new() -> Self {
    GraphData {
      vertices: Vector::new(),
      edges: Vector::new(),
      factories: HashMap::new(),
    }
  }

  pub fn get_vertices(&self) -> &Vector<VertexData> {
    &self.vertices
  }

  pub fn get_edges(&self) -> &Vector<(usize, &'static str, usize, &'static str)> {
    &self.edges
  }

  pub fn get_vertices_mut(&mut self) -> &mut Vector<VertexData> {
    &mut self.vertices
  }

  pub fn get_edges_mut(&mut self) -> &mut Vector<(usize, &'static str, usize, &'static str)> {
    &mut self.edges
  }
}