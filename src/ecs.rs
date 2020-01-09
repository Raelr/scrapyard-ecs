use std::collections::HashMap;
use std::cell::RefCell;
use std::any::{Any, TypeId};
use crate::index_allocator::GenerationalIndex;
use crate::index_array::IndexArray;

type Entity = GenerationalIndex;
type EntityMap<T> = IndexArray<T>;

pub struct ECS {
    entities: Vec<Entity>,
    components: HashMap<TypeId, RefCell<Box<dyn Any>>>
}