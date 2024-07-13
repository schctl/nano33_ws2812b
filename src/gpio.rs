//! Driver implemented using GPIO pins and bit-banging.

use core::arch::asm;
use core::mem::transmute;

use cortex_m::delay::Delay;
use volatile::VolatilePtr;

use crate::device::{self, Peripherals};
use crate::{Bit, Configuration, Rgb};

pub struct Driver {
    gpio_pin: u8,
    gpio_out: VolatilePtr<'static, u32>,
}

impl Driver {
    pub fn new(cfg: Configuration, pphl: &Peripherals) -> Self {
        assert!(
            device::check_gpio_config(cfg.gpio_port, cfg.gpio_pin),
            "unsuccessful gpio check"
        );
        device::configure_gpio(cfg.gpio_port, cfg.gpio_pin, pphl);

        Self {
            gpio_pin: cfg.gpio_pin,
            gpio_out: unsafe { VolatilePtr::new(device::gpio_out_ptr(cfg.gpio_port)) },
        }
    }

    #[inline]
    fn write_gpio(&self, bit: Bit) {
        self.gpio_out
            .update(|b| b & !(1 << self.gpio_pin) | ((bit as u32) << self.gpio_pin))
    }

    #[inline]
    unsafe fn write_bit(&self, bit: Bit) {
        // T0H - 375   ns
        // T0L - 687.5 ns
        // T1H - 687.5 ns
        // T1L - 625   ns

        match bit {
            Bit::Low => {
                self.write_gpio(Bit::High);

                for _ in 0..24 {
                    asm!("nop");
                }

                self.write_gpio(Bit::Low);

                for _ in 0..44 {
                    asm!("nop");
                }
            }
            Bit::High => {
                self.write_gpio(Bit::High);

                for _ in 0..44 {
                    asm!("nop");
                }

                self.write_gpio(Bit::Low);

                for _ in 0..40 {
                    asm!("nop");
                }
            }
        }
    }

    #[inline]
    unsafe fn write_byte(&self, byte: u8) {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            self.write_bit(transmute(bit));
        }
    }

    #[inline]
    unsafe fn write_rgb(&self, rgb: Rgb) {
        self.write_byte(rgb.g);
        self.write_byte(rgb.r);
        self.write_byte(rgb.b);
    }

    pub fn write<I>(&self, iter: I, delay: &mut Delay)
    where
        I: Iterator<Item = Rgb>,
    {
        for pixel in iter {
            unsafe {
                self.write_rgb(pixel);
            }
        }

        delay.delay_us(50);
    }
}
