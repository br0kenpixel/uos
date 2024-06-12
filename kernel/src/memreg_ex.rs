use bootloader_api::info::MemoryRegion;
use core::{
    ptr::{self, slice_from_raw_parts_mut, NonNull},
    slice,
};

pub trait MemoryRegionEx {
    fn size(&self) -> usize;
    fn as_slice(&self, phy_mem_offset: u64) -> &'static mut [u8];
}

impl MemoryRegionEx for MemoryRegion {
    fn size(&self) -> usize {
        (self.end - self.start) as usize
    }

    fn as_slice(&self, phy_mem_offset: u64) -> &'static mut [u8] {
        let start = self.start + phy_mem_offset;
        let length = self.size();

        unsafe { slice::from_raw_parts_mut(start as *mut u8, length) }
    }
}
