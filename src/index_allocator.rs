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
//
/// If the entity is removed or destroyed in the course of the game, the index is deallocated. In this case,
/// it is moved into the 'free' vector. When a new entity requests an index, the allocator checks the free
/// vector and sees that index 0 is free. It therefore assigns 0 to the new entity. Since the index has been
/// used before, its generation is incremented by one. Therefore, the new entity's index is now:
/// Index: 0 Generation: 1.
///
/// ```
/// use scrapyard_ecs::index_allocator::IndexAllocator;
/// use self::scrapyard_ecs::errors::Error;
/// fn generate_multiple_indices() -> Result<(), Error> {
///       // Initialise a new Allocator
///     let mut allocator = IndexAllocator::new();
///     // Allocate an index (index: 0, generation: 0)
///     let first = allocator.allocate()?;
///     // Allocate a second index (index: 1, generation: 0)
///     let second = allocator.allocate()?;
///     // Allocate a third index (index: 2, generation: 0)
///     let third = allocator.allocate()?;
///
///     allocator.deallocate(&first);
///     // First index is now deallocated (live = false)
///     let first = allocator.allocate()?;
///     // First should now have its initial index reallocated as (index: 0, generation: 1)
///     Ok(())
/// }
/// ````
///
/// When the new entity now wants to search up anything within its index, it simply checks that both the
/// index and generation are the same. As such even though the first entity may have elements in a component
/// list, if its generation does not match, then it is not used.
#[derive(Clone)]
pub struct GenerationalIndex {
    index: usize,
    pub generation: u32,
}

impl GenerationalIndex {
    /// Returns the index of the generational index.
    pub fn index(&self) -> usize {
        self.index
    }
}

/// The raw entry which contains all information relating to the index in question.
/// Contains the index's generation, as well as aboolean specifying whether the index is in
/// use. When an object is destroyed, the live boolean will be used to specify whether an index is
/// valid for use when iterating over a component list.
struct AllocatorEntry {
    live: bool,
    generation: u32
}

/// The allocator data structure. The allocator will be used to generate indices for entities.
/// An allocator ensures that a unique instance of every index is generated so as to prevent an overlap
/// in entity possession. The allocator currently possesses two main vectors: one for entires and one
/// for free indices.
///
/// Entries: Holds a vector of AllocatorEntries which contain all index information.
/// Free: Holds all indices which have been previously released and freed for use.
///
/// The allocator also has four main functions: new(), allocate(), deallocate(), and is_live().
///
/// New: Creates a new instance of the allocator.
/// Allocate: generates a generational index. First checks the free vector for available indices.
/// If the free vector is empty, a new entry is created using the last index is used for the index
/// value.
/// Deallocate: pushes the index into the free list and sets the live flag to false.
/// Is_live: Checks whether a given index is live or not.
pub struct IndexAllocator {
    entries: Vec<AllocatorEntry>,
    free: Vec<usize>,
}

impl IndexAllocator {

    /// Creates a new instance of the index allocator. by default the vector is created with a max
    /// space of 1024 entities. This should be changed later when more requirements are made.
    /// TODO: Change the size of the entity storage as project grows.
    pub fn new() -> IndexAllocator {
        let entries : Vec<AllocatorEntry> = Vec::with_capacity(1024);
        let free : Vec<usize> = Vec::with_capacity(1024);
         IndexAllocator {
            entries,
            free
        }
    }

    /// Returns a unique generational index.
    /// Function first checks to see if any free indices are available. If there are free indices,
    /// then take the first index, pop it out of the vector, increment its generation, and then
    /// set its live status to true.
    /// If there are no free indices, then simply push a new index into the  vector and assign its
    /// index to the vector's size - 1 and set its generation to 0.
    pub fn allocate(&mut self) -> Result<GenerationalIndex, Error> {
        // Check for free indices.
        let idx = if !self.free.is_empty() {
            // Grab index and entry value.
            let free_idx = self.free[0];
            let mut entry  = &mut self.entries[free_idx];
            // Increment generation and set value to live.
            entry.generation += 1;
            entry.live = true;
            // Return new index
            GenerationalIndex {index: self.free.pop()?, generation: entry.generation}
        } else {
            // Push a new entry into index list.
            self.entries.push(AllocatorEntry { live: true, generation: 0});
            // Return new generational index.
            GenerationalIndex {index: self.entries.len() - 1, generation: 0}
        };
        Ok(idx)
    }

    /// De-allocates an index and sets it to an unusable state.
    /// Push the index into the set of free indices and set the live value to false.
    /// The freed index value will be prioritised for reuse when considering re-allocation.
    pub fn deallocate(&mut self, index : &GenerationalIndex) {
        self.free.push(index.index());
        self.entries[index.index()].live = false;
    }

    /// Checks if a value is live.
    pub fn is_live(&self, index: GenerationalIndex) -> bool {
        self.entries[index.index()].live
    }
}
