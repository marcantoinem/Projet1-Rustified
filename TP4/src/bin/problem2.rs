#![no_std]
#![no_main]

use atmega_hal as hal;
use hal::prelude::*;
use panic_abort as _;

#[hal::entry]
fn problem2() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    loop {}
}
