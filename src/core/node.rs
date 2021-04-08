use crate::core::{CacheIndex, Cache};

#[derive(Clone)]
pub struct Node {
    inputs: Vec<CacheIndex>,
    outputs: Vec<CacheIndex>,
    compute: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
    remove_all_cache: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
    remove_input_cache: fn(port: usize, cache_index: CacheIndex, cache: &mut Cache),
    create_input_cache: fn(port: usize, cache: &mut Cache) -> Option<CacheIndex>
}

impl Node {
    pub fn new(
        inputs: Vec<CacheIndex>,
        outputs: Vec<CacheIndex>,
        compute: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
        remove_all_cache: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
        remove_input_cache: fn(port: usize, new_cache_index: CacheIndex, cache: &mut Cache),
        create_input_cache: fn(port: usize, cache: &mut Cache) -> Option<CacheIndex>
    ) -> Self {
        Node {
            inputs,
            outputs,
            compute,
            remove_all_cache,
            remove_input_cache,
            create_input_cache
        }
    }

    pub fn connect_input(&mut self, port: usize, new_cache_index: CacheIndex, cache: &mut Cache) {
        let old_cache_index = self.inputs.remove(port);
        (self.remove_input_cache)(port, old_cache_index, cache);
        self.inputs.insert(port, new_cache_index);
    }

    pub fn disconnect_input(&mut self, port: usize, cache: &mut Cache) {
        self.inputs.remove(port);
        self.inputs.insert(port, (self.create_input_cache)(port, cache).unwrap());
    }

    pub fn get_output(&self, port: usize) -> Option<&CacheIndex> {
        self.outputs.get(port)
    }

    pub fn compute(&self, cache: &mut Cache) {
        (self.compute)(&self.inputs, &self.outputs, cache);
    }

    pub fn remove_all_cache(&mut self, cache: &mut Cache) {

    }
}