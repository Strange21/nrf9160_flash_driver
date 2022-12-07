#![no_std]
#![no_main]

use cortex_m_rt::entry;

#[entry]
fn main() -> !{
    unimplemented!();
}

#[panic_handler] // panicking behavior
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
