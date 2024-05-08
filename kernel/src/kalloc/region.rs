use super::alloc_entry::AllocEntry;
use crate::memreg_ex::MemoryRegionEx;
use bootloader_api::info::MemoryRegion;
use core::{mem::size_of, ops::Add, ptr::slice_from_raw_parts_mut, slice};
use log::debug;

pub const ALLOC_ENTRY_SIZE: usize = size_of::<AllocEntry>();

pub struct RegionAllocator {
    mem: &'static mut [u8],
    allocs: &'static mut [AllocEntry],
}

impl RegionAllocator {
    pub fn new(region: MemoryRegion, phys_mem_offset: u64, n_allocs: usize) -> Self {
        let reserve_bytes = size_of::<AllocEntry>() * n_allocs;

        Self::new_raw(region, phys_mem_offset, reserve_bytes)
    }

    pub fn new_raw(region: MemoryRegion, phys_mem_offset: u64, reserve_bytes: usize) -> Self {
        assert!(
            reserve_bytes <= region.size() / 3,
            "Cannot reserve more than 1/3rd of the region"
        );

        let start_addr = phys_mem_offset + region.start;
        let region_slice =
            unsafe { slice::from_raw_parts_mut(start_addr as *mut u8, region.size()) };

        let region_split = region_slice.split_at_mut(reserve_bytes);
        let mem_space = region_split.1;
        let allocs_space = region_split.0;
        let n_allocs = allocs_space.len() / size_of::<AllocEntry>();

        let raw_allocs_ptr = allocs_space.as_mut_ptr();
        let allocs_ptr = raw_allocs_ptr.cast::<AllocEntry>();
        let n_allocs = unsafe { slice::from_raw_parts_mut(allocs_ptr, n_allocs) };

        mem_space.fill(0);
        n_allocs.fill(AllocEntry::EMPTY);

        Self {
            mem: mem_space,
            allocs: n_allocs,
        }
    }

    pub const fn mem_size(&self) -> usize {
        self.mem.len()
    }

    pub const fn max_allocs(&self) -> usize {
        self.allocs.len()
    }

    pub fn free(&self) -> usize {
        self.mem_size() - self.in_use()
    }

    pub fn in_use(&self) -> usize {
        self.allocs
            .iter()
            .filter(|entry| entry.is_some())
            .map(MemoryRegionEx::size)
            .sum()
    }
}
