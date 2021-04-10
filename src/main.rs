mod core;
// mod gui;
mod nodes;

use crate::core::App;
// use crate::gui::graph_widget::GraphWidget;
use crate::nodes::{particle_node_factory, value_node_factory};

fn main() {
    let mut app = App::new();

    let value_node = app.add_node(value_node_factory);
    let value_cache_index = app.get_node(value_node).get_output(0).unwrap().clone();
    *app.get_cache_mut().get_mut(&value_cache_index).unwrap() = 100.;

    let particle_node = app.add_node(particle_node_factory);

    app.add_edge(value_node, 0, particle_node, 0);

    for i in 0..100  {
        app.compute();
    }

    // let main_window = WindowDesc::new(GraphWidget::new());
    // AppLauncher::with_window(main_window)
    //     // .log_to_console()
    //     .launch(data)
}