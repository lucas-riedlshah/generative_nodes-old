use crate::core::{Cache, CacheIndex};

pub enum Direction {
    Input,
    Output,
}

pub struct Port {
    default_cache_index: CacheIndex,
    cache_index: CacheIndex,
    direction: Direction,
    is_connected: bool, // variable for the number of connected edges? Would consequently make is_connected redundant
}

impl Port {
    pub fn new(default_cache_index: CacheIndex, direction: Direction) -> Port {
        Port {
            cache_index: default_cache_index.clone(),
            default_cache_index,
            direction,
            is_connected: false,
        }
    }

    pub fn connect(&mut self, new_cache_index: CacheIndex) {
        self.cache_index = new_cache_index;
        self.is_connected = true;
    }

    pub fn disconnect(&mut self) {
        self.cache_index = self.default_cache_index.clone();
        self.is_connected = false;
    }

    pub fn get_cache_index(&self) -> &CacheIndex {
        &self.cache_index
    }

    // TODO: Remove
    // pub fn set_cache_index(&mut self, new_cache_index: CacheIndex) {
    //     self.cache_index = new_cache_index;
    // }

    pub fn get_default_cache_index(&self) -> &CacheIndex {
        &self.default_cache_index
    }

    pub fn set_default_cache_index(&mut self, new_default_cache_index: CacheIndex) {
        self.default_cache_index = new_default_cache_index;
    }

    pub fn get_direction(&self) -> &Direction {
        &self.direction
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    // TODO: Remove?
    // pub fn set_connected(&mut self, is_connected: bool) {
    //     self.is_connected = is_connected;
    // }
}

pub struct Node {
    /// Stores [Port]s containing [CacheIndex]es to any input/output data.
    ports: Vec<Port>,
    /** Modifies the [Cache].
        Should generally get immutable references to inputs. The except being "internal" values (inputs which aren't exposed by the GUI).
        Should alwawys get mutable references to outputs.
        Called every frame.
    */
    // TODO: Investigate whether storing all these funcs in Options is more efficient than just having empty funcs which are used if no func is given.
    // Granted, this efficiency doesn't really matter for disconnect/connect functions, but compute() is gonna be called every frame, soooo...
    compute: Option<fn(ports: &Vec<Port>, cache: &mut Cache)>,
    /// Called when the node is about to be removed. Primary purpose is to remove any input/output values from the [Cache].
    remove_all_cache: fn(ports: &Vec<Port>, cache: &mut Cache),
    /// Called when an input port is disconnected and so a new value must be created in the Cache.
    disconnect: Option<fn(node: &Node, port_index: usize, cache: &mut Cache) -> Option<CacheIndex>>,
    /// Called when an input port is connected and so the current ("internal") value must be removed from the [Cache].
    connect: Option<fn(node: &Node, port_index: usize, cache: &mut Cache)>,
}

impl Node {
    pub fn new(
        ports: Vec<Port>,
        remove_all_cache: fn(
            ports: &Vec<Port>,
            cache: &mut Cache,
        ),
    ) -> Self {
        Node {
            ports,
            compute: None,
            remove_all_cache,
            disconnect: None,
            connect: None,
        }
    }

    pub fn with_compute(
        mut self,
        compute_func: fn(ports: &Vec<Port>, cache: &mut Cache),
    ) -> Self {
        self.compute = Some(compute_func);
        self
    }

    pub fn with_create_remove_input_cache(
        mut self,
        disconnect: fn(node: &Node, port_index: usize, cache: &mut Cache) -> Option<CacheIndex>,
        connect: fn(node: &Node, port_index: usize, cache: &mut Cache),
    ) -> Self {
        self.disconnect = Some(disconnect);
        self.connect = Some(connect);
        self
    }

    pub fn connect_input(&mut self, port_index: usize, new_cache_index: CacheIndex, cache: &mut Cache) {
        // TODO: This is what is causing the crash when changing an input. Needs to only remove cache if it is owned by the node and not from a connected node.
        if let Direction::Input = self.ports[port_index].get_direction() {
            if let Some(func) = self.connect {
                (func)(&self, port_index, cache);
            }
            let port = self.ports.get_mut(port_index).unwrap();
            port.connect(new_cache_index);
        }
    }

    // TODO: I'd like to essentially make this method obselete by somehow storing a "default" cache index which is
    // just switched out for any connection. And then if disconnected, this default is used once-more.
    // TODO: Would need to check somehow to see if the port is meant to be able to handle multiple inputs - if this is something I decide to implement.
    pub fn disconnect_input(&mut self, port_index: usize, cache: &mut Cache) {
        if let Direction::Input = self.ports[port_index].get_direction() {
            self.ports.get_mut(port_index).unwrap().disconnect();
        }
    }

    pub fn get_output(&self, port_index: usize) -> Option<&CacheIndex> {
        match self.ports.get(port_index) {
            Some(port) => Some(port.get_cache_index()),
            None => None
        }
    }

    pub fn get_ports(&self) -> &Vec<Port> {
        &self.ports
    }

    pub fn compute(&self, cache: &mut Cache) {
        if let Some(compute_func) = self.compute {
            (compute_func)(&self.ports, cache);
        }
    }

    pub fn remove_all_cache(&mut self, cache: &mut Cache) {
        (self.remove_all_cache)(&self.ports, cache);
    }
}
