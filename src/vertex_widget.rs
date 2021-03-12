use druid::{
    widget::Container, BoxConstraints, Color, Command, Data, Env, Event, EventCtx, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, Point, RenderContext, Selector, Size, Target, UpdateCtx,
    Widget, WidgetExt, WidgetPod,
};

pub struct VertexWidget<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
    is_selected: bool,
}

impl<T: Data> VertexWidget<T> {
    pub fn new<W: Widget<T> + 'static>(inner: W) -> Self {
        VertexWidget {
            inner: WidgetPod::new(inner.boxed()),
            is_selected: false,
        }
    }
}

impl<T: Data> Widget<T> for VertexWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
        match event {
            Event::Command(command) => {
                if let Some(value) = command.get(Selector::<bool>::new("update_selected")) {
                    self.is_selected = *value;
                    ctx.request_layout();
                }
            }
            Event::MouseDown(mouse) => {
                if !self.inner.has_active() {
                    ctx.request_focus();
                    ctx.set_active(true);
                }
                ctx.submit_notification(Command::new(
                    Selector::new("vertex_clicked"),
                    mouse.clone(),
                    Target::Auto,
                ));
                ctx.set_handled();
            }
            Event::MouseUp(_mouse) => {
                if ctx.is_active() {
                    ctx.set_active(false)
                }
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, _bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let width = 200.;
        let height = 300.;
        let padding = 5.;
        self.inner.layout(
            ctx,
            &BoxConstraints::new(
                Size::ZERO,
                Size::new(width - padding * 2., height - padding * 2.),
            ),
            data,
            env,
        );
        self.inner
            .set_origin(ctx, data, env, Point::new(padding, padding));
        Size::new(width, height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let clip_rect = ctx.size().to_rect().to_rounded_rect(10.);
        ctx.fill(clip_rect, &Color::rgba8(50, 50, 50, 230));

        if self.is_selected {
            ctx.stroke(clip_rect, &Color::rgb8(200, 50, 150), 1.);
        } else {
            ctx.stroke(clip_rect, &Color::rgb8(25, 25, 25), 1.);
        }

        self.inner.paint(ctx, data, env);
    }
}
