use std::{cell::RefCell, rc::Rc};

use druid::{
    widget::{Container, CrossAxisAlignment, Flex, Label},
    Color, Widget, WidgetExt,
};

use crate::{
    core::{App, Cache, CacheIndex, Node},
    gui::{graph_widget::PortDirection, node_widget::NodeWidget, port_widget::PortWidget},
};

pub fn node_factory(cache: &mut Cache) -> Node {
    let value = cache.insert(10.);

    let inputs = Vec::new();

    let mut outputs = Vec::new();
    outputs.push(value);

    Node::new(inputs, outputs, remove_all_cache)
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.remove::<f64>(&outputs[0]);
}

pub fn widget_factory(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>> {
    Box::new(NodeWidget::new(
        Container::new(
            Flex::column()
                .with_child(Label::new("Value"))
                .with_spacer(5.)
                .with_child(
                    // Outputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::End)
                        .with_child(
                            Flex::row()
                                .with_child(Label::new("Value"))
                                .with_spacer(5.)
                                .with_child(PortWidget::new(
                                    index,
                                    0,
                                    PortDirection::Output,
                                    PortWidget::F64,
                                )),
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
