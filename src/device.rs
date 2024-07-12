use core::ptr::NonNull;

#[cfg(feature = "nrf52840")]
pub use nrf52840_pac::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GpioPort {
    P0 = 0,
    #[cfg(feature = "nrf52840")]
    P1 = 1,
}

pub(crate) fn check_gpio_config(port: GpioPort, pin: u8) -> bool {
    match port {
        GpioPort::P0 => pin < 32,
        #[cfg(feature = "nrf52840")]
        GpioPort::P1 => pin < 16,
    }
}

pub(crate) fn configure_gpio(port: GpioPort, pin: u8, pphl: &Peripherals) {
    match port {
        GpioPort::P0 => pphl.P0.pin_cnf[pin as usize].write(|w| {
            w.dir().output();
            w.input().disconnect();
            w.pull().disabled();
            w.drive().h0h1();
            w.sense().disabled();
            w
        }),
        #[cfg(feature = "nrf52840")]
        GpioPort::P1 => pphl.P1.pin_cnf[pin as usize].write(|w| {
            w.dir().output();
            w.input().disconnect();
            w.pull().disabled();
            w.drive().h0h1();
            w.sense().disabled();
            w
        }),
    }
}

pub(crate) const unsafe fn gpio_out_ptr(port: GpioPort) -> NonNull<u32> {
    NonNull::new_unchecked((0x50000000 + (0x300 * port as u32) + 0x504) as *mut _)
}
