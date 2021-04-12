use nalgebra::Vector2;

use crate::core::{Cache, CacheIndex, Node};

pub fn vector_node_factory(cache: &mut Cache) -> Node {
    let x = cache.insert(0.);
    let y = cache.insert(0.);
    let vector = cache.insert(Vector2::new(0., 0.));

    let mut inputs = Vec::new();
    inputs.push(x);
    inputs.push(y);

    let mut outputs = Vec::new();
    outputs.push(vector);

    Node::new(inputs, outputs, remove_all_cache)
        .with_compute(compute)
        .with_create_remove_input_cache(create_input_cache, remove_input_cache)
}

fn compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.get_mut::<Vector2<f64>>(&outputs[0]).unwrap().x =
        cache.get::<f64>(&inputs[0]).unwrap().clone();
    cache.get_mut::<Vector2<f64>>(&outputs[0]).unwrap().y =
        cache.get::<f64>(&inputs[1]).unwrap().clone();
}

fn remove_input_cache(node: &Node, port: usize, cache: &mut Cache) {
    match port {
        0 | 1 => cache.remove::<f64>(&node.get_inputs()[port]),
        _ => (),
    }
}

fn create_input_cache(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    match port {
        0 => Some(cache.insert(0.)),
        1 => Some(cache.insert(0.)),
        _ => None,
    }
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.remove::<f64>(&inputs[0]);
    cache.remove::<f64>(&inputs[1]);
    cache.remove::<Vector2<f64>>(&outputs[0]);
}
