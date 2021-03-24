use std::ops::Add;
use std::{cmp::Ordering, usize};

use druid::{
    kurbo::QuadBez, widget::ListIter, BoxConstraints, Color, Command, ContextMenu, Env, Event,
    EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, LocalizedString, MenuDesc, MenuItem, MouseEvent,
    PaintCtx, Point, RenderContext, Selector, Size, Target, UpdateCtx, Widget, WidgetPod,
};

use crate::graph_data::GraphData;
use crate::vertex_data::VertexData;

// These will need to be moved to a delegate when GraphWidget is no longer the root of the application.
const ADD_VERTEX: Selector<f64> = Selector::<f64>::new("add_vertex");
pub const ADD_EDGE: Selector<(Port, Point)> = Selector::<(Port, Point)>::new("begin_edge");

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Input,
    Output,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Port {
    vertex_id: usize,
    port_name: &'static str,
    direction: Direction,
}

impl Port {
    pub fn new(vertex_id: usize, port_name: &'static str, direction: Direction) -> Port {
        Port {
            vertex_id,
            port_name,
            direction,
        }
    }
}

struct Vertex {
    widget: WidgetPod<VertexData, Box<dyn Widget<VertexData>>>,
    position: Point,
    is_selected: bool,
}

impl Vertex {
    fn new<W: Widget<VertexData> + 'static>(widget: W) -> Self {
        Vertex {
            widget: WidgetPod::new(Box::new(widget)),
            position: Point::new(5., 5.),
            is_selected: false,
        }
    }
}

pub struct GraphWidget {
    vertices: Vec<Vertex>,
    // maybe replace edges with their own widgets so that they can be selected and stuff.
    edges: Vec<(Point, Point)>,
    // use this var to decide what order to process vertices.
    // default state should be [0, 1, 2, 3, ..., (len(vertices) - 1)]
    // when a vertex is focused, move it's index to the back of the vector
    vertex_render_order: Vec<usize>,
    translating_vertices: bool,
    creating_new_edge: bool,
    current_edge_end: Option<(Port, Point)>,
    last_mouse_pos: Point,
}

impl GraphWidget {
    pub fn new() -> Self {
        GraphWidget {
            vertices: Vec::new(),
            edges: Vec::new(),
            vertex_render_order: Vec::new(),
            translating_vertices: false,
            creating_new_edge: false,
            current_edge_end: None,
            last_mouse_pos: Point::ZERO,
        }
    }

    // This might need to be replaced.
    fn update_child_count(&mut self, data: &GraphData, _env: &Env) -> bool {
        let len = self.vertices.len();
        match len.cmp(&data.get_vertices().data_len()) {
            Ordering::Greater => self.vertices.truncate(data.get_vertices().data_len()),
            Ordering::Less => data.get_vertices().for_each(|vertex_data, i| {
                if i >= len {
                    let vertex = Vertex::new(vertex_data.generate_widget());
                    self.vertex_render_order.push(self.vertices.len());
                    self.vertices.push(vertex);
                }
            }),
            Ordering::Equal => (),
        }
        len != data.get_vertices().data_len()
    }

    fn deselect_all_vertices(&mut self, ctx: &mut EventCtx) {
        self.vertices.iter_mut().for_each(|vertex| {
            vertex.is_selected = false;
            ctx.submit_command(Command::new(
                Selector::<bool>::new("update_selected"),
                vertex.is_selected.clone(), // does this need to be cloned?
                Target::Widget(vertex.widget.id()),
            ));
        });
    }

    fn find_vertex_at_pos(&self, pos: &Point) -> Option<&usize> {
        self.vertex_render_order.iter().rev().find(|&&vertex_id| {
            self.vertices
                .get(vertex_id)
                .unwrap()
                .widget
                .layout_rect()
                .contains(*pos)
        })
    }
}

impl Widget<GraphData> for GraphWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut GraphData, env: &Env) {
        for vertex_index in self.vertex_render_order.iter().rev() {
            let vertex = self.vertices.get_mut(*vertex_index).unwrap();
            let vertex_data = data.get_vertices_mut().get_mut(*vertex_index).unwrap();
            vertex.widget.event(ctx, event, vertex_data, env);
        }

        // let mut vertices = self.vertices.iter_mut();
        // data.get_vertices_mut().for_each_mut(|vertex_data, _| {
        //     if let Some(vertex) = vertices.next() {
        //         vertex.widget.event(ctx, event, vertex_data, env);
        //     }
        // });

        match event {
            Event::Command(command) if command.is(ADD_VERTEX) => {
                println!("{}", command.get(ADD_VERTEX).unwrap());
            }
            Event::Notification(notification) => {
                if notification.is(ADD_EDGE) {
                    if let Some(edge_end) = notification.get(ADD_EDGE) {
                        match self.current_edge_end {
                            Some(first_edge_end) => {
                                if first_edge_end.0 != edge_end.0 {
                                    // The graphdata should probable handle this? Or (more likely?) a delegate
                                    // Both need to say yes this is okay.
                                    // Need to check if the edge already exists.
                                    data.get_edges_mut().push_back((
                                        first_edge_end.0.vertex_id,
                                        first_edge_end.0.port_name,
                                        edge_end.0.vertex_id,
                                        edge_end.0.port_name,
                                    ));
                                    // may need to also subtract graph widget position of this later if graph widget ends up not being the root widget.
                                    let p0 = (first_edge_end.1
                                        - self
                                            .vertices
                                            .get(first_edge_end.0.vertex_id)
                                            .unwrap()
                                            .position)
                                        .to_point();
                                    let p1 = (edge_end.1
                                        - self
                                            .vertices
                                            .get(edge_end.0.vertex_id)
                                            .unwrap()
                                            .position)
                                        .to_point();
                                    self.edges.push((p0, p1))
                                }
                                self.creating_new_edge = false;
                                self.current_edge_end = None;
                                ctx.request_paint();
                            }
                            None => {
                                self.creating_new_edge = true;
                                self.current_edge_end = Some(edge_end.clone());
                            }
                        }
                    }
                    ctx.set_handled();
                }
            }
            Event::MouseDown(mouse) => {
                if mouse.button.is_left() {
                    let mut has_active = false;
                    for vertex in &self.vertices {
                        if vertex.widget.is_active() {
                            has_active = true;
                            break;
                        }
                    }

                    if has_active {
                        if !mouse.mods.shift() {
                            self.deselect_all_vertices(ctx)
                        };

                        let vertex_index = self.find_vertex_at_pos(&mouse.pos);

                        if let Some(value) = vertex_index {
                            let index = value.clone();
                            let render_order_index = self
                                .vertex_render_order
                                .iter()
                                .position(|vertex_index| vertex_index == &index)
                                .unwrap();
                            self.vertex_render_order.remove(render_order_index);
                            self.vertex_render_order.push(index);
                            let vertex = self.vertices.get_mut(index).unwrap();

                            vertex.is_selected = true;
                            ctx.request_paint();
                        }

                        self.translating_vertices = true;
                        self.last_mouse_pos = mouse.pos;
                    } else {
                        self.deselect_all_vertices(ctx);
                        ctx.request_paint();
                    }
                } else {
                    self.translating_vertices = false;
                }
            }
            Event::MouseUp(mouse) => {
                self.translating_vertices = false;
                if mouse.button.is_right() {
                    ctx.show_context_menu(ContextMenu::new(
                        MenuDesc::<GraphData>::new(LocalizedString::new("Add vertex")).append(
                            MenuItem::new(
                                LocalizedString::new("Vertex Type 1"),
                                Command::new(ADD_VERTEX, 69., Target::Widget(ctx.widget_id())),
                            ),
                        ),
                        mouse.pos,
                    ))
                }
            }
            Event::MouseMove(mouse) => {
                if self.translating_vertices {
                    let delta = (mouse.pos - self.last_mouse_pos).to_point();
                    self.vertices.iter_mut().for_each(|vertex| {
                        if vertex.is_selected {
                            vertex.position += (delta.x, delta.y);
                        }
                    });
                    self.last_mouse_pos = mouse.pos;
                    ctx.request_layout();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &GraphData,
        env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            if self.update_child_count(data, env) {
                ctx.children_changed();
            }
        }

        for vertex_index in &self.vertex_render_order {
            let vertex = self.vertices.get_mut(*vertex_index).unwrap();
            let vertex_data = data.get_vertices().get(*vertex_index).unwrap();
            vertex.widget.lifecycle(ctx, event, vertex_data, env);
        }

        // let mut vertices = self.vertices.iter_mut();
        // data.get_vertices().for_each(|vertex_data, _| {
        //     if let Some(vertex) = vertices.next() {
        //         vertex.widget.lifecycle(ctx, event, vertex_data, env);
        //     }
        // });
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &GraphData, data: &GraphData, env: &Env) {
        for vertex_index in &self.vertex_render_order {
            let vertex = self.vertices.get_mut(*vertex_index).unwrap();
            let vertex_data = data.get_vertices().get(*vertex_index).unwrap();
            vertex.widget.update(ctx, vertex_data, env);
        }

        let mut vertices = self.vertices.iter_mut();
        data.get_vertices().for_each(|vertex_data, _| {
            if let Some(vertex) = vertices.next() {
                vertex.widget.update(ctx, vertex_data, env);
            }
        });

        if self.update_child_count(data, env) {
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &GraphData,
        env: &Env,
    ) -> Size {
        for vertex_index in &self.vertex_render_order {
            let vertex = self.vertices.get_mut(*vertex_index).unwrap();
            let vertex_data = data.get_vertices().get(*vertex_index).unwrap();
            vertex.widget.layout(ctx, bc, vertex_data, env);
            vertex
                .widget
                .set_origin(ctx, vertex_data, env, vertex.position);
        }

        // let mut vertices = self.vertices.iter_mut();
        // data.get_vertices().for_each(|vertex_data, _| {
        //     if let Some(vertex) = vertices.next() {
        //         vertex.widget.layout(ctx, bc, vertex_data, env);
        //         vertex
        //             .widget
        //             .set_origin(ctx, vertex_data, env, vertex.position);
        //     };
        // });

        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &GraphData, env: &Env) {
        let clip_rect = ctx.size().to_rect();
        ctx.fill(clip_rect, &Color::rgb8(15, 15, 15));

        for ((start_relative, end_relative), (start_vertex_index, _, end_vertex_index, _)) in
            self.edges.iter().zip(data.get_edges())
        {
            let start = *start_relative
                + self
                    .vertices
                    .get(*start_vertex_index)
                    .unwrap()
                    .position
                    .to_vec2();
            let end = *end_relative
                + self
                    .vertices
                    .get(*end_vertex_index)
                    .unwrap()
                    .position
                    .to_vec2();
            let path = QuadBez::new(
                start,
                // need to figure out a cheaper way to droop the cables. Or maybe not?
                Point::lerp(start, end, 0.5).add((0., 1. * ((start - end).hypot() + 1.).log(1.1))),
                end,
            );
            ctx.stroke(path, &Color::rgb8(100, 100, 100), 2.0);
        }

        for vertex_index in &self.vertex_render_order {
            let vertex = self.vertices.get_mut(*vertex_index).unwrap();
            let vertex_data = data.get_vertices().get(*vertex_index).unwrap();
            if vertex.is_selected {
                let vertex_rect = vertex.widget.layout_rect();
                ctx.stroke(
                    vertex_rect.inflate(5., 5.).to_rounded_rect(10.),
                    &Color::rgb8(200, 50, 150),
                    3.,
                );
            }
            vertex.widget.paint(ctx, vertex_data, env);
        }

        // let mut vertices = self.vertices.iter_mut();
        // data.get_vertices().for_each(|vertex_data, _| {
        //     if let Some(vertex) = vertices.next() {
        //         vertex.widget.paint(ctx, vertex_data, env);
        //     }
        // });
    }
}
