#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal as hal;
use hal::{clock::MHz8, prelude::*, Peripherals};
use inf1900_robot_hal::device::{read_input, set_twoway_del, Color};
use panic_abort as _;

#[hal::entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut pinb0 = pins.pb0.into_output();
    let mut pinb1 = pins.pb1.into_output();
    let pind2 = pins.pd2.into_floating_input();
    let mut counter: u8 = 0;

    let mut delay = hal::delay::Delay::<MHz8>::new();
    loop {
        if !read_input(&pind2) {
            while !read_input(&pind2) && counter <= 120 {
                counter += 1;
                delay.delay_ms(90u16);
            }
            for _ in 0..5 {
                set_twoway_del(&mut pinb0, &mut pinb1, Color::Green);
                delay.delay_ms(50u16);
                set_twoway_del(&mut pinb0, &mut pinb1, Color::None);
                delay.delay_ms(50u16);
            }
            delay.delay_ms(2000u16);
            for _ in 0..counter {
                set_twoway_del(&mut pinb0, &mut pinb1, Color::Red);
                delay.delay_ms(250u16);
                set_twoway_del(&mut pinb0, &mut pinb1, Color::None);
                delay.delay_ms(250u16);
            }
            set_twoway_del(&mut pinb0, &mut pinb1, Color::Green);
            delay.delay_ms(1000u16);
            set_twoway_del(&mut pinb0, &mut pinb1, Color::None);
        }
    }
}
