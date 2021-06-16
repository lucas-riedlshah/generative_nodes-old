use crate::core::{AllocatedVec, Cache, Node};

/// Used to determine the compute order of nodes.
#[derive(Clone)]
pub enum EdgeType {
    Normal,
    Post,
}

#[derive(Clone)]
pub struct Edge {
    pub(crate) from_node: usize,
    pub(crate) from_port: usize,
    pub(crate) to_node: usize,
    pub(crate) to_port: usize,
    pub(crate) edge_type: EdgeType,
}

pub struct App {
    cache: Cache,
    nodes: AllocatedVec<Node>,
    edges: Vec<Edge>,
    factories: Vec<fn(&mut Cache) -> Node>,
}

impl App {
    pub fn new() -> App {
        App {
            cache: Cache::new(),
            nodes: AllocatedVec::new(),
            edges: Vec::new(),
            factories: Vec::new(),
        }
    }

    pub fn with_factories(mut self, factories: Vec<fn(&mut Cache) -> Node>) -> Self {
        self.factories = factories;
        self
    }

    pub fn add_node(&mut self, node_factory_index: usize) -> usize {
        self.nodes
            .push((self.factories[node_factory_index])(&mut self.cache))
    }

    pub fn remove_node(&mut self, node_index: usize) {
        let mut removed_node = self.nodes.remove(node_index).unwrap();

        for index in (0..self.edges.len()).rev() {
            let edge = self.edges.get(index).unwrap();
            if edge.from_node == node_index || edge.to_node == node_index {
                self.remove_edge(index);
            }
        }

        removed_node.remove_all_cache(&mut self.cache);
    }

    pub fn get_node(&self, node_index: usize) -> &Node {
        &self.nodes.get(node_index).unwrap()
    }

    // TODO: Rewrite this and split it into separate and more specified private methods to make this code more readable.
    pub fn add_edge(&mut self, from_node: usize, from_port: usize, to_node: usize, to_port: usize) {
        let new_cache_index = self
            .nodes
            .get(from_node)
            .unwrap()
            .get_output(from_port)
            .unwrap()
            .clone();
        let node = self.nodes.get_mut(to_node).unwrap();

        if let Some(old_edge_index) = self
            .edges
            .iter()
            .position(|edge| edge.to_node == to_node && edge.to_port == to_port)
        {
            let edge = self.edges.remove(old_edge_index);
            node.disconnect_input(to_port, &mut self.cache);
        }

        node.connect_input(to_port, new_cache_index, &mut self.cache);

        let edge_type = if self
            .edges
            .iter()
            .any(|edge| edge.from_node == to_node && edge.to_node == from_node)
        {
            EdgeType::Post
        } else {
            EdgeType::Normal
        };

        self.edges.push(Edge {
            from_node,
            from_port,
            to_node,
            to_port,
            edge_type,
        });

        println!("new edge count: {}", self.edges.len());
    }

    fn remove_edge(&mut self, index: usize) {
        let edge = self.edges.remove(index);
        self.nodes
            .get_mut(edge.to_node)
            .unwrap()
            .disconnect_input(edge.to_port, &mut self.cache);
        println!("new edge count: {}", self.edges.len());
    }

    pub fn remove_edge_to(&mut self, to_node: usize, to_port: usize) {
        let edge_index = self
            .edges
            .iter()
            .position(|edge| edge.to_node == to_node && edge.to_port == to_port);

        if let Some(index) = edge_index {
            self.remove_edge(index)
        }
    }

    pub fn remove_edges_from(&mut self, from_node: usize, from_port: usize) {
        for index in (0..self.edges.len()).rev() {
            let edge = self.edges.get(index).unwrap();
            if edge.from_node == from_node && edge.from_port == from_port {
                self.remove_edge(index);
            }
        }
    }

    pub fn edges(&self) -> &Vec<Edge> {
        &self.edges
    }

    pub fn nodes(&self) -> &AllocatedVec<Node> {
        &self.nodes
    }

    pub fn get_cache(&self) -> &Cache {
        &self.cache
    }

    pub fn get_cache_mut(&mut self) -> &mut Cache {
        &mut self.cache
    }

    pub fn compute(&mut self) {
        for i in 0..self.nodes.len() {
            let node = self.nodes.get(i).unwrap();
            node.compute(&mut self.cache)
        }
    }
}
