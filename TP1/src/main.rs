#![no_std]
#![no_main]

use atmega_hal as hal;
use hal::clock::MHz8;
use inf1900_robot_hal::device::{read_input_debounced, set_twoway_del, Color};
use panic_abort as _;

#[hal::entry]
fn sound() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    let pind2 = pins.pd2.into_floating_input();
    loop {
        match read_input_debounced(&mut clock, &pind2) {
            true => set_twoway_del(&mut pina0, &mut pina1, Color::Amber),
            false => set_twoway_del(&mut pina0, &mut pina1, Color::Red),
        }
    }
}
