#![feature(try_trait)]
mod index_allocator;
mod errors;

#[cfg(test)]
mod tests {
    use crate::index_allocator::IndexAllocator;
    use crate::errors::Error;

    #[test]
    fn generate_index() -> Result<(), Error> {

        let mut allocator = IndexAllocator::new();

        let index = allocator.allocate()?;

        let success = index.index == 0 && index.generation == 0;

        println!("{}, {}", index.index, index.generation);

        assert_eq!(success, true);

        Ok(())
    }
}
