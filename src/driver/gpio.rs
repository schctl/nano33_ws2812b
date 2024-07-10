//! Driver implemented using GPIO pins and TIMER0.

use super::{Bit, Rgb};
use crate::device;
use crate::device::Peripherals;

fn set_bit_ccr(timer: &device::TIMER0, bit: Bit) {
    // T0H - 375   ns - 6 ccr
    // T0L - 687.5 ns - 11 ccr
    // T1H - 687.5 ns - 11 ccr
    // T1L - 625   ns - 10 ccr

    match bit {
        Bit::High => {
            timer.cc[0].write(|w| unsafe { w.bits(6) });
            timer.cc[1].write(|w| unsafe { w.bits(6 + 11) });
        }
        Bit::Low => {
            timer.cc[0].write(|w| unsafe { w.bits(11) });
            timer.cc[1].write(|w| unsafe { w.bits(11 + 10) });
        }
    }
}

fn set_reset_ccr(timer: &device::TIMER0) {
    // 50 us
    timer.cc[0].write(|w| unsafe { w.bits(800) });
}

fn timer_wait(timer: &device::TIMER0, cc: usize) {
    while timer.events_compare[cc].read().bits() == 0 {}
    timer.events_compare[cc].write(|w| unsafe { w.bits(0) });
}

fn timer_clear(timer: &device::TIMER0) {
    timer.tasks_clear.write(|w| unsafe { w.bits(1) });
}

fn reset_sequence(timer: &device::TIMER0) {
    set_reset_ccr(timer);
    timer_clear(timer);
    timer_wait(timer, 0);
}

pub struct Driver {
    gpio_port: u8,
    gpio_pin: u8,
}

impl Driver {
    pub fn new(gpio_port: u8, gpio_pin: u8) -> Self {
        Self {
            gpio_port,
            gpio_pin,
        }
    }

    /// It is the user's responsibility to call this function before using the driver.
    pub fn configure_gpio(&self, pphl: &Peripherals) {
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

    /// It is the user's responsibility to call this function before using the driver.
    pub fn configure_timer(&self, pphl: &Peripherals) {
        let timer = &pphl.TIMER0;

        timer.mode.write(|w| unsafe { w.bits(0) }); // timer mode
        timer.bitmode.write(|w| unsafe { w.bits(0) }); // 16 bit timer

        timer.prescaler.write(|w| unsafe { w.bits(0) }); // 16Mhz freq

        timer.intenclr.write(|w| {
            w.compare0().set_bit();
            w.compare1().set_bit();
            w
        });

        timer.shorts.write(|w| w.compare1_clear().set_bit());
    }

    #[inline]
    fn write_gpio(&self, pphl: &Peripherals, bit: Bit) {
        let gpio_out = match self.gpio_port {
            0 => pphl.P0.out.as_ptr(),
            #[cfg(feature = "nrf52840")]
            1 => pphl.P1.out.as_ptr(),
            _ => panic!("unsupported GPIO port `{0}`", self.gpio_port),
        };

        unsafe {
            *gpio_out = (*gpio_out) ^ ((bit as u8 as u32) << self.gpio_pin);
        }
    }

    #[inline]
    fn write_bit(&self, pphl: &Peripherals, bit: Bit) {
        let timer = &pphl.TIMER0;

        set_bit_ccr(timer, bit);
        timer_clear(timer);

        self.write_gpio(pphl, Bit::High);
        timer_wait(timer, 0);

        self.write_gpio(pphl, Bit::Low);
        timer_wait(timer, 1);
    }

    #[inline]
    fn write_byte(&self, pphl: &Peripherals, byte: u8) {
        for i in 0..8 {
            let bit = (byte >> (7 - i)) & 1;
            self.write_bit(pphl, unsafe { core::mem::transmute(bit) });
        }
    }

    #[inline]
    fn write_rgb(&self, pphl: &Peripherals, rgb: Rgb) {
        self.write_byte(pphl, rgb.g);
        self.write_byte(pphl, rgb.r);
        self.write_byte(pphl, rgb.b);
    }

    pub fn write<I>(&self, pphl: &Peripherals, iter: I)
    where
        I: Iterator<Item = Rgb>,
    {
        for pixel in iter {
            self.write_rgb(pphl, pixel);
        }

        reset_sequence(&pphl.TIMER0);
    }
}
