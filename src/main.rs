#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use atmega_hal as hal;
use hal::port::{mode::Output, Pin, PA0, PA1};
use hal::prelude::*;
use panic_halt as _;

enum Color {
    RED,
    GREEN,
}

impl Color {
    fn set_del(self, pina0: &mut Pin<Output, PA0>, pina1: &mut Pin<Output, PA1>) {
        match self {
            Color::RED => {
                pina0.set_low();
                pina1.set_high()
            }
            Color::GREEN => {
                pina0.set_high();
                pina1.set_low()
            }
        }
    }
}

#[avr_device::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    let pind2 = pins.pd2.into_floating_input();

    let mut clock = hal::delay::Delay::<hal::clock::MHz8>::new();

    loop {
        let first_read = pind2.is_high();
        clock.delay_ms(10u16);
        let second_read = pind2.is_high();
        match first_read & second_read {
            true => Color::GREEN.set_del(&mut pina0, &mut pina1),
            false => Color::RED.set_del(&mut pina0, &mut pina1),
        }
    }
}
