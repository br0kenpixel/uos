use super::meta::AllocationMetadata;
use crate::memreg_ex::MemoryRegionEx;
use bootloader_api::info::MemoryRegion;
use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::{self, NonNull},
};
use linked_list_allocator::LockedHeap;
use log::debug;

pub struct RegionAllocator(LockedHeap, AllocationMetadata);

impl RegionAllocator {
    pub fn new(region: MemoryRegion, phys_mem_offset: u64) -> Self {
        let modified_region = MemoryRegion {
            start: region.start + phys_mem_offset,
            end: region.end + phys_mem_offset,
            kind: region.kind,
        };

        debug!(
            "region_allocd: Initializing allocator @ 0x{:X} ({}B)",
            modified_region.start,
            modified_region.size()
        );

        let heap_slice = region.as_slice(phys_mem_offset);
        let heap = unsafe { LockedHeap::new(heap_slice.as_mut_ptr(), heap_slice.len()) };

        Self(heap, modified_region.into())
    }

    pub const fn metadata(&self) -> &AllocationMetadata {
        &self.1
    }
}

unsafe impl Allocator for RegionAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        let size = layout.size() + layout.align();
        debug!(
            "region_allocd: Allocating {}B at 0x{:X}",
            size,
            self.metadata().start()
        );

        let result = self
            .0
            .lock()
            .allocate_first_fit(layout)
            .map_err(|()| AllocError)?;

        let slice = ptr::slice_from_raw_parts_mut(result.as_ptr(), size);
        let nonnull = unsafe { NonNull::new_unchecked(slice) };

        Ok(nonnull)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        debug!(
            "region_allocd: Freeing {:X} at 0x{:X}",
            ptr.addr(),
            self.metadata().start()
        );
        self.0.lock().deallocate(ptr, layout);
    }
}
