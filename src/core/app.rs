use crate::core::{Cache, Node, AllocatedVec};

/// Used to determine the compute order of nodes.
enum EdgeType {
    Normal,
    Post,
}

struct Edge {
    from_node: usize,
    from_port: usize,
    to_node: usize,
    to_port: usize,
    edge_type: EdgeType,
}

pub struct App {
    cache: Cache,
    nodes: AllocatedVec<Node>,
    // from_node, from_port, to_node, to_port
    edges: Vec<Edge>,
}

impl App {
    pub fn new() -> App {
        App {
            cache: Cache::new(),
            nodes: AllocatedVec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_factory: fn(cache: &mut Cache) -> Node) -> usize {
        self.nodes.push((node_factory)(&mut self.cache))
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

    pub fn add_edge(&mut self, from_node: usize, from_port: usize, to_node: usize, to_port: usize) {
        let new_cache_index = self.nodes.get(from_node).unwrap().get_output(from_port).unwrap().clone();
        self.nodes.get_mut(to_node).unwrap().connect_input(to_port, new_cache_index, &mut self.cache);

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
