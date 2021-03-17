use std::collections::HashMap;

use druid::{
    widget::{Checkbox, Flex, Label, Slider, TextBox},
    AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc,
};

mod graph_data;
mod graph_widget;
mod port_widget;
mod vertex_data;
mod vertex_widget;

use crate::graph_data::GraphData;
use crate::graph_widget::GraphWidget;
use crate::port_widget::PortWidget;
use crate::vertex_data::{BoolInputLens, FloatInputLens, Packet, StringInputLens, VertexData};
use crate::vertex_widget::VertexWidget;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder());
    let mut data = GraphData::new();
    let mut inputs_1 = HashMap::new();
    inputs_1.insert("1", Packet::Float(5.));
    let mut outputs_1 = HashMap::new();
    outputs_1.insert("2", Packet::Float(5.));
    let mut id = data.get_vertices().len();
    data.get_vertices_mut().push_back(VertexData::new(
        inputs_1,
        outputs_1,
        id,
        placeholder_generator_thing,
    ));
    let mut inputs_2 = HashMap::new();
    inputs_2.insert("1", Packet::Bool(true));
    inputs_2.insert("2", Packet::String("it work".to_string()));
    let mut outputs_2 = HashMap::new();
    id = data.get_vertices().len();
    data.get_vertices_mut().push_back(VertexData::new(
        inputs_2.clone(),
        outputs_2.clone(),
        id,
        placeholder_generator_thing_2,
    ));
    id = data.get_vertices().len();
    data.get_vertices_mut().push_back(VertexData::new(
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
    // .debug_paint_layout()
}

fn placeholder_generator_thing(data: &VertexData) -> Box<dyn Widget<VertexData>> {
    Box::new(VertexWidget::new(
        Flex::column()
            .with_child(Label::new("Node Title 1").center())
            .with_child(
                Slider::new()
                    .with_range(-10., 10.)
                    .expand_width()
                    .lens(FloatInputLens("1")),
            ),
    ))
}

fn placeholder_generator_thing_2(data: &VertexData) -> Box<dyn Widget<VertexData>> {
    Box::new(VertexWidget::new(
        Flex::column()
            .with_child(Label::new("Node Title 2").center())
            .with_spacer(5.)
            .with_child(
                TextBox::new()
                    .lens(StringInputLens("2"))
                    .expand_width()
                    .align_left(),
            )
            .with_spacer(5.)
            .with_child(Checkbox::new("1").lens(BoolInputLens("1")).align_left())
            .with_spacer(5.)
            .with_child(PortWidget::new(data.id(), "hey")),
    ))
}
