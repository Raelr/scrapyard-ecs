use super::component::Component;
use crate::index_allocator::GenerationalIndex;
use self::super::errors::Error;

#[derive(Clone, Copy)]
pub struct ArrayEntry<T> where T: Component {
    value : T,
    generation : u32
}

pub struct IndexArray<T> where T: Component {
    pub entries : Vec<Option<ArrayEntry<T>>>
}

enum Components<T : Component> {
    Component(T)
}

impl<T> IndexArray<T> where T: Component {

    pub fn new() -> IndexArray<T>
        where T: Component {

        IndexArray {
            entries : Vec::with_capacity(1024)
        }
    }

    pub fn set(&mut self, value: T, index: &GenerationalIndex) -> Result<(), Error> {
        let input = Some(ArrayEntry{value, generation: index.generation});
        if self.contains(index)? {
            self.entries[index.index()] = input;
        } else {
            self.entries.push(input);
        }
        Ok(())
    }

    pub fn contains(&self, index : &GenerationalIndex) -> Result<bool, Error>{
        let mut success = false;
        if !(index.index() < 0 || index.index() >= self.entries.len()) {
            if self.entries[index.index()].is_some() {
                success = true
            }
        }
        Ok(success)
    }

//    pub fn get(&self, index: &GenerationalIndex) -> Result<&T, Error> {
//
//        Ok(())
//    }
//
//    pub fn get_mut(&mut self, index: &GenerationalIndex) -> Result<&mut T, Error> {
//        Ok(())
//    }
}