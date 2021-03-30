use std::marker::PhantomData;

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
        if self.data.contains::<T>() {
            let vec = self.data.get_mut::<Vec<T>>().unwrap();
            vec.push(value);
            return Some(CacheIndex::<T>::new(vec.len() - 1))
        }
        None
    }

    pub fn get<T: 'static>(&self, reg_index: CacheIndex<T>) -> Option<&T> {
        if self.data.contains::<T>() {
            return self.data.get::<Vec<T>>().unwrap().get(*reg_index.index())
        }
        None
    }
}
