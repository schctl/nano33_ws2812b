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
// 
// See the docs for more information.
// https://docs.rs/cortex-m-rt/latest/cortex_m_rt/
#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}
