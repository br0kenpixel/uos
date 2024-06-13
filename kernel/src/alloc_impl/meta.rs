use crate::memreg_ex::MemoryRegionEx;
use bootloader_api::info::{MemoryRegion, MemoryRegionKind};

pub struct AllocationMetadata {
    start: u64,
    end: u64,
}

impl AllocationMetadata {
    pub const fn start(&self) -> u64 {
        self.start
    }

    pub const fn end(&self) -> u64 {
        self.end
    }
}

impl From<MemoryRegion> for AllocationMetadata {
    fn from(value: MemoryRegion) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl MemoryRegionEx for AllocationMetadata {
    fn size(&self) -> usize {
        (self.end - self.start) as usize
    }

    fn as_slice(&self, phy_mem_offset: u64) -> &'static mut [u8] {
        let reg = MemoryRegion {
            start: self.start,
            end: self.end,
            kind: MemoryRegionKind::Usable,
        };

        reg.as_slice(phy_mem_offset)
    }
}
