use std::{ops::Add, time::Duration};

use druid::{kurbo::Circle, widget::prelude::*, Color, Command, Point, Target, TimerToken};

use crate::gui::delegate::ADD_EDGE;

use super::graph_widget::{PortDirection, REGISTER_PORT_LOCATION};

const RADIUS: f64 = 5.;

pub struct PortWidget {
    node: usize,
    port: usize,
    direction: PortDirection,
    color: Color,
    timer_id: TimerToken,
}

impl PortWidget {
    // Port Colors
    pub const F64: Color = Color::rgb8(102, 140, 74);
    pub const VECTOR2F64: Color = Color::rgb8(191, 191, 75);
    pub const SHAPE: Color = Color::rgb8(114, 94, 242);
    pub const PARTICLE: Color = Color::rgb8(191, 59, 59);

    pub fn new(node: usize, port: usize, direction: PortDirection, color: Color) -> PortWidget {
        PortWidget {
            node,
            port,
            direction,
            color,
            timer_id: TimerToken::INVALID,
        }
    }
}

impl<T> Widget<T> for PortWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::Timer(id) => {
                if *id == self.timer_id {
                    ctx.submit_command(Command::new(
                        REGISTER_PORT_LOCATION,
                        (
                            self.node,
                            self.port,
                            self.direction,
                            ctx.window_origin().add((RADIUS, RADIUS)),
                        ),
                        Target::Auto,
                    ));
                    self.timer_id = TimerToken::INVALID;
                }
            }
            Event::MouseDown(_mouse) => {
                ctx.request_focus();
                ctx.set_active(true);
                ctx.submit_command(Command::new(ADD_EDGE, (self.node, self.port), Target::Auto));
            }
            Event::MouseUp(_mouse) => {
                ctx.set_active(false);
            }
            _ => (),
        }
    }
    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &T, _env: &Env) {
        // TODO: Check back in the future when hopefully a timer is not necessary.
        if let LifeCycle::WidgetAdded = event {
            ctx.submit_command(Command::new(
                REGISTER_PORT_LOCATION,
                (self.node, self.port, self.direction, Point::ZERO),
                Target::Auto,
            ));
            self.timer_id = ctx.request_timer(Duration::from_millis(1));
        }
    }
    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &T, _data: &T, _env: &Env) {}
    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        _bc: &BoxConstraints,
        _data: &T,
        _env: &Env,
    ) -> druid::Size {
        Size::new(RADIUS * 2., RADIUS * 2.)
    }
    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, _env: &Env) {
        ctx.fill(
            Circle::new(Point::new(RADIUS, RADIUS), RADIUS),
            &self.color,
        );
    }
}
