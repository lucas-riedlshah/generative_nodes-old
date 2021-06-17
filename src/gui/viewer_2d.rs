use std::{cell::RefCell, rc::Rc, time::Duration};

use druid::{kurbo, Color, Event, LifeCycle, Point, RenderContext, TimerToken, Widget};

use crate::{core::App, nodes::common::shapes::Circle};

pub struct Viewer2D {
    render_timer_token: TimerToken,
}

impl Viewer2D {
    pub fn new() -> Viewer2D {
        Viewer2D {
            render_timer_token: TimerToken::INVALID,
        }
    }
}

impl Widget<Rc<RefCell<App>>> for Viewer2D {
    fn event(
        &mut self,
        ctx: &mut druid::EventCtx,
        event: &druid::Event,
        data: &mut Rc<RefCell<App>>,
        _env: &druid::Env,
    ) {
        match event {
            Event::Timer(token) => {
                if *token == self.render_timer_token {
                    let mut app = data.borrow_mut();
                    app.compute();
                    self.render_timer_token = ctx.request_timer(Duration::from_millis(17));
                    ctx.request_paint();
                }
            }
            _ => (),
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut druid::LifeCycleCtx,
        event: &druid::LifeCycle,
        _data: &Rc<RefCell<App>>,
        _env: &druid::Env,
    ) {
        if let LifeCycle::WidgetAdded = event {
            self.render_timer_token = ctx.request_timer(Duration::from_secs(1));
        }
    }

    fn update(
        &mut self,
        _ctx: &mut druid::UpdateCtx,
        _old_data: &Rc<RefCell<App>>,
        _data: &Rc<RefCell<App>>,
        _env: &druid::Env,
    ) {
    }

    fn layout(
        &mut self,
        _ctx: &mut druid::LayoutCtx,
        bc: &druid::BoxConstraints,
        _data: &Rc<RefCell<App>>,
        _env: &druid::Env,
    ) -> druid::Size {
        bc.min()
    }

    fn paint(&mut self, ctx: &mut druid::PaintCtx, data: &Rc<RefCell<App>>, _env: &druid::Env) {
        if data
            .borrow()
            .get_cache()
            .get_all_of_type::<Circle>()
            .is_some()
        {
            for i in data
                .borrow()
                .get_cache()
                .get_all_of_type::<Circle>()
                .unwrap()
                .iter()
            {
                if let Some(circle) = i {
                    ctx.fill(
                        kurbo::Circle::new(
                            Point::new(circle.get_position().x, circle.get_position().y),
                            *circle.get_radius(),
                        ),
                        &Color::WHITE,
                    )
                }
            }
        }
    }
}
