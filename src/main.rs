mod core;
mod gui;
mod nodes;

use std::{cell::RefCell, rc::Rc};

use druid::{AppLauncher, PlatformError, WindowDesc};

use crate::core::App;
use crate::gui::delegate::Delegate;
use crate::gui::graph_widget::GraphWidget;

fn main() -> Result<(), PlatformError> {
    let app = App::new().with_factories(nodes::node_factories());

    let main_window = WindowDesc::new(GraphWidget::new());

    AppLauncher::with_window(main_window)
        .delegate(Delegate::new(nodes::node_widget_factories()))
        // .log_to_console()
        .launch(Rc::new(RefCell::new(app)))
}
