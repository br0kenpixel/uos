#![no_std]
#![no_main]
#![allow(unused, dead_code)]
#![feature(allocator_api)]

//extern crate alloc;

mod kernel_alloc;
mod logger;
mod memreg_ex;

use crate::{
    kernel_alloc::{find_best_memory_region, KernelAlloc},
    logger::KernelLogger,
    memreg_ex::MemoryRegionEx,
};
use bootloader_api::{
    config::Mapping,
    info::{FrameBuffer, MemoryRegionKind, PixelFormat},
    BootInfo,
};
use core::{panic::PanicInfo, ptr};
use log::{debug, info, LevelFilter, Log};

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let mut framebuf = boot_info.framebuffer.take().unwrap();
    let info = framebuf.info();
    let buffer = framebuf.into_buffer();
    //let allocator = KernelAlloc::new(, len)

    //let mut data = alloc::vec::Vec::with_capacity(10);
    //data.push("aaaaa");

    KernelLogger::init(buffer, info);

    info!("Hello, world!");
    //info!("{boot_info:#?}");

    let best_region = find_best_memory_region(&boot_info.memory_regions);
    debug!(
        "Found memory region for allocator: 0x{:X}, len: {}",
        best_region.start,
        best_region.size(),
    );
    //let allocator = KernelAlloc::new(best_region.into_ptr(), best_region.size());
    //info!("Allocator ready");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

const CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.kernel_stack_size = 100 * 1024;
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

bootloader_api::entry_point!(kernel_main, config = &CONFIG);
