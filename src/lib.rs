#![feature(try_trait)]
pub mod index_allocator;
pub mod errors;
pub mod index_array;
pub mod component;
pub mod storage;
pub mod ecs;
pub mod entity;

#[cfg(test)]
mod tests {
    use crate::index_allocator::{IndexAllocator};
    use crate::errors::Error;
    use crate::component::Component;
    use crate::index_array::IndexArray;
    use crate::ecs::ECS;

    struct Pos {
        x: f32
    }

    impl Component for Pos {
        type Storage = IndexArray<Self>;
    }

    #[test]
    fn generate_index() -> Result<(), Error> {
        println!("\nTest one:");
        let mut allocator = IndexAllocator::new();
        let index = allocator.allocate()?;
        let success = index.index() == 0 && index.generation == 0;
        println!("Index: {}, {}", index.index(), index.generation);
        assert_eq!(success, true);
        Ok(())
    }

    #[test]
    fn generate_multiple_indices() -> Result<(), Error> {
        let mut allocator = IndexAllocator::new();
        let first = allocator.allocate()?;
        let second = allocator.allocate()?;
        let third = allocator.allocate()?;

        let success = first.index() == 0 && first.generation == 0
            && second.index() == 1 && second.generation == 0
            && third.index() == 2 && third.generation == 0;
        assert_eq!(success, true);
        Ok(())
    }

    #[test]
    fn deallocate_indices() -> Result<(), Error> {
        let mut allocator = IndexAllocator::new();
        let first = allocator.allocate()?;
        allocator.deallocate(&first);
        let first = allocator.allocate()?;
        let success = first.index() == 0 && first.generation == 1;
        assert_eq!(success, true);
        Ok(())
    }

    #[test]
    fn register_component() -> Result<(), Error> {
        let mut ecs = ECS::new();
        ecs.register::<Pos>();
        let pos_map = &*ecs.get_map::<Pos>()?;
        assert_eq!(pos_map.len(), 0);
        Ok(())
    }

    #[test]
    fn add_components() -> Result<(), Error> {
        let mut ecs = ECS::new();
        ecs.register::<Pos>();
        let entity = ecs.create_entity()?
            .with(Pos { x: 0.0 })?
            .build();
        let pos_map = &*ecs.get_map::<Pos>()?;
        let comp = pos_map.get(&entity)?;
        assert_eq!(comp.x, 0.0);
        Ok(())
    }

    struct Vel {
        x: f32
    }

    impl Component for Vel {
        type Storage = IndexArray<Self>;
    }

    #[test]
    fn get_multiple_refs() -> Result<(), Error> {
        let mut ecs = ECS::new();
        ecs.register::<Pos>();
        ecs.register::<Vel>();
        let entity = ecs
            .create_entity()?
            .with(Pos { x: 0.0 })?
            .with(Vel { x: 5.0 })?
            .build();
        {
            let pos_map = &mut *ecs.get_map_mut::<Pos>()?;
            let vel_map = &*ecs.get_map::<Vel>()?;

            pos_map.get_mut(&entity)?.x += vel_map.get(&entity)?.x;
        }
        assert_eq!(ecs.get_map::<Pos>()?.get(&entity)?.x, 5.0);
        assert_eq!(ecs.get_map::<Vel>()?.get(&entity)?.x, 5.0);
        Ok(())
    }
}
