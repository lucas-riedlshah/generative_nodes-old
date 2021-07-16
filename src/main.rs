mod core;
mod gui;
mod nodes;

use std::{cell::RefCell, rc::Rc};

use druid::{Point, Rect, Size};
use druid::{widget::{Split, Scroll, WidgetExt}, AppLauncher, PlatformError, WindowDesc};
use gui::graph_viewer::GraphViewer;
use gui::viewer_2d::Viewer2D;

use crate::core::App;
use crate::gui::delegate::Delegate;
use crate::gui::graph_widget::Graph;

// TODO: Need to go through all files and check where I use vec[i] vs. vec.get(i) and make sure it all makes sense and stuff.

fn main() -> Result<(), PlatformError> {
    let app = App::new().with_factories(nodes::node_factories());

    let mut scroll = Scroll::new(Graph::new());

    let main_window = WindowDesc::new(Split::columns( GraphViewer::new(scroll), Viewer2D::new()).draggable(true));

    AppLauncher::with_window(main_window)
        .delegate(Delegate::new(nodes::node_widget_factories()))
        .log_to_console()
        .launch(Rc::new(RefCell::new(app)))
}
