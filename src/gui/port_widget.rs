use std::ops::Add;

use druid::{kurbo::Circle, widget::prelude::*, Color, Command, Point, Target};

use crate::gui::graph_widget::{ADD_EDGE, Port};

const RADIUS: f64 = 5.;

pub struct PortWidget {
    port: Port,
}

impl PortWidget {
    pub fn new(port: Port) -> PortWidget {
        PortWidget {
            port
        }
    }
}

impl<T> Widget<T> for PortWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseUp(_mouse) => ctx.submit_notification(Command::new(
                ADD_EDGE,
                (self.port, ctx.window_origin().add((RADIUS, RADIUS))),
                Target::Auto,
            )),
            _ => (),
        }
    }
    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &T, _env: &Env) {}
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
        ctx.fill(Circle::new(Point::new(RADIUS, RADIUS), RADIUS), &Color::rgb8(200, 50, 150));
    }
}
