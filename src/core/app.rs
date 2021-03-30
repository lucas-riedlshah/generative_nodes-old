use std::sync::Arc;

use druid::Data;
use crate::core::{Cache, Graph};

#[derive(Clone, Data)]
pub struct App {
    cache: Arc<Cache>,
    graph: Graph,
}

impl App {
    pub fn new() -> App {
        App {
            cache: Arc::new(Cache::new()),
            graph: Graph::new(),
        }
    }

    pub fn get_cache(&self) -> &Cache {
        &self.cache
    }

    pub fn get_cache_mut(&mut self) -> Option<&mut Cache> {
        Arc::get_mut(&mut self.cache)
    }
}
