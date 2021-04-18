use crate::core::{Cache, CacheIndex};

pub struct Node {
    /// Stores [CacheIndex]'s to all inputs(/internal values).
    inputs: Vec<CacheIndex>,
    /// Stores [CacheIndex]'s to all outputs.
    outputs: Vec<CacheIndex>,
    /** Modifies the [Cache].
        Should generally get immutable references to inputs. The except being "internal" values (inputs which aren't exposed by the GUI).
        Should alwawys get mutable references to outputs.
        Called every frame.
    */
    compute: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
    /// Called when the node is about to be removed. Primary purpose is to remove any input/output values from the [Cache].
    remove_all_cache: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
    // TODO: consider replacing the create/remove input cache funcs with complete disconnect/connect funcs.
    /// Called when an input port is disconnected and so a new value must be created in the Cache.
    create_input_cache: fn(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex>,
    /// Called when an input port is connected and so the current ("internal") value must be removed from the [Cache].
    remove_input_cache: fn(node: &Node, port: usize, cache: &mut Cache),
}

impl Node {
    pub fn new(
        inputs: Vec<CacheIndex>,
        outputs: Vec<CacheIndex>,
        remove_all_cache: fn(
            inputs: &Vec<CacheIndex>,
            outputs: &Vec<CacheIndex>,
            cache: &mut Cache,
        ),
    ) -> Self {
        Node {
            inputs,
            outputs,
            compute: default_compute,
            remove_all_cache,
            create_input_cache: default_create_input_cache,
            remove_input_cache: default_remove_input_cache,
        }
    }

    pub fn with_compute(
        mut self,
        compute_func: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
    ) -> Self {
        self.compute = compute_func;
        self
    }

    pub fn with_create_remove_input_cache(
        mut self,
        create_input_cache_func: fn(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex>,
        remove_input_cache_func: fn(node: &Node, port: usize, cache: &mut Cache),
    ) -> Self {
        self.create_input_cache = create_input_cache_func;
        self.remove_input_cache = remove_input_cache_func;
        self
    }

    pub fn connect_input(&mut self, port: usize, new_cache_index: CacheIndex, cache: &mut Cache) {
        // TODO: investigate replacing these functions with something that uses TypeId & the raw anymap instead of "dynamic" functions
        (self.remove_input_cache)(&self, port, cache);
        *self.inputs.get_mut(port).unwrap() = new_cache_index;
    }

    pub fn disconnect_input(&mut self, port: usize, cache: &mut Cache) {
        self.inputs.remove(port);
        self.inputs
            .insert(port, (self.create_input_cache)(&self, port, cache).unwrap());
    }

    pub fn get_output(&self, port: usize) -> Option<&CacheIndex> {
        self.outputs.get(port)
    }

    pub fn get_inputs(&self) -> &Vec<CacheIndex> {
        &self.inputs
    }

    pub fn get_outputs(&self) -> &Vec<CacheIndex> {
        &self.outputs
    }

    pub fn compute(&self, cache: &mut Cache) {
        (self.compute)(&self.inputs, &self.outputs, cache);
    }

    pub fn remove_all_cache(&mut self, cache: &mut Cache) {
        (self.remove_all_cache)(&self.inputs, &self.outputs, cache);
    }
}

// TODO: switch to Option<fn> instead of these default fn's. 
fn default_compute(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache) {}
fn default_remove_input_cache(node: &Node, port: usize, cache: &mut Cache) {}
fn default_create_input_cache(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex> {
    None
}
