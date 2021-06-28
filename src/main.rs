mod core;
mod gui;
mod nodes;

use std::{cell::RefCell, rc::Rc};

use druid::Point;
use druid::{widget::{Split, ClipBox, }, AppLauncher, PlatformError, WindowDesc};
use gui::viewer_2d::Viewer2D;

use crate::core::App;
use crate::gui::delegate::Delegate;
use crate::gui::graph_widget::Graph;

// TODO: Need to go through all files and check where I use vec[i] vs. vec.get(i) and make sure it all makes sense and stuff.

fn main() -> Result<(), PlatformError> {
    let app = App::new().with_factories(nodes::node_factories());

    let mut clip = ClipBox::new(Graph::new());
    clip.pan_to(Point::new(100., -50.));
    
    let main_window = WindowDesc::new(Split::columns(clip, Viewer2D::new()).draggable(true));

    AppLauncher::with_window(main_window)
        .delegate(Delegate::new(nodes::node_widget_factories()))
        .log_to_console()
        .launch(Rc::new(RefCell::new(app)))
}
