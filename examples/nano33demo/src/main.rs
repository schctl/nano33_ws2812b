#![no_std]
#![no_main]

use driver::{device, Configuration, Rgb};
use nrf5x_ws2812b as driver;

use panic_halt as _;

/// Start high speed crystal oscillator (64Mhz).
fn setup_hf_clock(clock: &device::CLOCK) {
    clock.tasks_hfclkstart.write(|w| unsafe { w.bits(1) });
    while clock.events_hfclkstarted.read().bits() == 0 {}
}

#[cortex_m_rt::entry]
fn main() -> ! {
    let cpu_pphl = cortex_m::Peripherals::take().unwrap();
    let mcu_pphl = device::Peripherals::take().unwrap();

    setup_hf_clock(&mcu_pphl.CLOCK);

    let mut delay_provider = cortex_m::delay::Delay::new(cpu_pphl.SYST, 64_000_000);

    let ws2812b = driver::gpio::Driver::new(
        Configuration {
            gpio_port: device::GpioPort::P0,
            gpio_pin: 23,
        },
        &mcu_pphl,
    );

    let leds = [Rgb::H_RED, Rgb::ZERO, Rgb::H_GREEN];

    loop {
        ws2812b.write(leds.into_iter(), &mut delay_provider);
        delay_provider.delay_ms(500);
    }
}
