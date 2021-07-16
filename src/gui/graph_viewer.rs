use druid::{BoxConstraints, Command, Data, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, Menu, MenuItem, PaintCtx, Point, Size, Target, UpdateCtx, Widget, WidgetPod, widget::{Axis, Scroll}};

use super::delegate::ADD_NODE;

pub struct GraphViewer<T, W> {
    inner: WidgetPod<T, Scroll<T, W>>,
    // TODO: This has_setup feels hacky. Surely there is a better way to do this.
    has_setup: bool
}

impl<T: Data, W: Widget<T>> GraphViewer<T, W> {
    pub fn new(inner: Scroll<T, W>) -> GraphViewer<T, W> {
        GraphViewer {
            inner: WidgetPod::new(inner),
            has_setup: false
        }
    }
}

impl<T: Data, W: Widget<T>> Widget<T> for GraphViewer<T, W> {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut T,
        env: &Env,
    ) {
        match event {
            Event::MouseUp(mouse) => {
                if mouse.button.is_right() {
                    let mut menu = Menu::new("Add Node");

                    for i in ["Value", "Vector2D", "Particle", "Circle"].iter() {
                        menu = menu.entry(MenuItem::new(i.to_string()).command(Command::new(
                            ADD_NODE,
                            (i, (self.inner.widget().viewport_rect().origin().to_vec2() + mouse.pos.to_vec2()).to_point()),
                            Target::Global,
                        )));
                    }
                    
                    ctx.show_context_menu::<T>(
                        menu,
                        mouse.pos,
                    )
                }
            }
            _ => ()
        }
        self.inner.event(ctx, event, data, env);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &T,
        env: &Env,
    ) {
        self.inner.lifecycle(ctx, event, data, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        _old_data: &T,
        data: &T,
        env: &Env,
    ) {
        self.inner.update(ctx, data, env);
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &T,
        env: &Env,
    ) -> Size {
        self.inner.layout(ctx, bc, data, env);
        self.inner.set_origin(ctx, data, env, Point::ZERO);

        if !self.has_setup {
            let scroll = self.inner.widget_mut();
            scroll.scroll_by(((scroll.child_size() - scroll.viewport_rect().size()) / 2.).to_vec2());
            self.has_setup = true;
        }
        
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.inner.paint(ctx, data, env);
    }
}
