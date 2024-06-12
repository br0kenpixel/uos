#![no_std]
#![no_main]
#![allow(unused, dead_code)]
#![feature(allocator_api)]
#![feature(strict_provenance)]

//extern crate alloc;

mod logger;
mod mem_stats;
mod memreg_ex;

use crate::{logger::KernelLogger, mem_stats::mem_stats, memreg_ex::MemoryRegionEx};
use alloc_impl::{kernel::KernelAllocator, region::RegionAllocator};
use bootloader_api::{
    config::Mapping,
    info::{FrameBuffer, MemoryRegionKind, PixelFormat},
    BootInfo,
};
use core::{arch::asm, panic::PanicInfo, ptr};
use log::{debug, info, LevelFilter, Log};

mod alloc_impl;

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let mut framebuf = boot_info.framebuffer.take().unwrap();
    let info = framebuf.info();
    let buffer = framebuf.into_buffer();
    KernelLogger::init(buffer, info);

    let allocator = KernelAllocator::new(
        &boot_info.memory_regions,
        boot_info.physical_memory_offset.into_option().unwrap(),
    );

    info!("Hello, world!");
    mem_stats(&boot_info.memory_regions);

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
