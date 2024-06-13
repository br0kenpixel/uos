use os::OsAllocator;

pub mod kernel;
mod meta;
pub mod os;
pub mod region;

#[global_allocator]
pub static ALLOCATOR: OsAllocator = OsAllocator::uninit();
