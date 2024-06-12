use linked_list_allocator::LockedHeap;
use os::OsAllocator;

pub mod kernel;
pub mod os;
pub mod region;

#[global_allocator]
pub static ALLOCATOR: OsAllocator = OsAllocator::uninit();
