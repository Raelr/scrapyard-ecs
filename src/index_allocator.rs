use self::super::errors::Error;

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

        let entries : Vec<AllocatorEntry> = Vec::with_capacity(1000);
        let free : Vec<usize> = Vec::with_capacity(1000);

         IndexAllocator {
            entries,
            free
        }
    }

    pub fn allocate(&mut self) -> Result<GenerationalIndex, Error> {

        let mut index : usize = 0;
        let mut generation : u32 = 0;

        if !self.free.is_empty() {

            let free_idx = self.free[0];
            let mut entry  = &mut self.entries[free_idx];

            entry.generation += 1;
            entry.live = true;

            index = self.free.pop()?;
            generation = entry.generation;

        } else {

            self.entries.push(AllocatorEntry { live: true, generation: 0});
            index = self.entries.len() - 1;
        }

        Ok(GenerationalIndex {index, generation})
    }
}
