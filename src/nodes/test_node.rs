use crate::core::{Cache, CacheIndex, Node};

pub fn test_node_factory(cache: &mut Cache) -> Node {
    let mut inputs = Vec::new();
    inputs.push(cache.insert("input 0".to_owned()));
    inputs.push(cache.insert("input 1".to_owned()));
    inputs.push(cache.insert("input 2".to_owned()));

    let mut outputs = Vec::new();
    outputs.push(cache.insert("output 0".to_owned()));
    outputs.push(cache.insert("output 1".to_owned()));
    outputs.push(cache.insert("output 2".to_owned()));

    Node::new(inputs, outputs, compute, remove_all_cache, remove_input_cache, create_input_cache)
}

fn compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    let number = cache.get::<String>(&inputs[0]).unwrap();
    println!("compute: {}", number);
}

fn remove_input_cache(port: usize, cache_index: CacheIndex, cache: &mut Cache) {
    match port {
        0 | 1 | 2 => cache.remove::<String>(&cache_index),
        _ => ()
    }
}

fn create_input_cache(port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        0 => Some(cache.insert("input 0".to_owned())),
        1 => Some(cache.insert("input 1".to_owned())),
        2 => Some(cache.insert("input 2".to_owned())),
        _ => None
    }
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    for cache_index in inputs.iter().chain(outputs) {
        cache.remove::<String>(cache_index);
    }
}

// fn placeholder_generator_thing(data: &Node) -> Box<dyn Widget<Node>> {
//     Box::new(NodeWidget::new(
//         Container::new(
//             Flex::column()
//                 .with_child(Label::new("Node Title 2"))
//                 .with_spacer(5.)
//                 .with_child(
//                     // Inputs
//                     Flex::column()
//                         .cross_axis_alignment(CrossAxisAlignment::Start)
//                         .with_child(
//                             Flex::row()
//                                 .with_child(PortWidget::new(Port::new(
//                                     data.id(),
//                                     "string",
//                                     Direction::Input,
//                                 )))
//                                 .with_spacer(5.)
//                                 .with_child(TextBox::new().lens(StringInputLens("string"))),
//                         )
//                         .with_spacer(5.)
//                         .with_child(
//                             Flex::row()
//                                 .with_child(PortWidget::new(Port::new(
//                                     data.id(),
//                                     "float",
//                                     Direction::Input,
//                                 )))
//                                 .with_spacer(5.)
//                                 .with_child(
//                                     Slider::new()
//                                         .with_range(-100., 100.)
//                                         .lens(FloatInputLens("float")),
//                                 ),
//                         )
//                         .with_spacer(5.)
//                         .with_child(
//                             Flex::row()
//                                 .with_child(PortWidget::new(Port::new(
//                                     data.id(),
//                                     "bool",
//                                     Direction::Input,
//                                 )))
//                                 .with_spacer(5.)
//                                 .with_child(Checkbox::new("bool").lens(BoolInputLens("bool"))),
//                         )
//                         .expand_width(),
//                 )
//                 .with_spacer(5.)
//                 .with_child(
//                     // Outputs
//                     Flex::column()
//                         .cross_axis_alignment(CrossAxisAlignment::End)
//                         .with_child(PortWidget::new(Port::new(
//                             data.id(),
//                             "bool_out",
//                             Direction::Output,
//                         )))
//                         .expand_width(),
//                 )
//                 .fix_width(150.)
//                 .padding(5.),
//         )
//         .rounded(10.)
//         .background(Color::rgba8(50, 50, 50, 230))
//         .border(Color::rgb8(25, 25, 25), 1.),
//     ))
// }
