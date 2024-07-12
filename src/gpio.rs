//! Driver implemented using GPIO pins and TIMER0.

use core::arch::asm;
use core::mem::transmute;

use cortex_m::delay::Delay;

use crate::device::Peripherals;
use crate::{Bit, Rgb};

pub struct Driver {
    gpio_port: u8,
    gpio_pin: u8,
}

impl Driver {
    pub fn new(gpio_port: u8, gpio_pin: u8, pphl: &Peripherals) -> Self {
        let this = Self {
            gpio_port,
            gpio_pin,
        };

        this.configure_gpio(pphl);
        this
    }

    fn configure_gpio(&self, pphl: &Peripherals) {
        match self.gpio_port {
            0 => pphl.P0.pin_cnf[self.gpio_pin as usize].write(|w| {
                w.dir().output();
                w.input().disconnect();
                w.pull().disabled();
                w.drive().h0h1();
                w.sense().disabled();
                w
            }),
            #[cfg(feature = "nrf52840")]
            1 => pphl.P1.pin_cnf[self.gpio_pin as usize].write(|w| {
                w.dir().output();
                w.input().disconnect();
                w.pull().disabled();
                w.drive().h0h1();
                w.sense().disabled();
                w
            }),
            _ => panic!("unsupported GPIO port `{0}`", self.gpio_port),
        }
    }

    #[inline]
    pub unsafe fn write_gpio(&self, bit: Bit) {
        #[cfg(feature = "nrf52840")]
        let gpio_out = (0x50000000 + (0x300 * self.gpio_port as u32) + 0x504) as *mut u32;

        *gpio_out = (*gpio_out) & !(1 << self.gpio_pin) | ((bit as u32) << self.gpio_pin);
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
