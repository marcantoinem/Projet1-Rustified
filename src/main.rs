#![no_std]
#![no_main]
#![feature(asm_experimental_arch, unsize)]

mod device;
mod music;

use atmega_hal as hal;
use avr_hal_generic::simple_pwm::IntoPwmPin;
use hal::clock::MHz8;
use hal::prelude::*;
use hal::simple_pwm::{Prescaler, Timer1Pwm};
use music::Music;
use panic_abort as _;

#[avr_device::entry]
fn sound() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let mut clock = hal::delay::Delay::<MHz8>::new();
    // let mut pina0 = pins.pa0.into_output();
    // let mut pina1 = pins.pa1.into_output();
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Direct);
    let mut pind4 = pins.pd4.into_output().into_pwm(&timer1);
    let mut pind5 = pins.pd5.into_output().into_pwm(&timer1);

    loop {
        // Inspired by https://github.com/robsoncouto/arduino-songs
        Music::TETRIS.play_pwm(&mut clock, &mut pind4, &mut pind5);
        clock.delay_ms(1000u16);
    }
}
