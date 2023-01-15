use atmega_hal as hal;
use hal::clock::MHz8;
use hal::delay::Delay;
use hal::port::mode::{Floating, Input, Output};
use hal::port::{Pin, PinOps};
use hal::prelude::*;
pub enum Color {
    RED,
    GREEN,
}

pub fn set_twoway_del<P1, P2>(
    pina0: &mut Pin<Output, P1>,
    pina1: &mut Pin<Output, P2>,
    color: Color,
) where
    P1: PinOps,
    P2: PinOps,
{
    match color {
        Color::RED => {
            pina0.set_low();
            pina1.set_high();
        }
        Color::GREEN => {
            pina0.set_high();
            pina1.set_low();
        }
    }
}

pub fn read_input_debounced<PIN>(clock: &mut Delay<MHz8>, pin: &Pin<Input<Floating>, PIN>) -> bool
where
    PIN: PinOps,
{
    let first_read = pin.is_high();
    clock.delay_ms(10_u16);
    let second_read = pin.is_high();
    first_read & second_read
}
