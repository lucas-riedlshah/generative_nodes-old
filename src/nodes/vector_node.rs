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

pub fn node_factory(cache: &mut Cache) -> Node {
    let x = cache.insert(0.);
    let y = cache.insert(0.);
    let vector = cache.insert(Vector2::new(0., 0.));

    let mut inputs = Vec::new();
    inputs.push(x);
    inputs.push(y);

    let mut outputs = Vec::new();
    outputs.push(vector);

    Node::new(inputs, outputs, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(create_input_cache, remove_input_cache)
}

fn compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.get_mut::<Vector2<f64>>(&outputs[0]).unwrap().x =
        cache.get::<f64>(&inputs[0]).unwrap().clone();
    cache.get_mut::<Vector2<f64>>(&outputs[0]).unwrap().y =
        cache.get::<f64>(&inputs[1]).unwrap().clone();
}

fn remove_input_cache(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        0 | 1 => cache.remove::<f64>(&node.get_inputs()[port]),
        _ => (),
    }
}

fn create_input_cache(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        0 => Some(cache.insert(0.)),
        1 => Some(cache.insert(0.)),
        _ => None,
    }
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.remove::<f64>(&inputs[0]);
    cache.remove::<f64>(&inputs[1]);
    cache.remove::<Vector2<f64>>(&outputs[0]);
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
                                .with_child(PortWidget::new(index, 0, PortDirection::Input))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("X")),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(index, 1, PortDirection::Input))
                                .with_spacer(5.)
                                // .with_child(TextBox::new().lens(StringInputLens("string"))),
                                .with_child(Label::new("Y")),
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
                                .with_child(PortWidget::new(index, 0, PortDirection::Output)),
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
