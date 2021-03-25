use druid::{
    BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx,
    Point, Size, UpdateCtx, Widget, WidgetPod,
};
pub struct NodeWidget<T> {
    inner: WidgetPod<T, Box<dyn Widget<T>>>,
}

impl<T: Data> NodeWidget<T> {
    pub fn new<W: Widget<T> + 'static>(inner: W) -> Self {
        NodeWidget {
            inner: WidgetPod::new(Box::new(inner)),
        }
    }
}

impl<T: Data> Widget<T> for NodeWidget<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.inner.event(ctx, event, data, env);
        match event {
            Event::MouseDown(_mouse) => {
                if !self.inner.has_active() {
                    ctx.request_focus();
                    ctx.set_active(true);
                }
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

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let size = self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ZERO);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}
