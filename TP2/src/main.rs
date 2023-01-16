#![no_std]
#![no_main]

use atmega_hal as hal;
use hal::clock::MHz8;
use hal::prelude::*;
use inf1900_robot_hal::device::{read_input_debounced, set_twoway_del, Color};
use panic_abort as _;

enum StateButton {
    Wait,
    Pressed,
    Released,
}

fn _problem1() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    let pind2 = pins.pd2.into_floating_input();
    let mut state = StateButton::Wait;
    let mut counter = 0;
    loop {
        state = match (&state, read_input_debounced(&mut clock, &pind2)) {
            (StateButton::Wait, true) => StateButton::Pressed,
            (StateButton::Pressed, false) => StateButton::Released,
            (StateButton::Released, _) => {
                counter += 1;
                StateButton::Wait
            }
            (_, _) => state,
        };
        if counter == 3 {
            set_twoway_del(&mut pina0, &mut pina1, Color::Green);
            clock.delay_ms(2000u16);
            set_twoway_del(&mut pina0, &mut pina1, Color::None);
            counter = 0;
        }
    }
}

enum StateProblem2 {
    Start,
    FirstPress,
    FirstRelease,
    SecondPress,
    SecondRelease,
    ThirdPress,
}

#[hal::entry]
fn _problem2() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    let pind2 = pins.pd2.into_floating_input();
    let mut state = StateProblem2::Start;
    loop {
        state = match (&state, read_input_debounced(&mut clock, &pind2)) {
            (StateProblem2::Start, true) => StateProblem2::FirstPress,
            (StateProblem2::FirstPress, false) => StateProblem2::FirstRelease,
            (StateProblem2::FirstRelease, true) => StateProblem2::SecondPress,
            (StateProblem2::SecondPress, false) => StateProblem2::SecondRelease,
            (StateProblem2::SecondRelease, true) => StateProblem2::ThirdPress,
            (StateProblem2::ThirdPress, false) => StateProblem2::Start,
            (_, _) => state,
        };
        let led_color = match state {
            StateProblem2::Start => Color::Red,
            StateProblem2::FirstPress => Color::Amber,
            StateProblem2::FirstRelease => Color::Green,
            StateProblem2::SecondPress => Color::Red,
            StateProblem2::SecondRelease => Color::None,
            StateProblem2::ThirdPress => Color::Green,
        };
        set_twoway_del(&mut pina0, &mut pina1, led_color)
    }
}
