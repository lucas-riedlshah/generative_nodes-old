use std::{cell::RefCell, rc::Rc};

use druid::{
    widget::{Container, CrossAxisAlignment, Flex, Label},
    Color, Widget, WidgetExt,
};
use nalgebra::Vector2;

use crate::{
    core::{App, Cache, CacheIndex, Node, Port, Direction},
    gui::{graph_widget::PortDirection, node_widget::NodeWidget, port_widget::PortWidget},
};

use super::common::shapes::Circle;

// Inputs
const POSITION: usize = 0;
const RADIUS: usize = 1;
// Outputs
const CIRCLE: usize = 2;

pub fn node_factory(cache: &mut Cache) -> Node {
    let position = cache.insert(Vector2::new(0., 0.));
    let radius = cache.insert(5.);
    let circle = cache.insert(Circle::new(Vector2::new(0., 0.), 5.));

    let mut ports = Vec::new();
    ports.push(Port::new(position, Direction::Input));
    ports.push(Port::new(radius, Direction::Input));
    ports.push(Port::new(circle, Direction::Output));

    Node::new(ports, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(disconnect, connect)
}

fn compute(ports: &Vec<Port>, cache: &mut Cache) {
    let position = *cache.get::<Vector2<f64>>(&ports[POSITION].get_cache_index()).unwrap();
    let radius = *cache.get::<f64>(&ports[RADIUS].get_cache_index()).unwrap();
    let circle = cache.get_mut::<Circle>(&ports[CIRCLE].get_cache_index()).unwrap();
    circle.set_position(position);
    circle.set_radius(radius);
}

fn connect(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        POSITION => cache.remove::<Vector2<f64>>(&node.get_ports()[POSITION].get_cache_index()),
        RADIUS => cache.remove::<f64>(&node.get_ports()[RADIUS].get_cache_index()),
        _ => (),
    }
}

fn disconnect(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        POSITION => Some(cache.insert(Vector2::new(0., 0.))),
        RADIUS => Some(cache.insert(5.)),
        _ => None,
    }
}

fn remove_all_cache(ports: &Vec<Port>, cache: &mut Cache) {
    cache.remove::<Vector2<f64>>(&ports[POSITION].get_cache_index());
    cache.remove::<f64>(&ports[RADIUS].get_cache_index());
    cache.remove::<Circle>(&ports[CIRCLE].get_cache_index());
}

pub fn widget_factory(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>> {
    Box::new(NodeWidget::new(
        Container::new(
            Flex::column()
                .with_child(Label::new("Circle"))
                .with_spacer(5.)
                .with_child(
                    // Inputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(index, POSITION, PortDirection::Input, PortWidget::VECTOR2F64))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Position")),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(index, RADIUS, PortDirection::Input, PortWidget::F64))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Radius")),
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
                                .with_child(Label::new("Circle"))
                                .with_spacer(5.)
                                .with_child(PortWidget::new(index, CIRCLE, PortDirection::Output, PortWidget::SHAPE)),
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
