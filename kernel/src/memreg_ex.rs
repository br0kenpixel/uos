use bootloader_api::info::MemoryRegion;
use core::ptr::{self, NonNull};

pub trait MemoryRegionEx {
    fn size(&self) -> usize;
    fn into_ptr(self) -> NonNull<u8>;
}

impl MemoryRegionEx for MemoryRegion {
    fn size(&self) -> usize {
        (self.end - self.start) as usize
    }

    fn into_ptr(self) -> NonNull<u8> {
        unsafe { NonNull::new_unchecked(self.start as *mut u8) }
    }
}
