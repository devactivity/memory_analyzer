use std::alloc::{alloc, dealloc, Layout};

#[derive(Debug)]
pub struct Allocation {
    pub id: u32,
    pub size: usize,
    pub description: String,
}

#[derive(Debug)]
pub struct MemoryTracker {
    pub allocations: Vec<Allocation>,
}

impl MemoryTracker {
    pub fn new() -> MemoryTracker {
        MemoryTracker {
            allocations: Vec::new(),
        }
    }

    pub fn record_allocation(&mut self, id: u32, size: usize, description: String) {
        let allocation = Allocation {
            id,
            size,
            description,
        };

        self.allocations.push(allocation);
    }

    pub fn report(&self) {
        println!("Memory Allocations:");
        for alloc in &self.allocations {
            println!(
                "ID: {}, Size: {}, Description: {}",
                alloc.id, alloc.size, alloc.description
            );
        }
    }

    pub fn peak_usage(&self) -> usize {
        self.allocations.iter().map(|a| a.size).sum()
    }
}

pub fn allocate_memory(size: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align(size, 1).unwrap();
        let ptr = alloc(layout);
        if ptr.is_null() {
            panic!("failed to allocate memory");
        }

        ptr
    }
}

pub fn deallocate_memory(ptr: *mut u8, size: usize) {
    unsafe {
        let layout = Layout::from_size_align(size, 1).unwrap();
        dealloc(ptr, layout);
    }
}

pub fn main() {
    let mut tracker = MemoryTracker::new();

    // simulate memory allocations
    let size_1 = 512;
    let size_2 = 2048;
    let ptr_1 = allocate_memory(size_1);

    tracker.record_allocation(1, size_1, "Allocated 512 bytes".to_string());

    let ptr_2 = allocate_memory(size_2);

    tracker.record_allocation(2, size_2, "Allocated 2048 bytes".to_string());

    // simulate some operations
    println!("Simulating operations...");
    // in a real scenario, you might perform oeprations here that use the allocated memory

    // deallocate memory
    deallocate_memory(ptr_1, size_1);
    tracker.record_allocation(3, size_1, "Deallocated 512 bytes".to_string());

    deallocate_memory(ptr_2, size_2);
    tracker.record_allocation(4, size_2, "Deallocated 2048 bytes".to_string());

    // report allocations
    tracker.report();

    // display peak memory usage
    println!("Peak memory usage: {} bytes", tracker.peak_usage());
}
