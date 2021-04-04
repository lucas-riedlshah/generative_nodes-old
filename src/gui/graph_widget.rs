use std::{cmp::Ordering, ops::Add, time::Instant};

use druid::{
    kurbo::QuadBez, BoxConstraints, Color, Command, ContextMenu, Env, Event, EventCtx, LayoutCtx,
    LifeCycle, LifeCycleCtx, LocalizedString, MenuDesc, MenuItem, PaintCtx, Point, RenderContext,
    Selector, Size, Target, UpdateCtx, Widget, WidgetPod,
};

use crate::core::Graph;
use crate::core::Node;

// These will need to be moved to a delegate when GraphWidget is no longer the root of the application.
const ADD_NODE: Selector<f64> = Selector::<f64>::new("add_node");
pub const ADD_EDGE: Selector<(Port, Point)> = Selector::<(Port, Point)>::new("begin_edge");

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Input,
    Output,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Port {
    node_id: usize,
    port_name: &'static str,
    direction: Direction,
}

impl Port {
    pub fn new(node_id: usize, port_name: &'static str, direction: Direction) -> Port {
        Port {
            node_id,
            port_name,
            direction,
        }
    }
}

struct GraphWidgetNode {
    widget: WidgetPod<Node, Box<dyn Widget<Node>>>,
    position: Point,
    is_selected: bool,
}

impl GraphWidgetNode {
    fn new<W: Widget<Node> + 'static>(widget: W) -> Self {
        GraphWidgetNode {
            widget: WidgetPod::new(Box::new(widget)),
            position: Point::new(5., 5.),
            is_selected: false,
        }
    }
}

pub struct GraphWidget {
    nodes: Vec<GraphWidgetNode>,
    // maybe replace edges with their own widgets so that they can be selected and stuff.
    edges: Vec<(Point, Point)>,
    node_render_order: Vec<usize>,
    is_translating_nodes: bool,
    creating_new_edge: bool,
    current_edge_end: Option<(Port, Point)>,
    last_mouse_pos: Point,
    last_layout_instant: Instant,
}

impl GraphWidget {
    pub fn new() -> Self {
        GraphWidget {
            nodes: Vec::new(),
            edges: Vec::new(),
            node_render_order: Vec::new(),
            is_translating_nodes: false,
            creating_new_edge: false,
            current_edge_end: None,
            last_mouse_pos: Point::ZERO,
            last_layout_instant: Instant::now(),
        }
    }

    // This might need to be replaced.
    fn update_child_count(&mut self, data: &Graph, _env: &Env) -> bool {
        let len = self.nodes.len();
        match len.cmp(&data.get_nodes().len()) {
            Ordering::Greater => self.nodes.truncate(data.get_nodes().len()),
            Ordering::Less => {
                for (node_data, i) in data.get_nodes().iter().zip(0..data.get_nodes().len()) {
                    if i >= len {
                        let node = GraphWidgetNode::new(node_data.generate_widget());
                        self.node_render_order.push(self.nodes.len());
                        self.nodes.push(node);
                    }
                }
            }
            Ordering::Equal => (),
        }
        len != data.get_nodes().len()
    }

    fn deselect_all_nodes(&mut self, ctx: &mut EventCtx) {
        self.nodes.iter_mut().for_each(|node| {
            node.is_selected = false;
            ctx.submit_command(Command::new(
                Selector::<bool>::new("update_selected"),
                node.is_selected.clone(), // does this need to be cloned?
                Target::Widget(node.widget.id()),
            ));
        });
    }

    fn find_node_at_pos(&self, pos: &Point) -> Option<&usize> {
        self.node_render_order.iter().rev().find(|&&node_id| {
            self.nodes
                .get(node_id)
                .unwrap()
                .widget
                .layout_rect()
                .contains(*pos)
        })
    }
}

impl Widget<Graph> for GraphWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Graph, env: &Env) {
        for node_index in self.node_render_order.iter().rev() {
            let node = self.nodes.get_mut(*node_index).unwrap();
            let node_data = data.get_nodes_mut().get_mut(*node_index).unwrap();
            node.widget.event(ctx, event, node_data, env);
        }

        match event {
            Event::Command(command) if command.is(ADD_NODE) => {
                println!("{}", command.get(ADD_NODE).unwrap());
            }
            Event::Notification(notification) => {
                if notification.is(ADD_EDGE) {
                    if let Some(edge_end) = notification.get(ADD_EDGE) {
                        match self.current_edge_end {
                            Some(first_edge_end) => {
                                if first_edge_end.0 != edge_end.0 {
                                    // Both nodes need to say yes this is okay.
                                    // Need to check if the edge already exists.
                                    data.get_edges_mut().push((
                                        first_edge_end.0.node_id,
                                        first_edge_end.0.port_name,
                                        edge_end.0.node_id,
                                        edge_end.0.port_name,
                                    ));
                                    // may need to also subtract graph widget position of this later if graph widget ends up not being the root widget.
                                    let p0 = (first_edge_end.1
                                        - self
                                            .nodes
                                            .get(first_edge_end.0.node_id)
                                            .unwrap()
                                            .position)
                                        .to_point();
                                    let p1 = (edge_end.1
                                        - self.nodes.get(edge_end.0.node_id).unwrap().position)
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
                    for node in &self.nodes {
                        if node.widget.is_active() {
                            has_active = true;
                            break;
                        }
                    }

                    if has_active {
                        if !mouse.mods.shift() {
                            self.deselect_all_nodes(ctx)
                        };

                        let node_index = self.find_node_at_pos(&mouse.pos);

                        if let Some(value) = node_index {
                            let index = value.clone();
                            let render_order_index = self
                                .node_render_order
                                .iter()
                                .position(|node_index| node_index == &index)
                                .unwrap();
                            self.node_render_order.remove(render_order_index);
                            self.node_render_order.push(index);
                            let node = self.nodes.get_mut(index).unwrap();

                            node.is_selected = true;
                            ctx.request_paint();
                        }

                        self.is_translating_nodes = true;
                        self.last_mouse_pos = mouse.pos;
                    } else {
                        self.deselect_all_nodes(ctx);
                        ctx.request_paint();
                    }
                } else {
                    self.is_translating_nodes = false;
                }
            }
            Event::MouseUp(mouse) => {
                if self.is_translating_nodes {
                    ctx.request_layout();
                }
                self.is_translating_nodes = false;
                if mouse.button.is_right() {
                    ctx.show_context_menu(ContextMenu::new(
                        MenuDesc::<Graph>::new(LocalizedString::new("Add node")).append(
                            MenuItem::new(
                                LocalizedString::new("Node Type 1"),
                                Command::new(ADD_NODE, 69., Target::Widget(ctx.widget_id())),
                            ),
                        ),
                        mouse.pos,
                    ))
                }
            }
            Event::MouseMove(mouse) => {
                if self.is_translating_nodes {
                    let delta = mouse.pos - self.last_mouse_pos;
                    self.nodes.iter_mut().for_each(|node| {
                        if node.is_selected {
                            node.position += (delta.x, delta.y);
                        }
                    });
                    self.last_mouse_pos = mouse.pos;
                    if self.last_layout_instant.elapsed().as_millis() > 16 {
                        ctx.request_layout();
                    }
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Graph, env: &Env) {
        if let LifeCycle::WidgetAdded = event {
            if self.update_child_count(data, env) {
                ctx.children_changed();
            }
        }

        for node_index in &self.node_render_order {
            let node = self.nodes.get_mut(*node_index).unwrap();
            let node_data = data.get_nodes().get(*node_index).unwrap();
            node.widget.lifecycle(ctx, event, node_data, env);
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &Graph, data: &Graph, env: &Env) {
        for node_index in &self.node_render_order {
            let node = self.nodes.get_mut(*node_index).unwrap();
            let node_data = data.get_nodes().get(*node_index).unwrap();
            node.widget.update(ctx, node_data, env);
        }

        let mut nodes = self.nodes.iter_mut();
        for node_data in data.get_nodes() {
            if let Some(node) = nodes.next() {
                node.widget.update(ctx, node_data, env);
            }
        }

        if self.update_child_count(data, env) {
            ctx.children_changed();
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Graph,
        env: &Env,
    ) -> Size {
        let child_box_constraints = BoxConstraints::new(Size::ZERO, Size::new(1000., 1000.));
        for node_index in &self.node_render_order {
            let node = self.nodes.get_mut(*node_index).unwrap();
            let node_data = data.get_nodes().get(*node_index).unwrap();
            node.widget
                .layout(ctx, &child_box_constraints, node_data, env);
            node.widget.set_origin(ctx, node_data, env, node.position);
        }

        self.last_layout_instant = Instant::now();
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Graph, env: &Env) {
        let clip_rect = ctx.size().to_rect();
        ctx.fill(clip_rect, &Color::rgb8(15, 15, 15));

        for ((start_relative, end_relative), (start_node_index, _, end_node_index, _)) in
            self.edges.iter().zip(data.get_edges())
        {
            let start = *start_relative
                + self
                    .nodes
                    .get(*start_node_index)
                    .unwrap()
                    .position
                    .to_vec2();
            let end = *end_relative + self.nodes.get(*end_node_index).unwrap().position.to_vec2();
            let path = QuadBez::new(
                start,
                // need to figure out a cheaper way to droop the cables. Or maybe not?
                Point::lerp(start, end, 0.5).add((0., 1. * ((start - end).hypot() + 1.).log(1.1))),
                end,
            );
            ctx.stroke(path, &Color::rgb8(100, 100, 100), 2.0);
        }

        for node_index in &self.node_render_order {
            let node = self.nodes.get_mut(*node_index).unwrap();
            let node_data = data.get_nodes().get(*node_index).unwrap();
            if node.is_selected {
                let node_rect = node.widget.layout_rect();
                ctx.stroke(
                    node_rect.inflate(5., 5.).to_rounded_rect(15.),
                    &Color::rgb8(200, 50, 150),
                    1.,
                );
            }
            node.widget.paint(ctx, node_data, env);
        }
    }
}
