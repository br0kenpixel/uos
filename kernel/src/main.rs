#![no_std]
#![no_main]
#![allow(clippy::module_name_repetitions, clippy::cast_possible_truncation)]
//#![allow(unused, dead_code)]
#![feature(allocator_api)]

extern crate alloc;

mod alloc_impl;
mod cpuid;
mod heapless;
mod logger;
mod mem_stats;
mod memreg_ex;

use crate::{logger::KernelLogger, mem_stats::mem_stats};
use alloc::{string::String, vec};
use alloc_impl::{kernel::KernelAllocator, ALLOCATOR};
use bootloader_api::{config::Mapping, BootInfo};
use core::{arch::asm, panic::PanicInfo};
use log::{debug, info};
use ubyte::ToByteUnit;

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let framebuf = boot_info.framebuffer.take().unwrap();
    let info = framebuf.info();
    let buffer = framebuf.into_buffer();
    KernelLogger::init(buffer, info);

    {
        let allocator = KernelAllocator::new(
            &boot_info.memory_regions,
            boot_info.physical_memory_offset.into_option().unwrap(),
        );
        let size = allocator.total_mem();

        ALLOCATOR.init(allocator);
        debug!("Heap initialized, total memory: {}", size.bytes());
    }

    info!("Hello, world!");
    mem_stats(&boot_info.memory_regions);

    let _numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let _text = String::from("Hello, World! This is some example text! :)");

    debug!("Detecting CPU...");
    let cpu = cpuid::CpuInfo::default();
    debug!(
        "CPU: [{}] {}, {}xC {}xT",
        cpu.vendor(),
        cpu.brand(),
        cpu.physical_cores(),
        cpu.logical_cores()
    );

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
    config.kernel_stack_size = 1024 * 1024;
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

bootloader_api::entry_point!(kernel_main, config = &CONFIG);
