use std::any::TypeId;

use anymap::AnyMap;

pub struct CacheIndex {
    type_id: TypeId,
    index: usize,
}

impl CacheIndex {
    pub fn new<T: Default + 'static>(index: usize) -> CacheIndex {
        CacheIndex {
            type_id: TypeId::of::<T>(),
            index,
        }
    }

    pub fn index(&self) -> &usize {
        &self.index
    }

    pub fn is_type<T: Default + 'static>(&self) -> bool {
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

    pub fn register<T: Default + 'static>(&mut self) {
        self.data.insert(AllocatedVec::<T>::new());
    }

    pub fn insert<T: Default + 'static>(&mut self, value: T) -> CacheIndex {
        if !self.data.contains::<T>() {
            self.register::<T>();
        }
        let vec = self.data.get_mut::<AllocatedVec<T>>().unwrap();
        vec.push(value);
        CacheIndex::new::<T>(vec.len() - 1)
    }

    pub fn get<T: Default + 'static>(&self, cache_index: &CacheIndex) -> Option<&T> {
        match self.data.get::<AllocatedVec<T>>() {
            Some(vec) => vec.get(*cache_index.index()),
            None => return None,
        }
    }

    pub fn get_mut<T: Default + 'static>(&mut self, cache_index: &CacheIndex) -> Option<&mut T> {
        match self.data.get_mut::<AllocatedVec<T>>() {
            Some(vec) => vec.get_mut(*cache_index.index()),
            None => return None,
        }
    }

    pub fn set<T: Default + 'static>(&mut self, cache_index: &CacheIndex, value: T) {
        self.data.get_mut::<AllocatedVec<T>>().unwrap().set(*cache_index.index(), value)
    }

    pub fn remove<T: Default + 'static>(&mut self, cache_index: &CacheIndex) {
        self.data
            .get_mut::<AllocatedVec<T>>()
            .unwrap()
            .remove(*cache_index.index());
    }
}

pub struct AllocatedVec<T> {
    vec: Vec<T>,
    free: Vec<usize>,
}

impl<T: Default> AllocatedVec<T> {
    pub fn new() -> AllocatedVec<T> {
        AllocatedVec {
            vec: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) {
        match self.free.pop() {
            Some(index) => self.set(index, value),
            None => self.vec.push(value),
        }
    }

    pub fn set(&mut self, index: usize, value: T) {
        if index < self.vec.len() {
            self.vec[index] = value;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.vec.get_mut(index)
    }

    pub fn remove(&mut self, index: usize) {
        self.set(index, T::default());
        self.free.push(index);
        self.free.sort_unstable_by(|a, b| b.cmp(a));
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }
}
