use bootloader_api::info::MemoryRegion;

#[const_trait]
pub trait MemoryRegionEx {
    fn size(&self) -> usize;
}

impl const MemoryRegionEx for MemoryRegion {
    fn size(&self) -> usize {
        (self.end - self.start) as usize
    }
}
