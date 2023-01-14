use atmega_hal as hal;
use hal::clock::MHz8;
use hal::delay::Delay;
use hal::port::mode::{Floating, Input, Output};
use hal::port::{Pin, PinOps};
use hal::prelude::*;
enum _Color {
    RED,
    GREEN,
}

fn _set_twoway_del<P1, P2>(pina0: &mut Pin<Output, P1>, pina1: &mut Pin<Output, P2>, color: _Color)
where
    P1: PinOps,
    P2: PinOps,
{
    match color {
        _Color::RED => {
            pina0.set_low();
            pina1.set_high();
        }
        _Color::GREEN => {
            pina0.set_high();
            pina1.set_low();
        }
    }
}

fn _read_input_debounced<PIN>(clock: &mut Delay<MHz8>, pin: &Pin<Input<Floating>, PIN>) -> bool
where
    PIN: PinOps,
{
    let first_read = pin.is_high();
    clock.delay_ms(10_u16);
    let second_read = pin.is_high();
    first_read & second_read
}

fn _tp1() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();

    let pind2 = pins.pd2.into_floating_input();

    let mut clock = hal::delay::Delay::<MHz8>::new();

    loop {
        match _read_input_debounced(&mut clock, &pind2) {
            true => _set_twoway_del(&mut pina0, &mut pina1, _Color::GREEN),
            false => _set_twoway_del(&mut pina0, &mut pina1, _Color::RED),
        }
    }
}