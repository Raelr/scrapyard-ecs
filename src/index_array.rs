use super::component::Component;
use crate::index_allocator::GenerationalIndex;
use self::super::errors::Error;
use crate::storage::Storage;

#[derive(Clone, Copy)]
pub struct ArrayEntry<T> {
    value : T,
    generation : u32
}

pub struct IndexArray<T> {
    pub entries : Vec<Option<ArrayEntry<T>>>
}

impl<T> Storage<T> for IndexArray<T> {
    
    fn new() -> Self where Self: Sized {
        IndexArray { entries : Vec::with_capacity(1024) }
    }

    fn get(&self, index: &GenerationalIndex) -> Option<&T> {
        match self.entries[index.index()] {
            Some(ref _v) => Some(&_v.value),
            None => None
        }
    }

    fn get_mut(&mut self, index: &GenerationalIndex) -> Option<&mut T> {
        match self.entries[index.index()] {
            Some(ref mut _v) => Some(&mut _v.value),
            None => None
        }
    }

    fn set(&mut self, value: T, index: &GenerationalIndex) {
        let input = Some(ArrayEntry{value, generation: index.generation});
        if self.contains(index) {
            self.entries[index.index()] = input;
        } else {
            self.entries.push(input);
        }
    }

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn contains(&self, index: &GenerationalIndex) -> bool {
        let mut success = false;
        if !(index.index() < 0 || index.index() >= self.entries.len()) {
            if self.entries[index.index()].is_some() {
                success = true
            }
        }
        success
    }
}