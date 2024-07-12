#![no_std]

pub mod device;
pub mod gpio;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Bit {
    Low = 0,
    High = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub const ZERO: Self = Self { r: 0, g: 0, b: 0 };
    pub const H_RED: Self = Self { r: 127, g: 0, b: 0 };
    pub const H_GREEN: Self = Self { r: 0, g: 127, b: 0 };
    pub const H_BLUE: Self = Self { r: 0, g: 0, b: 127 };

    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<[u8; 3]> for Rgb {
    fn from(rgb: [u8; 3]) -> Self {
        Self {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        }
    }
}

pub struct Configuration {
    pub gpio_port: device::GpioPort,
    pub gpio_pin: u8,
}
