#![feature(try_trait)]
pub mod index_allocator;
pub mod errors;
pub mod index_array;
pub mod component;
pub mod storage;
pub mod ecs;

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
}
