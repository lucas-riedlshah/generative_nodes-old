use crate::core::{Cache, CacheIndex, Node};

pub fn value_node_factory(cache: &mut Cache) -> Node {
    let value = cache.insert(1.);

    let inputs = Vec::new();

    let mut outputs = Vec::new();
    outputs.push(value);

    Node::new(inputs, outputs, remove_all_cache)
}

fn remove_all_cache(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {
    cache.remove::<f64>(&outputs[0]);
}
