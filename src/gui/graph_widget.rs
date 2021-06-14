use std::{cell::RefCell, collections::HashMap, ops::Add, rc::Rc, time::Instant};

use druid::{
    kurbo::QuadBez, BoxConstraints, Color, Command, Env, Event, EventCtx, LayoutCtx,
    LifeCycle, LifeCycleCtx, LocalizedString, PaintCtx, Point, RenderContext,
    Selector, Size, Target, UpdateCtx, Widget, WidgetPod,
};

use crate::core::App;

use super::delegate::{ADD_NODE, ADD_NODE_WIDGET};

pub const REGISTER_PORT_LOCATION: Selector<(usize, usize, PortDirection, Point)> =
    Selector::new("register_port_location");

#[derive(Clone, Copy)]
pub enum PortDirection {
    Input,
    Output,
}

struct GraphWidgetNode {
    widget: WidgetPod<Rc<RefCell<App>>, Box<dyn Widget<Rc<RefCell<App>>>>>,
    position: Point,
    is_selected: bool,
}

impl GraphWidgetNode {
    fn new<W: Widget<Rc<RefCell<App>>> + 'static>(widget: W) -> Self {
        GraphWidgetNode {
            widget: WidgetPod::new(Box::new(widget)),
            position: Point::new(5., 5.),
            is_selected: false,
        }
    }
}

pub struct GraphWidget {
    nodes: Vec<GraphWidgetNode>, // TODO: Will need to replace with AllocatedVec so that the indexes match if nodes are removed.
    // maybe replace edges with their own widgets so that they can be selected and stuff.
    port_locations: HashMap<usize, (HashMap<usize, Point>, HashMap<usize, Point>)>,
    node_render_order: Vec<usize>,
    is_translating_nodes: bool,
    last_mouse_pos: Point,
    last_layout_instant: Instant,
}

impl GraphWidget {
    pub fn new() -> Self {
        GraphWidget {
            nodes: Vec::new(),
            port_locations: HashMap::new(),
            node_render_order: Vec::new(),
            is_translating_nodes: false,
            last_mouse_pos: Point::ZERO,
            last_layout_instant: Instant::now(),
        }
    }

    fn deselect_all_nodes(&mut self, ctx: &mut EventCtx) {
        self.nodes.iter_mut().for_each(|node| {
            node.is_selected = false;
            ctx.submit_command(Command::new(
                Selector::<bool>::new("update_selected"),
                node.is_selected,
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

impl Widget<Rc<RefCell<App>>> for GraphWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Rc<RefCell<App>>, env: &Env) {
        for node_index in self.node_render_order.iter().rev() {
            let node = self.nodes.get_mut(*node_index).unwrap();
            node.widget.event(ctx, event, data, env);
        }

        match event {
            Event::Command(command) => {
                if command.is(ADD_NODE_WIDGET) {
                    let (index, func) = command.get(ADD_NODE_WIDGET).unwrap();
                    let node = GraphWidgetNode::new(func(*index));
                    self.node_render_order.push(self.nodes.len());
                    self.nodes.push(node);
                    ctx.children_changed();
                }
                if command.is(REGISTER_PORT_LOCATION) {
                    let (node, port, direction, position) =
                        command.get(REGISTER_PORT_LOCATION).unwrap();

                    let (inputs, outputs) = match self.port_locations.get_mut(node) {
                        Some(node_ports) => node_ports,
                        None => {
                            self.port_locations
                                .insert(*node, (HashMap::new(), HashMap::new()));
                            self.port_locations.get_mut(node).unwrap()
                        }
                    };

                    (match direction {
                        PortDirection::Input => inputs,
                        PortDirection::Output => outputs,
                    })
                    .insert(*port, (*position - self.nodes[*node].position).to_point());
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
                        MenuDesc::<Rc<RefCell<App>>>::new(LocalizedString::new("Add Node")).append(
                            MenuItem::new(
                                LocalizedString::new("Particle"),
                                Command::new(ADD_NODE, 0, Target::Widget(ctx.widget_id())),
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

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &Rc<RefCell<App>>,
        env: &Env,
    ) {
        for node_index in &self.node_render_order {
            let node = self.nodes.get_mut(*node_index).unwrap();
            node.widget.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &Rc<RefCell<App>>,
        data: &Rc<RefCell<App>>,
        env: &Env,
    ) {
        for node_index in &self.node_render_order {
            let node = self.nodes.get_mut(*node_index).unwrap();
            node.widget.update(ctx, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &Rc<RefCell<App>>,
        env: &Env,
    ) -> Size {
        let child_box_constraints = BoxConstraints::new(Size::ZERO, Size::new(1000., 1000.));
        for node_index in &self.node_render_order {
            let node = self.nodes.get_mut(*node_index).unwrap();
            node.widget.layout(ctx, &child_box_constraints, data, env);
            node.widget.set_origin(ctx, data, env, node.position);
        }

        self.last_layout_instant = Instant::now();
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Rc<RefCell<App>>, env: &Env) {
        let clip_rect = ctx.size().to_rect();
        ctx.fill(clip_rect, &Color::rgb8(15, 15, 15));

        for edge in data.borrow().edges() {
            let start = *self
                .port_locations
                .get(&edge.from_node)
                .unwrap()
                .1 // outputs
                .get(&edge.from_port)
                .unwrap()
                + self.nodes.get(edge.from_node).unwrap().position.to_vec2();
            let end = *self
                .port_locations
                .get(&edge.to_node)
                .unwrap()
                .0 // inputs
                .get(&edge.to_port)
                .unwrap()
                + self.nodes.get(edge.to_node).unwrap().position.to_vec2();
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
            if node.is_selected {
                let node_rect = node.widget.layout_rect();
                ctx.stroke(
                    node_rect.inflate(5., 5.).to_rounded_rect(15.),
                    &Color::rgb8(200, 50, 150),
                    1.,
                );
            }
            node.widget.paint(ctx, data, env);
        }
    }
}
