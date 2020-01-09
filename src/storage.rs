use crate::index_allocator::GenerationalIndex;

type Index = GenerationalIndex;

pub trait Storage<T> {
    fn new() -> Self
        where Self: Sized;
    fn get(&self, index: &Index) -> Option<&T>;
    fn get_mut(&mut self, index: &Index) -> Option<&mut T>;
    fn set(&mut self, value: T, index: &Index);
    fn len(&self) -> usize;
    fn contains(&self, index: &Index) -> bool;
}