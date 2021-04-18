mod circle_node;
pub mod common;
mod particle_node;
mod value_node;
mod vector_node;

use std::{cell::RefCell, rc::Rc};

use druid::Widget;

use crate::core::{App, Cache, Node};

pub fn node_widget_factories() -> Vec<fn(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>>> {
    vec![
        value_node::widget_factory,
        vector_node::widget_factory,
        particle_node::widget_factory,
        circle_node::widget_factory,
    ]
}

pub fn node_factories() -> Vec<fn(&mut Cache) -> Node> {
    vec![
        value_node::node_factory,
        vector_node::node_factory,
        particle_node::node_factory,
        circle_node::node_factory,
    ]
}
