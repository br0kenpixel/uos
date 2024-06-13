use super::region::RegionAllocator;
use bootloader_api::info::{MemoryRegion, MemoryRegionKind};
use core::{
    alloc::{AllocError, Allocator, Layout},
    ptr::NonNull,
};
use log::{debug, warn};

const MAX_MEM_REGIONS: usize = 4096;
const EMPTY_ALLOC_SLOT: Option<RegionAllocator> = Option::<RegionAllocator>::None;
pub struct KernelAllocator([Option<RegionAllocator>; MAX_MEM_REGIONS]);

impl KernelAllocator {
    pub fn new(region_allocs: &[MemoryRegion], phys_mem_offset: u64) -> Self {
        assert!(!region_allocs.is_empty());
        debug!("kernel_allocd: Initializing allocator");

        let mut allocators = [EMPTY_ALLOC_SLOT; MAX_MEM_REGIONS];
        let mut i = 0;
        for region in region_allocs {
            if region.kind != MemoryRegionKind::Usable {
                continue;
            }

            if i == MAX_MEM_REGIONS {
                warn!("kernel_allocd: Breaking allocation creation, no free slots left");
                break;
            }

            allocators[i] = Some(RegionAllocator::new(*region, phys_mem_offset));
            i += 1;
        }

        debug!("kernel_allocd: Initialized allocator with {} regions", i);

        Self(allocators)
    }

    pub fn allocators(&self) -> impl Iterator<Item = &RegionAllocator> {
        self.0
            .iter()
            .filter(|slot| slot.is_some())
            .map(|slot| unsafe { slot.as_ref().unwrap_unchecked() })
    }
}

unsafe impl Allocator for KernelAllocator {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        for allocator in self.allocators() {
            match allocator.allocate(layout) {
                Ok(ptr) => return Ok(ptr),
                Err(_) => continue,
            }
        }

        Err(AllocError)
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        for allocator in self.allocators() {
            let allocator_region =
                (allocator.region().start as usize)..(allocator.region().end as usize);

            if allocator_region.contains(&ptr.addr().get()) {
                allocator.deallocate(ptr, layout);
                return;
            }
        }

        warn!("kernel_allocd: Could not find holding allocator");
    }
}
