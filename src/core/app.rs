use crate::core::{Cache, Node};

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
    nodes: Vec<Node>,
    // from_node, from_port, to_node, to_port
    edges: Vec<Edge>,
}

impl App {
    pub fn new() -> App {
        App {
            cache: Cache::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node_factory: fn(cache: &mut Cache) -> Node) {
        self.nodes.push((node_factory)(&mut self.cache));
    }

    pub fn remove_node(&mut self, node_index: usize) {
        let mut removed_node = self.nodes.remove(node_index);
        // need to handle removing edges to the removed node.
        removed_node.remove_all_cache(&mut self.cache);
    }

    pub fn add_edge(&mut self, from_node: usize, from_port: usize, to_node: usize, to_port: usize) {
        let new_cache_index = self.nodes[from_node].get_output(from_port).unwrap().clone();
        self.nodes[to_node].connect_input(to_port, new_cache_index, &mut self.cache);

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

    fn remove_edge(&mut self, index: usize, to_node: usize, to_port: usize) {
        self.nodes
            .get_mut(to_node)
            .unwrap()
            .disconnect_input(to_port, &mut self.cache);
        self.edges.remove(index);
        println!("new edge count: {}", self.edges.len());
    }

    pub fn remove_edge_to(&mut self, to_node: usize, to_port: usize) {
        let edge_index = self
            .edges
            .iter()
            .position(|edge| edge.to_node == to_node && edge.to_port == to_port);

        if let Some(index) = edge_index {
            self.remove_edge(index, to_node, to_port)
        }
    }

    pub fn remove_edges_from(&mut self, from_node: usize, from_port: usize) {
        for index in (0..self.edges.len()).rev() {
            let edge = self.edges.get(index).unwrap();
            if edge.from_node == from_node && edge.from_port == from_port {
                self.remove_edge(index, edge.to_node, edge.to_port);
            }
        }
    }

    pub fn compute(&mut self) {
        for node in self.nodes.iter() {
            node.compute(&mut self.cache)
        }
    }
}
