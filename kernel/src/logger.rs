// src: https://github.com/rust-osdev/bootloader/blob/main/common/src/logger.rs

use bootloader_api::info::{FrameBufferInfo, PixelFormat};
use conquer_once::spin::OnceCell;
use core::{
    fmt::{self, Write},
    ptr,
};
use log::LevelFilter;
use noto_sans_mono_bitmap::{get_bitmap, get_bitmap_width, BitmapChar, BitmapHeight, FontWeight};
use spinning_top::Spinlock;

/// The global logger instance used for the `log` crate.
pub static LOGGER: OnceCell<KernelLogger> = OnceCell::uninit();

/// A [`Logger`] instance protected by a spinlock.
pub struct KernelLogger(Spinlock<Logger>);

/// Additional vertical space between lines
const LINE_SPACING: usize = 0;
/// Additional vertical space between separate log messages
const LOG_SPACING: usize = 2;

impl KernelLogger {
    pub fn init(framebuffer: &'static mut [u8], info: FrameBufferInfo) {
        let logger = LOGGER.get_or_init(move || Self::new(framebuffer, info));

        let _ = log::set_logger(logger);
        log::set_max_level(LevelFilter::Debug);
        log::debug!("Kernel logger initialized");
    }

    /// Create a new instance that logs to the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        Self(Spinlock::new(Logger::new(framebuffer, info)))
    }
}

impl log::Log for KernelLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let mut logger = self.0.lock();
        writeln!(logger, "{}:    {}", record.level(), record.args()).unwrap();
        logger.add_vspace(LOG_SPACING);
    }

    fn flush(&self) {}
}

/// Allows logging text to a pixel-based framebuffer.
pub struct Logger {
    framebuffer: &'static mut [u8],
    info: FrameBufferInfo,
    x_pos: usize,
    y_pos: usize,
}

impl Logger {
    /// Creates a new logger that uses the given framebuffer.
    pub fn new(framebuffer: &'static mut [u8], info: FrameBufferInfo) -> Self {
        let mut logger = Self {
            framebuffer,
            info,
            x_pos: 0,
            y_pos: 0,
        };
        logger.clear();
        logger
    }

    fn newline(&mut self) {
        self.y_pos += 14 + LINE_SPACING;
        self.carriage_return();
    }

    fn add_vspace(&mut self, space: usize) {
        self.y_pos += space;
    }

    fn carriage_return(&mut self) {
        self.x_pos = 0;
    }

    /// Erases all text on the screen.
    pub fn clear(&mut self) {
        self.x_pos = 0;
        self.y_pos = 0;
        self.framebuffer.fill(0);
    }

    const fn width(&self) -> usize {
        self.info.width
    }

    const fn height(&self) -> usize {
        self.info.height
    }

    fn write_char(&mut self, c: char) {
        match c {
            '\n' => self.newline(),
            '\r' => self.carriage_return(),
            c => {
                if self.x_pos >= self.width() {
                    self.newline();
                }
                const BITMAP_LETTER_WIDTH: usize =
                    get_bitmap_width(FontWeight::Regular, BitmapHeight::Size14);
                if self.y_pos >= (self.height() - BITMAP_LETTER_WIDTH) {
                    self.clear();
                }
                let bitmap_char = get_bitmap(c, FontWeight::Regular, BitmapHeight::Size14).unwrap();
                self.write_rendered_char(bitmap_char);
            }
        }
    }

    fn write_rendered_char(&mut self, rendered_char: BitmapChar) {
        for (y, row) in rendered_char.bitmap().iter().enumerate() {
            for (x, byte) in row.iter().enumerate() {
                self.write_pixel(self.x_pos + x, self.y_pos + y, *byte);
            }
        }
        self.x_pos += rendered_char.width();
    }

    fn write_pixel(&mut self, x: usize, y: usize, intensity: u8) {
        let pixel_offset = y * self.info.stride + x;
        let color = match self.info.pixel_format {
            PixelFormat::Rgb => [intensity, intensity, intensity / 2, 0],
            PixelFormat::Bgr => [intensity / 2, intensity, intensity, 0],
            PixelFormat::U8 => [if intensity > 200 { 0xf } else { 0 }, 0, 0, 0],
            _ => panic!(),
        };
        let bytes_per_pixel = self.info.bytes_per_pixel;
        let byte_offset = pixel_offset * bytes_per_pixel;
        self.framebuffer[byte_offset..(byte_offset + bytes_per_pixel)]
            .copy_from_slice(&color[..bytes_per_pixel]);
        let _ = unsafe { ptr::read_volatile(&self.framebuffer[byte_offset]) };
    }
}

unsafe impl Send for Logger {}
unsafe impl Sync for Logger {}

impl fmt::Write for Logger {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }
        Ok(())
    }
}
