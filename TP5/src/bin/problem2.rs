#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal as hal;
use hal::{
    clock::MHz8,
    delay::Delay,
    pac::{EXINT, TC1},
    port::{
        mode::{Floating, Input},
        Pin, PD2,
    },
    Peripherals,
};

#[hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let mut pins = hal::pins!(dp);

    loop {
        // let b = nb::block!(serial.read()).void_unwrap();
        // ufmt::uwriteln!(&mut serial, "Got {}!\r", b).void_unwrap();
        // ufmt::uwriteln!(&mut serial, "Hello from the Robot !\r").void_unwrap();
    }
}
