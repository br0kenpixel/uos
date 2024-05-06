use bootloader_api::info::{MemoryRegion, MemoryRegionKind};
use log::debug;

use crate::memreg_ex::MemoryRegionEx;

pub fn mem_stats(regions: &[MemoryRegion]) {
    let mut total_regions = 0;
    let mut total_mem = 0;
    let mut usable_regions = 0;
    let mut usable_mem = 0;

    for region in regions {
        total_regions += 1;
        total_mem += region.size();

        if region.kind == MemoryRegionKind::Usable {
            usable_regions += 1;
            usable_mem = region.size();
        }
    }

    debug!(
        "Found {} memory regions, {} usable, {} reserved",
        total_regions,
        usable_regions,
        regions.len() - usable_regions
    );
    debug!(
        "Available memory: {}B total, {}B usable, {}B reserved",
        total_mem,
        usable_mem,
        total_mem - usable_mem
    );
}
