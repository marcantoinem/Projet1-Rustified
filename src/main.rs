#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use atmega_hal as hal;
use hal::clock::MHz8;
use hal::delay::Delay;
use hal::port::mode::{Floating, Input, Output};
use hal::port::{Pin, PinOps};
use hal::prelude::*;
use panic_abort as _;

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

// fn pitch_to_frequency(pitch: u8) -> f32 {
//     let float = 440f32 >> 2;
//     // 2f32.powi(pitch);
//     // (2f32.powf((pitch as f32 - 69.0) / 12.0)) as u16
// }

#[inline(never)]
fn make_sound<P1, P2>(
    clock: &mut Delay<MHz8>,
    pin0: &mut Pin<Output, P1>,
    pin1: &mut Pin<Output, P2>,
    frequency: u16,
    time_ms: u32,
) where
    P1: PinOps,
    P2: PinOps,
{
    let waiting_time = (1.0 / (frequency as f32) * 1_000_000.0) as u32;
    let number_of_cycle = time_ms as u32 * frequency as u32 / 2000;
    for _ in 0..number_of_cycle {
        pin0.set_high();
        pin1.set_low();
        clock.delay_us(waiting_time);
        pin0.set_low();
        pin1.set_high();
        clock.delay_us(waiting_time);
    }
}

#[avr_device::entry]
fn sound() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let notes = [
        659, 587, 523, 587, 659, 783, 783, 523, 523, 587, 587, 659, 587, 523, 587, 659, 783, 783,
        659, 523, 523, 523,
    ];
    loop {
        for note in notes {
            make_sound(&mut clock, &mut pina0, &mut pina1, note as u16, 200);
            clock.delay_ms(100u16);
        }
        clock.delay_ms(1000u16);
    }
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
