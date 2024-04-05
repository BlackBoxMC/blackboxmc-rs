use std::ops::{Deref, DerefMut};

use once_cell::sync::Lazy;
use parking_lot::{Mutex, MutexGuard};

/// Thread-safe wrapper of [Vec](std::vec::Vec) that lasts for the lifetime of the program. Used by the [extends_blackbox](blackbox_rs::macros::extends_blackbox) macros to allow for dynamic structs that extend Java classes.
pub struct MemoryMap<T> {
    parts: Lazy<Vec<Option<Mutex<T>>>>,
}

impl<T> MemoryMap<T> {
    pub const fn new() -> Self {
        Self {
            parts: Lazy::new(|| Vec::new()),
        }
    }
    pub fn get(&self, index: usize) -> Option<MutexGuard<T>> {
        match self.parts.get(index) {
            Some(a) => match a {
                Some(a) => Some(a.lock()),
                None => None,
            },
            None => None,
        }
    }
    pub fn push(&mut self, item: T) {
        self.parts.push(Some(Mutex::new(item)))
    }

    // we have to override remove because we cannot allow the vec to be modified,
    // because java will always count on the indexes it has bound to structs to be bound to said structs.
    pub fn remove(&mut self, index: usize) {
        self.parts[index] = None;
    }
}
impl<T> Deref for MemoryMap<T> {
    type Target = Lazy<Vec<Option<Mutex<T>>>>;

    fn deref(&self) -> &Self::Target {
        &self.parts
    }
}
impl<T> DerefMut for MemoryMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.parts
    }
}
