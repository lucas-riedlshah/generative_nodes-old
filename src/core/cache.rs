use std::any::TypeId;

use anymap::AnyMap;

use crate::core::AllocatedVec;

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
        self.data.insert(AllocatedVec::<T>::new());
    }

    pub fn insert<T: 'static>(&mut self, value: T) -> CacheIndex {
        if !self.data.contains::<AllocatedVec<T>>() {
            self.register::<T>();
        }
        let vec = self.data.get_mut::<AllocatedVec<T>>().unwrap();
        vec.push(value);
        CacheIndex::new::<T>(vec.len() - 1)
    }

    pub fn get<T: 'static>(&self, cache_index: &CacheIndex) -> Option<&T> {
        if cache_index.is_type::<T>() {
            if let Some(vec) = self.data.get::<AllocatedVec<T>>() {
                return vec.get(*cache_index.index());
            }
        }
        None
    }

    pub fn get_mut<T: 'static>(&mut self, cache_index: &CacheIndex) -> Option<&mut T> {
        if cache_index.is_type::<T>() {
            if let Some(vec) = self.data.get_mut::<AllocatedVec<T>>() {
                return vec.get_mut(*cache_index.index());
            }
        }
        None
    }

    pub fn set<T: 'static>(&mut self, cache_index: &CacheIndex, value: T) {
        if cache_index.is_type::<T>() {
            self.data
                .get_mut::<AllocatedVec<T>>()
                .unwrap()
                .set(*cache_index.index(), Some(value))
        }
    }

    pub fn remove<T: 'static>(&mut self, cache_index: &CacheIndex) {
        if cache_index.is_type::<T>() {
            self.data
                .get_mut::<AllocatedVec<T>>()
                .unwrap()
                .remove(*cache_index.index());
        }
    }
}