use crate::core::{Cache, CacheIndex};

pub struct Node {
    // TODO: flatten inputs and outputs into one vec.
    /// Stores [CacheIndex]'s to all inputs(/internal values).
    inputs: Vec<CacheIndex>,
    /// Stores [CacheIndex]'s to all outputs.
    outputs: Vec<CacheIndex>,
    /** Modifies the [Cache].
        Should generally get immutable references to inputs. The except being "internal" values (inputs which aren't exposed by the GUI).
        Should alwawys get mutable references to outputs.
        Called every frame.
    */
    compute: Option<fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache)>,
    /// Called when the node is about to be removed. Primary purpose is to remove any input/output values from the [Cache].
    remove_all_cache: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
    // TODO: consider replacing the create/remove input cache funcs with complete disconnect/connect funcs.
    /// Called when an input port is disconnected and so a new value must be created in the Cache.
    disconnect: Option<fn(node: &Node, port: usize, cache: &mut Cache) -> Option<CacheIndex>>,
    /// Called when an input port is connected and so the current ("internal") value must be removed from the [Cache].
    connect: Option<fn(node: &Node, port: usize, cache: &mut Cache)>,
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
            compute: None,
            remove_all_cache,
            disconnect: None,
            connect: None,
        }
    }

    pub fn with_compute(
        mut self,
        compute_func: fn(inputs: &Vec<CacheIndex>, outputs: &Vec<CacheIndex>, cache: &mut Cache),
    ) -> Self {
        self.compute = Some(compute_func);
        self
    }

    pub fn with_create_remove_input_cache(
        mut self,
        create_input_cache_func: fn(
            node: &Node,
            port: usize,
            cache: &mut Cache,
        ) -> Option<CacheIndex>,
        remove_input_cache_func: fn(node: &Node, port: usize, cache: &mut Cache),
    ) -> Self {
        self.disconnect = Some(create_input_cache_func);
        self.connect = Some(remove_input_cache_func);
        self
    }

    pub fn connect_input(&mut self, port: usize, new_cache_index: CacheIndex, cache: &mut Cache) {
        if let Some(func) = self.connect {
            (func)(&self, port, cache);
        }
        *self.inputs.get_mut(port).unwrap() = new_cache_index;
    }

    pub fn disconnect_input(&mut self, port: usize, cache: &mut Cache) {
        self.inputs.remove(port);
        if let Some(func) = self.disconnect {
            self.inputs
                .insert(port, (func)(&self, port, cache).unwrap());
        }
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
        if let Some(func) = self.compute {
            (func)(&self.inputs, &self.outputs, cache);
        }
    }

    pub fn remove_all_cache(&mut self, cache: &mut Cache) {
        (self.remove_all_cache)(&self.inputs, &self.outputs, cache);
    }
}
