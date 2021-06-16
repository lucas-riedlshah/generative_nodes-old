use std::{cell::RefCell, rc::Rc};

use druid::{
    widget::{Container, CrossAxisAlignment, Flex, FlexParams, Label, Slider, Stepper},
    Color, Widget, WidgetExt,
};

use crate::{
    core::{App, Cache, Direction, Node, Port},
    gui::{
        cache_lens::CacheLens, graph_widget::PortDirection, node_widget::NodeWidget,
        port_widget::PortWidget,
    },
};

// Outputs
const VALUE: usize = 0;

pub fn node_factory(cache: &mut Cache) -> Node {
    let value = cache.insert(0.);

    let mut ports = Vec::new();
    ports.push(Port::new(value, Direction::Output));

    Node::new(ports, remove_all_cache)
}

fn remove_all_cache(ports: &Vec<Port>, cache: &mut Cache) {
    cache.remove::<f64>(ports[VALUE].get_cache_index());
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
                                .with_spacer(5.)
                                .with_child(PortWidget::new(
                                    index,
                                    VALUE,
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
