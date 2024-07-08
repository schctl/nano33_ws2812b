#![no_std]
#![no_main]

// ------- Panic Handler -------

use core::panic::PanicInfo;

#[inline(never)]
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

// ------------------------------

// The `cortex_m_rt` crate handles some lower level stuff for our Cortex-M microprocessor that
// we don't necessarily want to write ourselves - like our entry point (reset handler) and memory layout.
// See the docs for more information. https://docs.rs/cortex-m-rt/latest/cortex_m_rt/

#[cortex_m_rt::entry]
fn main() -> ! {
    let _core_p = cortex_m::Peripherals::take().unwrap();
    let _board_p = nrf52840_pac::Peripherals::take().unwrap();

    let gpio0 = _board_p.P0;

    gpio0.pin_cnf[13].write(|w| {
        w.dir().output();
        w.input().disconnect();
        w.pull().disabled();
        w.drive().s0s1();
        w.sense().disabled();
        w
    });

    gpio0.outset.write(|w| w.pin13().set());

    loop {}
}
