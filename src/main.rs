#![no_std]
#![no_main]
#![feature(asm_experimental_arch, unsize)]

mod device;
mod music;

use atmega_hal as hal;
use hal::clock::MHz8;
use hal::prelude::*;
use music::TETRIS;
use panic_abort as _;

#[avr_device::entry]
fn sound() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    let mut clock = hal::delay::Delay::<MHz8>::new();

    loop {
        // Inspired by https://github.com/robsoncouto/arduino-songs
        TETRIS.play(&mut clock, &mut pina0, &mut pina1);
        clock.delay_ms(1000u16);
    }
}
