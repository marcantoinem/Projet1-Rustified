#![no_std]
#![no_main]

use atmega_hal as hal;
use hal::clock::MHz8;
use hal::prelude::*;
use inf1900_robot_hal::music::Music;
use panic_abort as _;

#[hal::entry]
fn sound() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    loop {
        // Inspired by https://github.com/robsoncouto/arduino-songs
        Music::NEVERGONNAGIVEYOUUP.play(&mut clock, &mut pina0, &mut pina1, 103);
        Music::THELIONSLEEPSTONIGHT.play(&mut clock, &mut pina0, &mut pina1, 103);
        clock.delay_ms(1000u16);
    }
}
