#![no_std]
#![no_main]
#![allow(unused, dead_code)]

mod logger;

use crate::logger::KernelLogger;
use bootloader_api::{
    info::{FrameBuffer, PixelFormat},
    BootInfo,
};
use core::{panic::PanicInfo, ptr};
use log::{info, LevelFilter, Log};

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    let mut framebuf = boot_info.framebuffer.take().unwrap();
    let info = framebuf.info();
    let buffer = framebuf.into_buffer();

    KernelLogger::init(buffer, info);
    
    info!("Hello, world!");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

bootloader_api::entry_point!(kernel_main);
