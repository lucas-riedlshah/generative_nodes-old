use std::{marker::PhantomData, mem::replace};

use anymap::AnyMap;

struct CacheIndex<T> {
    value_type: PhantomData<T>,
    index: usize,
}

impl<T> CacheIndex<T> {
    pub fn new(index: usize) -> CacheIndex<T> {
        CacheIndex::<T> {
            value_type: PhantomData,
            index,
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
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

    pub fn insert<T: 'static>(&mut self, value: T) -> Option<CacheIndex<T>> {
        if !self.data.contains::<T>() {
            self.register::<T>();
        }
        let vec = self.data.get_mut::<Vec<T>>().unwrap();
        vec.push(value);
        return Some(CacheIndex::<T>::new(vec.len() - 1));
    }

    pub fn get<T: 'static>(&self, index: &CacheIndex<T>) -> Option<&T> {
        match self.data.get::<Vec<T>>() {
            Some(vec) => {
                vec.get(*index.index())
            }
            None => return None
        }
    }

    pub fn get_mut<T: 'static>(&mut self, index: &CacheIndex<T>) -> Option<&mut T> {
        match self.data.get_mut::<Vec<T>>() {
            Some(vec) => {
                vec.get_mut(*index.index())
            }
            None => return None
        }
    }

    pub fn set<T: 'static>(&mut self, index: &CacheIndex<T>, new_value: T) {
        replace(self.data.get_mut::<Vec<T>>().unwrap().get_mut(*index.index()).unwrap(), new_value);
    }

    pub fn remove<T: 'static>(&mut self, index: &CacheIndex<T>) -> T {
        self.data.get_mut::<Vec<T>>().unwrap().remove(*index.index())
    }
}
