#![no_std]
#![no_main]
#![allow(unused, dead_code)]

mod logger;
mod mem_stats;
mod memreg_ex;

use crate::{logger::KernelLogger, mem_stats::mem_stats, memreg_ex::MemoryRegionEx};
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
    KernelLogger::init(buffer, info);

    info!("Hello, world!");
    mem_stats(&boot_info.memory_regions);

    loop {}
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
