#![no_std]

pub mod device;
pub mod gpio;
pub mod spi;

/// Logic state.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bit {
    Low = 0,
    High = 1,
}

/// Three-channel pixel color.
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

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    #[inline]
    pub(crate) const fn grb_channel(self, idx: usize) -> u8 {
        match idx {
            0 => self.g,
            1 => self.r,
            2 => self.b,
            _ => unreachable!(),
        }
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

/// Generic driver configuration options.
pub struct Configuration {
    pub gpio_port: device::GpioPort,
    pub gpio_pin: u8,
    pub max_stack: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            gpio_port: device::GpioPort::P0,
            gpio_pin: 0,
            max_stack: 4096,
        }
    }
}
