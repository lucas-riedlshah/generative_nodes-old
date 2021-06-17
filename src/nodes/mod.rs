mod circle_node;
pub mod common;
mod particle_node;
mod value_node;
mod vector_node;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

use druid::Widget;

use crate::core::{App, Cache, Node};

pub fn node_widget_factories() -> HashMap<&'static str, fn(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>>> {
    let mut factories = HashMap::<&str, fn(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>>>::new();
    factories.insert("Value", value_node::widget_factory);
    factories.insert("Vector2D", vector_node::widget_factory);
    factories.insert("Particle", particle_node::widget_factory);
    factories.insert("Circle", circle_node::widget_factory);
    factories
}

pub fn node_factories() -> HashMap<&'static str, fn(&mut Cache) -> Node> {
    let mut factories = HashMap::<&'static str, fn(&mut Cache) -> Node>::new();
    factories.insert("Value", value_node::node_factory);
    factories.insert("Vector2D", vector_node::node_factory);
    factories.insert("Particle", particle_node::node_factory);
    factories.insert("Circle", circle_node::node_factory);
    factories
}
