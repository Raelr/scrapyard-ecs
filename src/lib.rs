#![feature(try_trait)]
mod index_allocator;
mod errors;

#[cfg(test)]
mod tests {
    use crate::index_allocator::{IndexAllocator, GenerationalIndex};
    use crate::errors::Error;

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

        println!("\nTest two:");
        println!("First: {}, {}", first.index(), first.generation);
        println!("Second: {}, {}", second.index(), second.generation);
        println!("Third: {}, {}", third.index(), third.generation);

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
        println!("\nTest three:");
        println!("Before Deallocation: {}, {}", first.index(), first.generation);
        let first = allocator.allocate()?;
        println!("After Deallocation: {}, {}", first.index(), first.generation);
        let success = first.index() == 0 && first.generation == 1;
        assert_eq!(success, true);
        Ok(())
    }
}
