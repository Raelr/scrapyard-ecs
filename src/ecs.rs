use std::collections::HashMap;
use std::cell::{RefCell, RefMut, Ref};
use std::any::{Any, TypeId};
use crate::storage::Storage;
use crate::index_allocator::{GenerationalIndex, IndexAllocator};
use crate::component::Component;
use crate::errors::Error;
use crate::entity::EntityBuilder;

type Entity = GenerationalIndex;

pub struct ECS {
    entities: Vec<Entity>,
    allocator: IndexAllocator,
    components: HashMap<TypeId, RefCell<Box<dyn Any>>>
}

impl ECS {
    pub fn new() -> Self {
        ECS { entities: Vec::new(),
            allocator: IndexAllocator::new(),
            components: HashMap::new() }
    }

    pub fn register<C: Component>(&mut self) {
        self.components.insert(TypeId::of::<C>(),
                               RefCell::new(Box::new(C::Storage::new())));
    }

    pub fn add_component<C: Component>(&mut self, component: C, index: &GenerationalIndex)
        -> Result<(), Error> {
        RefMut::map(
            self.components.get(&TypeId::of::<C>())?.borrow_mut(),
            |t| t.downcast_mut::<C::Storage>().unwrap())
            .set(component, index);
        Ok(())
    }

    pub fn get_map<C: Component>(&self) -> Result<Ref<dyn Storage<C>>, Error> {
        Ok(Ref::map(self.components.get(&TypeId::of::<C>())?.borrow(),
                    |c| c.downcast_ref::<C::Storage>().unwrap()))
    }

    pub fn get_map_mut<C: Component>(&self) -> Result<RefMut<dyn Storage<C>>, Error> {
        Ok(RefMut::map(self.components.get(&TypeId::of::<C>())?.borrow_mut(),
                       |c| c.downcast_mut::<C::Storage>().unwrap()))
    }

    pub fn create_entity(&mut self) -> Result<EntityBuilder, Error> {
        let idx = self.allocator.allocate()?;
        self.entities.push(idx.clone());
        Ok(EntityBuilder {id: idx, ecs: self })
    }
}