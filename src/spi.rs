//! Driver implemented using SPI peripheral.

use core::slice;

use crate::device::{self, Peripherals};
use crate::{Configuration, Rgb};

const SPI_POOL_SIZE: usize = 1 << 13;
static mut SPI_POOL: [u8; SPI_POOL_SIZE] = [0; SPI_POOL_SIZE];
static mut SPI_PTR: usize = 0;

pub struct Driver {
    pool_ptr: usize,
    stack: usize,
}

impl Driver {
    pub fn new(cfg: Configuration, pphl: &Peripherals) -> Self {
        assert!(
            device::check_gpio_config(cfg.gpio_port, cfg.gpio_pin),
            "unsuccessful gpio check"
        );
        device::configure_gpio(cfg.gpio_port, cfg.gpio_pin, pphl);

        let pool_ptr = unsafe {
            let current = SPI_PTR;
            SPI_PTR += cfg.max_stack;

            if SPI_PTR >= SPI_POOL_SIZE {
                panic!("reached grouped SPI driver stack pool");
            }

            current as usize
        };

        pphl.SPIM0
            .config
            .write(|w| unsafe { w.bits(pphl.SPIM0.config.read().bits() & 0x110) }); // MSB bit order
        pphl.SPIM0
            .txd
            .ptr
            .write(|w| unsafe { w.bits(SPI_POOL.as_ptr().offset(pool_ptr as _) as _) }); // data pointer
        pphl.SPIM0
            .frequency
            .write(|w| unsafe { w.bits(0x40000000) }); // 4 Mbps - 250ns bit time
        pphl.SPIM0
            .psel
            .mosi
            .write(|w| unsafe { w.bits(cfg.gpio_pin as u32 | ((cfg.gpio_port as u32) << 5)) }); // gpio pin
        pphl.SPIM0.enable.write(|w| w.enable().enabled());

        Self {
            pool_ptr,
            stack: cfg.max_stack,
        }
    }

    pub fn begin_transaction<'a>(&self, data: &'a [Rgb]) -> Transaction<'a> {
        Transaction {
            data,
            written: 0,
            pool_ptr: self.pool_ptr as u32,
            stack: self.stack,
        }
    }
}

pub struct Transaction<'a> {
    pub data: &'a [Rgb],
    written: usize,
    pool_ptr: u32,
    stack: usize,
}

impl<'a> Transaction<'a> {
    pub fn completed(&self) -> bool {
        self.data.len() == self.written * 3
    }

    pub fn drive(&mut self) {
        let mut written = 0;

        let buffer = |w| unsafe {
            slice::from_raw_parts_mut(
                SPI_POOL
                    .as_mut_ptr()
                    .offset(self.pool_ptr as _)
                    .offset(w as _),
                self.stack - w,
            )
        };

        for i in (self.written / 3)..self.data.len() {
            for channel in (self.written % 3)..3 {
                let (written_c, bits) =
                    self.encode_byte(self.data[i].grb_channel(channel), (buffer)(written));
                if bits != 8 {
                    break;
                } else {
                    written += written_c;
                }
                if written >= self.stack {
                    break;
                }
            }
        }

        self.written += written;

        let spim0 = unsafe { Peripherals::steal().SPIM0 };
        spim0.txd.maxcnt.write(|w| unsafe { w.bits(written as _) });
        spim0.tasks_start.write(|w| unsafe { w.bits(1) });
    }

    /// Encode and write a single byte to our TXD buffer.
    ///
    /// # Returns
    /// - `.0`: number of bytes written to the buffer
    /// - `.1`: number of bits encoded
    #[rustfmt::skip]
    fn encode_byte(&self, byte: u8, buffer: &mut [u8]) -> (usize, usize) {
        let mut written = 0;
        let mut encoded = 0;

        for i in (0..8).step_by(2).rev() {
            if written >= buffer.len() {
                break;
            }

            let bit0 = (byte >> i) & 0x1;
            let bit1 = (byte >> (i - 1)) & 0x1;

            if bit0 == 0 && bit1 == 0 {
                buffer[written] = 0b1000_1000; written += 1; encoded += 2;
            } else if bit0 == 1 && bit1 == 1 {
                if written + 2 <= buffer.len() {
                    buffer[written] = 0b11100_111; written += 1;
                    buffer[written] = 0b11100_111; written += 1; encoded += 2;
                } else {
                    break;
                }
            } else if bit0 == 0 && bit1 == 1 {
                buffer[written] = 0b100_11100; written += 1; encoded += 2;
            } else if bit0 == 1 && bit1 == 0 {
                buffer[written] = 0b11100_100; written += 1; encoded += 2;
            } else {
                unreachable!();
            }
        }

        (written, encoded)
    }
}
