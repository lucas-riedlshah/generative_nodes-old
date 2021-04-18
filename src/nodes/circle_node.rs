use std::{cell::RefCell, rc::Rc};

use druid::{
    widget::{Container, CrossAxisAlignment, Flex, Label},
    Color, Widget, WidgetExt,
};
use nalgebra::Vector2;

use crate::{
    core::{App, Cache, CacheIndex, Node},
    gui::{graph_widget::PortDirection, node_widget::NodeWidget, port_widget::PortWidget},
};

use super::common::shapes::Circle;

const POSITION: usize = 0;
const RADIUS: usize = 1;

const CIRCLE: usize = 0;

pub fn node_factory(cache: &mut Cache) -> Node {
    let position = cache.insert(Vector2::new(0., 0.));
    let radius = cache.insert(5.);
    let circle = cache.insert(Circle::new(Vector2::new(0., 0.), 5.));

    let mut inputs = Vec::new();
    inputs.push(position);
    inputs.push(radius);

    let mut outputs = Vec::new();
    outputs.push(circle);

    Node::new(inputs, outputs, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(create_input_cache, remove_input_cache)
}

fn compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    let position = *cache.get::<Vector2<f64>>(&inputs[POSITION]).unwrap();
    let radius = *cache.get::<f64>(&inputs[RADIUS]).unwrap();
    let circle = cache.get_mut::<Circle>(&outputs[CIRCLE]).unwrap();
    circle.set_position(position);
    circle.set_radius(radius);
}

fn remove_input_cache(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        POSITION => cache.remove::<Vector2<f64>>(&node.get_inputs()[POSITION]),
        RADIUS => cache.remove::<f64>(&node.get_inputs()[RADIUS]),
        _ => (),
    }
}

fn create_input_cache(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        POSITION => Some(cache.insert(Vector2::new(0., 0.))),
        RADIUS => Some(cache.insert(5.)),
        _ => None,
    }
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.remove::<Vector2<f64>>(&inputs[POSITION]);
    cache.remove::<f64>(&inputs[RADIUS]);
    cache.remove::<Circle>(&inputs[CIRCLE]);
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
                .fix_width(150.)
                .padding(5.),
        )
        .rounded(10.)
        .background(Color::rgba8(50, 50, 50, 230))
        .border(Color::rgb8(25, 25, 25), 1.),
    ))
}
