use std::{cell::RefCell, rc::Rc, time::Instant};

use druid::{
    AppDelegate, Command, DelegateCtx, Env, Handled, Selector, Target, Widget, WindowId,
};

use crate::core::App;

pub const ADD_NODE: Selector<usize> = Selector::new("add_node");
pub const ADD_EDGE: Selector<(usize, usize)> = Selector::new("begin_edge");
pub const ADD_NODE_WIDGET: Selector<(
    usize,
    fn(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>>,
)> = Selector::new("add_node_widget");

pub struct Delegate {
    creating_new_edge: bool,
    current_edge_end: Option<(usize, usize)>,
    node_widget_factories: Vec<fn(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>>>,
}

impl Delegate {
    pub fn new(
        node_widget_factories: Vec<fn(index: usize) -> Box<dyn Widget<Rc<RefCell<App>>>>>,
    ) -> Delegate {
        Delegate {
            creating_new_edge: false,
            current_edge_end: None,
            node_widget_factories,
        }
    }
}

impl AppDelegate<Rc<RefCell<App>>> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        command: &Command,
        data: &mut Rc<RefCell<App>>,
        _env: &Env,
    ) -> Handled {
        // handle adding/removing nodes and edges here
        let mut app = data.borrow_mut();

        if command.is(ADD_NODE) {
            println!("{}", command.get(ADD_NODE).unwrap());
        } else if command.is(ADD_EDGE) {
            if let Some(end) = command.get(ADD_EDGE) {
                match self.current_edge_end {
                    Some(start) => {
                        if start.0 != end.0 {
                            // TODO: Receiving node needs to say this is okay. Can sort this out later when I move the whole connect func to be dynamic.
                            app.add_edge(start.0, start.1, end.0, end.1);
                        }
                        self.creating_new_edge = false;
                        self.current_edge_end = None;
                        // ctx.request_paint(); <-- This might not need to be called.
                    }
                    None => {
                        self.creating_new_edge = true;
                        self.current_edge_end = Some(end.clone());
                    }
                }
                return Handled::Yes;
            }
        }
        Handled::No
    }

    // TODO: I guess setup goes here unless there's an actual delegate setup func somewehere. Investigate this later.
    fn window_added(
        &mut self,
        id: WindowId,
        data: &mut Rc<RefCell<App>>,
        _env: &Env,
        ctx: &mut DelegateCtx,
    ) {
        let mut app = data.borrow_mut();

        let value_node = app.add_node(0);
        ctx.submit_command(Command::new(
            ADD_NODE_WIDGET,
            (value_node, *self.node_widget_factories.get(0).unwrap()),
            Target::Window(id),
        ));

        let vector_node = app.add_node(1);
        ctx.submit_command(Command::new(
            ADD_NODE_WIDGET,
            (vector_node, *self.node_widget_factories.get(1).unwrap()),
            Target::Window(id),
        ));

        let particle_node = app.add_node(2);
        ctx.submit_command(Command::new(
            ADD_NODE_WIDGET,
            (particle_node, *self.node_widget_factories.get(2).unwrap()),
            Target::Window(id),
        ));

        let circle_node = app.add_node(3);
        ctx.submit_command(Command::new(
            ADD_NODE_WIDGET,
            (circle_node, *self.node_widget_factories.get(3).unwrap()),
            Target::Window(id),
        ));

        app.add_edge(value_node, 0, vector_node, 0);
        app.add_edge(value_node, 0, vector_node, 1);
        // app.add_edge(vector_node, 0, particle_node, 3);
    }
}
