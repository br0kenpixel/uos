//! Minimal 16550 UART driver for COM1, used to mirror log output to the
//! host terminal via QEMU's `-serial stdio`.

use core::{arch::asm, fmt};

/// Base I/O port of COM1.
const PORT: u16 = 0x3F8;

/// Write a byte to an I/O port.
unsafe fn outb(port: u16, value: u8) {
    unsafe {
        asm!("out dx, al", in("dx") port, in("al") value, options(nomem, nostack, preserves_flags));
    }
}

/// Read a byte from an I/O port.
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    unsafe {
        asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack, preserves_flags));
    }
    value
}

/// A handle to the COM1 serial port.
pub struct SerialPort;

impl SerialPort {
    /// Initialize COM1 at 38400 baud, 8N1.
    pub fn init() -> Self {
        unsafe {
            outb(PORT + 1, 0x00); // disable interrupts
            outb(PORT + 3, 0x80); // enable DLAB (set baud rate divisor)
            outb(PORT, 0x03); // divisor low byte (115200 / 3 = 38400 baud)
            outb(PORT + 1, 0x00); // divisor high byte
            outb(PORT + 3, 0x03); // 8 bits, no parity, one stop bit
            outb(PORT + 2, 0xC7); // enable FIFO, clear them, 14-byte threshold
            outb(PORT + 4, 0x0B); // IRQs enabled, RTS/DSR set
        }
        Self
    }

    /// Whether the transmit holding register is empty and ready for a byte.
    fn is_transmit_empty() -> bool {
        unsafe { inb(PORT + 5) & 0x20 != 0 }
    }

    /// Send a single byte, spinning until the port is ready.
    fn send(byte: u8) {
        while !Self::is_transmit_empty() {}
        unsafe { outb(PORT, byte) };
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            // Terminals expect CRLF line endings.
            if byte == b'\n' {
                Self::send(b'\r');
            }
            Self::send(byte);
        }
        Ok(())
    }
}
