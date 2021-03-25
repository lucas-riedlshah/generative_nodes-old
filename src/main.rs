use std::collections::HashMap;

use druid::{
    widget::{Checkbox, Container, CrossAxisAlignment, Flex, Label, Slider, TextBox},
    AppLauncher, Color, PlatformError, Widget, WidgetExt, WindowDesc,
};

mod graph_data;
mod graph_widget;
mod node_data;
mod node_widget;
mod port_widget;

use crate::graph_data::GraphData;
use crate::graph_widget::{Direction, GraphWidget, Port};
use crate::node_data::{BoolInputLens, FloatInputLens, NodeData, Packet, StringInputLens};
use crate::node_widget::NodeWidget;
use crate::port_widget::PortWidget;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let mut data = GraphData::new();

    let mut id = data.get_nodes().len();

    let mut inputs_2 = HashMap::new();
    inputs_2.insert("bool", Packet::Bool(true));
    inputs_2.insert("float", Packet::Float(50.));
    inputs_2.insert("string", Packet::String("it work".to_string()));

    let mut outputs_2 = HashMap::new();
    outputs_2.insert("bool_out", Packet::Bool(false));

    data.get_nodes_mut().push_back(NodeData::new(
        inputs_2.clone(),
        outputs_2.clone(),
        id,
        placeholder_generator_thing_2,
    ));

    id = data.get_nodes().len();

    data.get_nodes_mut().push_back(NodeData::new(
        inputs_2,
        outputs_2,
        id,
        placeholder_generator_thing_2,
    ));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn ui_builder() -> impl Widget<GraphData> {
    GraphWidget::new()
    //.debug_paint_layout()
}

fn placeholder_generator_thing_2(data: &NodeData) -> Box<dyn Widget<NodeData>> {
    Box::new(NodeWidget::new(
        Container::new(
            Flex::column()
                .with_child(Label::new("Node Title 2"))
                .with_spacer(5.)
                .with_child(
                    // Inputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::Start)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(Port::new(
                                    data.id(),
                                    "string",
                                    Direction::Input,
                                )))
                                .with_spacer(5.)
                                .with_child(TextBox::new().lens(StringInputLens("string"))),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(Port::new(
                                    data.id(),
                                    "float",
                                    Direction::Input,
                                )))
                                .with_spacer(5.)
                                .with_child(Slider::new().with_range(-100., 100.).lens(FloatInputLens("float"))),
                        )
                        .with_spacer(5.)
                        .with_child(
                            Flex::row()
                                .with_child(PortWidget::new(Port::new(
                                    data.id(),
                                    "bool",
                                    Direction::Input,
                                )))
                                .with_spacer(5.)
                                .with_child(Checkbox::new("bool").lens(BoolInputLens("bool"))),
                        )
                        .expand_width(),
                )
                .with_spacer(5.)
                .with_child(
                    // Outputs
                    Flex::column()
                        .cross_axis_alignment(CrossAxisAlignment::End)
                        .with_child(PortWidget::new(Port::new(
                            data.id(),
                            "bool_out",
                            Direction::Output,
                        )))
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
