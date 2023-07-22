use std::ops::{Deref, DerefMut};

use parking_lot::{Mutex, MutexGuard};

pub struct MemoryMap<T> {
    parts: Vec<Mutex<T>>,
}

impl<T> MemoryMap<T> {
    pub fn new() -> Self {
        Self { parts: Vec::new() }
    }
    pub fn get(&self, index: usize) -> Option<MutexGuard<T>> {
        match self.parts.get(index) {
            Some(a) => Some(a.lock()),
            None => None,
        }
    }
    pub fn push(&mut self, item: T) {
        self.parts.push(Mutex::new(item))
    }
}
impl<T> Deref for MemoryMap<T> {
    type Target = Vec<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}
impl<T> DerefMut for MemoryMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parts
    }
}
