#[cfg(feature = "analyzer")]
use crate::analyzer::{allocate_memory, deallocate_memory, MemoryTracker};

#[cfg(feature = "analyzer")]
pub struct GameEngine {
    tracker: MemoryTracker,
}

#[cfg(feature = "analyzer")]
impl GameEngine {
    pub fn new() -> Self {
        GameEngine {
            tracker: MemoryTracker::new(),
        }
    }

    pub fn allocate_resource(&mut self, size: usize, description: &str) -> *mut u8 {
        let ptr = allocate_memory(size);

        self.tracker.record_allocation(
            self.tracker.allocations.len() as u32 + 1,
            size,
            description.to_string(),
        );

        ptr
    }

    pub fn deallocate_resource(&mut self, ptr: *mut u8, size: usize) {
        deallocate_memory(ptr, size);

        self.tracker.record_allocation(
            self.tracker.allocations.len() as u32 + 1,
            size,
            "Deallocated resource".to_string(),
        );
    }

    pub fn run(&mut self) {
        // simulate game loop
        let resource = self.allocate_resource(4098, "Texture data");
        // perform operations with the resource
        self.deallocate_resource(resource, 1024);

        // report memory usage
        self.tracker.report();
        println!("Peak memory usage: {} bytes", self.tracker.peak_usage());
    }
}

#[cfg(feature = "analyzer")]
pub fn main() {
    let mut engine = GameEngine::new();
    engine.run()
}
