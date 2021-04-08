mod core;
// mod gui;
mod nodes;

use crate::core::App;
// use crate::gui::graph_widget::GraphWidget;
use crate::nodes::test_node_factory;

fn main() {
    let mut app = App::new();

    app.add_node(test_node_factory);
    app.add_node(test_node_factory);
    app.add_node(test_node_factory);
    app.add_node(test_node_factory);

    app.compute();

    app.add_edge(0, 1, 1, 0);
    app.add_edge(0, 1, 1, 1);
    app.add_edge(0, 1, 1, 2);
    app.add_edge(1, 0, 0, 0);
    app.add_edge(1, 0, 0, 1);

    app.compute();

    app.remove_edge_to(0, 0);
    app.remove_edges_from(0, 1);

    // let main_window = WindowDesc::new(GraphWidget::new());
    // AppLauncher::with_window(main_window)
    //     // .log_to_console()
    //     .launch(data)

}