use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};

/// Thread-safe wrapper of [Vec](std::vec::Vec) that lasts for the lifetime of the program. Used by the [extends_blackbox](blackbox_rs::macros::extends_blackbox) macros to allow for dynamic structs that extend Java classes.
pub struct MemoryMap<T> {
    parts: Lazy<Vec<Mutex<T>>>,
}

impl<T> MemoryMap<T> {
    pub const fn new() -> Self {
        Self {
            parts: Lazy::new(|| Vec::new()),
        }
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
