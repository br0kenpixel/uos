use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
    slice,
};

use bootloader_api::info::{MemoryRegion, MemoryRegionKind};

use crate::memreg_ex::MemoryRegionEx;

const MAX_ALLOCS: usize = 4096;

pub struct KernelAlloc {
    memory: &'static mut [u8],
    allocs: [Option<AllocatedRegion>; MAX_ALLOCS],
}

#[derive(Debug, Clone, Copy)]
struct AllocatedRegion {
    start: NonNull<[u8]>,
    len: usize,
}

impl KernelAlloc {
    pub fn new(memory: NonNull<u8>, len: usize) -> Self {
        let memory = unsafe { slice::from_raw_parts_mut(memory.as_ptr(), len) };

        Self {
            memory,
            allocs: [None; MAX_ALLOCS],
        }
    }

    fn allocate_first_fit(&mut self, amount: usize) -> Option<NonNull<[u8]>> {
        todo!()
    }

    fn release(&mut self, region: NonNull<[u8]>) {
        todo!()
    }
}

pub fn find_best_memory_region(regions: &[MemoryRegion]) -> MemoryRegion {
    let mut usable_regions = regions
        .iter()
        .filter(|region| region.kind == MemoryRegionKind::Usable);
    let mut best_region = usable_regions.next().unwrap();

    for region in usable_regions {
        let size = region.size();
        let best_size = best_region.size();

        if size > best_size {
            best_region = region;
        }
    }

    *best_region
}
