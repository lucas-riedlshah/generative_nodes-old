use std::{cell::RefCell, rc::Rc};

use druid::{Color, Widget, WidgetExt, widget::{Container, CrossAxisAlignment, Flex, Label, Slider, TextBox}};
use nalgebra::Vector2;

use crate::{core::{App, Cache, CacheIndex, Direction, Node, Port}, gui::{cache_lens::CacheLens, graph_widget::PortDirection, node_widget::NodeWidget, port_widget::PortWidget}};

// Inputs
const X: usize = 0;
const Y: usize = 1;
// Outputs
const VECTOR: usize = 2;

pub fn node_factory(cache: &mut Cache) -> Node {
    let x = cache.insert(0.);
    let y = cache.insert(0.);
    let vector = cache.insert(Vector2::new(0., 0.));

    let mut ports = Vec::new();
    ports.push(Port::new(x, Direction::Input));
    ports.push(Port::new(y, Direction::Input));
    ports.push(Port::new(vector, Direction::Output));
    
    Node::new(ports, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(disconnect, connect)
}

fn compute(ports: &Vec<Port>, cache: &mut Cache) {
    cache.get_mut::<Vector2<f64>>(ports[VECTOR].get_cache_index()).unwrap().x =
        cache.get::<f64>(ports[X].get_cache_index()).unwrap().clone();
    cache.get_mut::<Vector2<f64>>(ports[VECTOR].get_cache_index()).unwrap().y =
        cache.get::<f64>(ports[Y].get_cache_index()).unwrap().clone();
}

fn connect(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        X | Y => cache.remove::<f64>(&node.get_ports()[port].get_cache_index()),
        _ => (),
    }
}

fn disconnect(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        0 => Some(cache.insert(0.)),
        1 => Some(cache.insert(0.)),
        _ => None,
    }
}

fn remove_all_cache(ports: &Vec<Port>, cache: &mut Cache) {
    cache.remove::<f64>(&ports[X].get_cache_index());
    cache.remove::<f64>(&ports[Y].get_cache_index());
    cache.remove::<Vector2<f64>>(&ports[VECTOR].get_cache_index());
}

pub fn widget_factory(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>> {
    Box::new(NodeWidget::new(
        Container::new(
            Flex::column()
                .with_child(Label::new("Vector2"))
                .with_spacer(5.)
                .with_child(
                    // Inputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(index, X, PortDirection::Input, PortWidget::F64))
                                .with_spacer(5.)
                                .with_child(Label::new("X"))
                                .with_child(Slider::new()
                                    .with_range(-5., 5.)
                                    .lens(CacheLens::<f64>::new(index, X)))
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(index, Y, PortDirection::Input, PortWidget::F64))
                                .with_spacer(5.)
                                .with_child(Label::new("Y"))
                                .with_child(Slider::new()
                                    .with_range(-5., 5.)
                                    .lens(CacheLens::<f64>::new(index, Y)))
                        )
                        .expand_width(),
                )
                .with_spacer(5.)
                .with_child(
                    // Outputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::End)
                        .with_child(
                            Flex::row()
                                .with_child(Label::new("Vector2"))
                                .with_spacer(5.)
                                .with_child(PortWidget::new(index, VECTOR, PortDirection::Output, PortWidget::VECTOR2F64)),
                        )
                        .with_spacer(5.)
                        .expand_width(),
                )
                .fix_width(200.)
                .padding(5.),
        )
        .rounded(10.)
        .background(Color::rgba8(50, 50, 50, 230))
        .border(Color::rgb8(25, 25, 25), 1.),
    ))
}
