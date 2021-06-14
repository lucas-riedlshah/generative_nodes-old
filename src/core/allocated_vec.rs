use std::slice::{Iter, IterMut};

pub struct AllocatedVec<T> {
    vec: Vec<Option<T>>,
    free: Vec<usize>,
}

impl<T> AllocatedVec<T> {
    pub fn new() -> AllocatedVec<T> {
        AllocatedVec {
            vec: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn push(&mut self, value: T) -> usize {
        match self.free.pop() {
            Some(index) => {
                self.set(index, Some(value));
                index
            },
            None => {
                let index = self.vec.len();
                self.vec.push(Some(value));
                index
            },
        }
    }

    pub fn set(&mut self, index: usize, value: Option<T>) {
        if index < self.vec.len() {
            self.vec[index] = value;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self.vec.get(index) {
            Some(value) => value.as_ref(),
            None => None
        }
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        match self.vec.get_mut(index) {
            Some(value) => value.as_mut(),
            None => None
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        let old_value = self.vec.remove(index);
        self.vec.insert(index, None);
        self.free.push(index);
        self.free.sort_unstable_by(|a, b| b.cmp(a));
        old_value
    }

    pub fn len(&self) -> usize {
        self.vec.len() - self.free.len()
    }
    
    pub fn raw_len(&self) -> usize {
        self.vec.len()
    }

    pub fn iter(&self) -> Iter<'_, Option<T>> {
        self.vec.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Option<T>> {
        self.vec.iter_mut()
    }
}