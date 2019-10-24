/// # Scrapyard Index Allocator
///
/// The index allocator is the data structure responsible for storing and synchronising all generational
/// indices as used by the ECS system. The allocator contains two vectors, one which stores all indices,
/// and one containing all free indices. Free indices are indices which have been previously used by
/// a previous item or data structure beforehand and were de-allocated at a certain point. If an index
/// exists within the free vector, then it is immediately used for any entities which require an index
/// allocation.

use self::super::errors::Error;

/// A generational index is a unique index which stores both the entity's storage index in a collection,
/// and also its specific instance number (or generation). Essentially, when a structure needs to search for
/// an element in a collection, it must ensure that both a) the indices are the same, and that b) the
/// generations are the same. In doing so, we always make sure that the index is pointing toward the correct
/// data.
///
/// ## Examples:
///
/// When first created, an index list may have no available entries. When an entity is made, it's index
/// is created. Since it is the first entity in the list, it is assigned an index of 0, and since it is
/// the first entity using that index, it is assigned a generation of 0. As such, the generational index
/// created is: Index: 0 Generation: 0.
///
/// If the entity is removed or destroyed in the course of the game, the index is deallocated. In this case,
/// it is moved into the 'free' vector. When a new entity requests an index, the allocator checks the free
/// vector and sees that index 0 is free. It therefore assigns 0 to the new entity. Since the index has been
/// used before, its generation is incremented by one. Therefore, the new entity's index is now:
/// Index: 0 Generation: 1.
///
/// When the new entity now wants to search up anything within its index, it simply checks that both the
/// index and generation are the same. As such even though the first entity may have elements in a component
/// list, if its generation does not match, then it is not used.
#[derive(Clone)]
pub struct GenerationalIndex {
    pub index: usize,
    pub generation: u32,
}

struct AllocatorEntry {
    live: bool,
    generation: u32
}

pub struct IndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl IndexAllocator {

    pub fn new() -> IndexAllocator {

        let entries : Vec<AllocatorEntry> = Vec::with_capacity(1024);
        let free : Vec<usize> = Vec::with_capacity(1024);

         IndexAllocator {
            entries,
            free
        }
    }

    pub fn allocate(&mut self) -> Result<GenerationalIndex, Error> {
        if !self.free.is_empty() {
            let free_idx = self.free[0];
            let mut entry  = &mut self.entries[free_idx];
            entry.generation += 1;
            entry.live = true;
            return Ok(GenerationalIndex {index: self.free.pop()?, generation: entry.generation});
        } else {
            self.entries.push(AllocatorEntry { live: true, generation: 0});
            Ok(GenerationalIndex {index: self.entries.len() - 1, generation: 0})
        }
    }
}
