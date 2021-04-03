use std::{any::{Any, TypeId}, mem::replace};

use anymap::AnyMap;

pub struct CacheIndex {
    type_id: TypeId,
    index: usize,
}

impl CacheIndex {
    pub fn new<T: 'static>(index: usize) -> CacheIndex {
        CacheIndex {
            type_id: TypeId::of::<T>(),
            index,
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn is_type<T: 'static>(&self) -> bool {
        self.type_id == TypeId::of::<T>()
    }
}

pub struct Cache {
    data: AnyMap,
}

impl Cache {
    pub fn new() -> Cache {
        Cache {
            data: AnyMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self) {
        self.data.insert(Vec::<T>::new());
    }

    pub fn insert<T: 'static>(&mut self, value: T) -> CacheIndex {
        if !self.data.contains::<T>() {
            self.register::<T>();
        }
        let vec = self.data.get_mut::<Vec<T>>().unwrap();
        vec.push(value);
        CacheIndex::new::<T>(vec.len() - 1)
    }

    pub fn get<T: 'static>(&self, index: &CacheIndex) -> Option<&T> {
        match self.data.get::<Vec<T>>() {
            Some(vec) => vec.get(*index.index()),
            None => return None,
        }
    }

    pub fn get_mut<T: 'static>(&mut self, index: &CacheIndex) -> Option<&mut T> {
        match self.data.get_mut::<Vec<T>>() {
            Some(vec) => vec.get_mut(*index.index()),
            None => return None,
        }
    }

    pub fn set<T: 'static>(&mut self, index: &CacheIndex, new_value: T) {
        replace(
            self.data
                .get_mut::<Vec<T>>()
                .unwrap()
                .get_mut(*index.index())
                .unwrap(),
            new_value,
        );
    }

    pub fn remove<T: 'static>(&mut self, index: &CacheIndex) -> T {
        self.data
            .get_mut::<Vec<T>>()
            .unwrap()
            .remove(*index.index())
    }
}
