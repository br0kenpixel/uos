mod alloc_entry;
mod region;

use bootloader_api::info::MemoryRegion;
pub use region::{RegionAllocator, ALLOC_ENTRY_SIZE};
