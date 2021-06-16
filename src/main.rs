mod core;
mod gui;
mod nodes;

use std::{cell::RefCell, rc::Rc};

use druid::{widget::Split, AppLauncher, PlatformError, WindowDesc};
use gui::viewer_2d::Viewer2D;

use crate::core::App;
use crate::gui::delegate::Delegate;
use crate::gui::graph_widget::GraphWidget;

// TODO: Need to go through all files and check where I use vec[i] vs. vec.get(i) and make sure it all makes sense and stuff.

fn main() -> Result<(), PlatformError> {
    let app = App::new().with_factories(nodes::node_factories());

    let main_window = WindowDesc::new(Split::columns(GraphWidget::new(), Viewer2D::new()).draggable(true));

    AppLauncher::with_window(main_window)
        .delegate(Delegate::new(nodes::node_widget_factories()))
        // .log_to_console()
        .launch(Rc::new(RefCell::new(app)))
}
