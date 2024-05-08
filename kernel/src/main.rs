#![no_std]
#![no_main]
#![allow(unused, dead_code, clippy::module_name_repetitions)]
#![feature(const_trait_impl)]

mod kalloc;
mod logger;
mod mem_stats;
mod memreg_ex;

use crate::{
    kalloc::RegionAllocator, logger::KernelLogger, mem_stats::mem_stats, memreg_ex::MemoryRegionEx,
};
use bootloader_api::{config::Mapping, info::MemoryRegionKind, BootInfo};
use core::{arch::asm, panic::PanicInfo};
use log::{debug, info};

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuf = boot_info.framebuffer.take().unwrap();
    let info = framebuf.info();
    let buffer = framebuf.into_buffer();
    KernelLogger::init(buffer, info);

    info!("Hello, world!");
    mem_stats(&boot_info.memory_regions);

    let mem = boot_info
        .memory_regions
        .iter()
        .filter(|region| region.kind == MemoryRegionKind::Usable)
        .max_by_key(|region| region.size())
        .unwrap();
    debug!("{mem:#?}");

    let phys_offset = boot_info.physical_memory_offset.into_option().unwrap();
    debug!("Physical memory starts at 0x{:X}", phys_offset);

    let ralloc = RegionAllocator::new(*mem, phys_offset, 10);

    loop {
        unsafe {
            asm!("NOP");
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    info!("{info:#?}");
    loop {}
}

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024;
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

bootloader_api::entry_point!(kernel_main, config = &CONFIG);
