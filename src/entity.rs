use crate::index_allocator::GenerationalIndex;
use crate::ecs::ECS;
use crate::component::Component;
use crate::errors::Error;

pub struct EntityBuilder<'a> {
    pub id: GenerationalIndex,
    pub ecs: &'a mut ECS
}

impl<'a> EntityBuilder<'a> {
    pub fn new(id: GenerationalIndex, ecs: &'a mut ECS) -> Self {
        EntityBuilder { id, ecs }
    }

    pub fn with<C: Component>(self, component: C) -> Result<Self, Error> {
        self.ecs.add_component(component, &self.id)?;
        Ok(self)
    }

    pub fn build(self) -> GenerationalIndex {
        self.id
    }
}