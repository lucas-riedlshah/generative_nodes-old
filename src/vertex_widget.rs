use druid::{
    BoxConstraints, Color, Command, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx,
    PaintCtx, Point, RenderContext, Selector, Size, Target, UpdateCtx, Widget, WidgetExt,
    WidgetPod,
};

// Need to investigate ways to totally remove this widget as I don't think it's actually needed.
// Moving is_selected to the app state would make sense. The graph can just store a closure for how to modify the state then when a vertex is selected.

pub struct VertexWidget<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> VertexWidget<T> {
    pub fn new<W: Widget<T> + 'static>(inner: W) -> Self {
        VertexWidget {
            inner: WidgetPod::new(inner.boxed()),
        }
    }
}

impl<T: Data> Widget<T> for VertexWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
        match event {
            Event::MouseDown(mouse) => {
                if !self.inner.has_active() {
                    ctx.request_focus();
                    ctx.set_active(true);
                }
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
        // sort out removing these set values so that the vertex's inner will totally decide it's own size.
        let width = 200.;
        let height = 300.;
        let size = self.inner.layout(
            ctx,
            &BoxConstraints::new(Size::ZERO, Size::new(width, height)),
            data,
            env,
        );
        self.inner.set_origin(ctx, data, env, Point::ZERO);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let clip_rect = ctx.size().to_rect().to_rounded_rect(10.);
        ctx.fill(clip_rect, &Color::rgba8(50, 50, 50, 230));

        ctx.stroke(clip_rect, &Color::rgb8(25, 25, 25), 1.);

        self.inner.paint(ctx, data, env);
    }
}
