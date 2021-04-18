use std::rc::Rc;

use crate::core::{AllocatedVec, App, Edge, Node};

pub trait Graph {
    fn get_edges(&self) -> &Vec<Edge>;
}

impl Graph for Rc<App> {
    fn get_edges(&self) -> &Vec<Edge> {
        &self.get_edges()
    }
}
