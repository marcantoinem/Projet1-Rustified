#![no_std]
#![no_main]
#![feature(const_for, const_mut_refs)]

use atmega_hal as hal;
use atmega_hal::port::Pin;
use avr_hal_generic::delay::Delay;
use avr_hal_generic::port::{mode::Output, PinOps};
use hal::clock::MHz8;
use hal::prelude::*;
use panic_abort as _;

#[derive(Copy, Clone)]
struct PwmTiming {
    time_high: u32,
    time_low: u32,
    number_repetition: u32,
}

impl PwmTiming {
    const fn new(duty: u8, frequency: u32, duration_ms: u32) -> PwmTiming {
        let b = 1_000_000u32 / frequency;
        let a = duty as u32 * b / 255;
        let number_repetition = duration_ms * 1000 / (a + b);
        PwmTiming {
            time_high: a,
            time_low: b - a,
            number_repetition,
        }
    }
    const fn default() -> PwmTiming {
        PwmTiming {
            time_high: 0,
            time_low: 0,
            number_repetition: 0,
        }
    }
    const fn from_array<const N: usize>(
        duties: [u8; N],
        frequency: u32,
        duration_ms: u32,
    ) -> [PwmTiming; N] {
        let mut new_array: [PwmTiming; N] = [PwmTiming::default(); N];
        let mut i = 0;
        while i < duties.len() {
            new_array[i] = PwmTiming::new(duties[i], frequency, duration_ms);
            i += 1;
        }
        new_array
    }
}

fn software_pwm<P>(clock: &mut Delay<MHz8>, parameters: PwmTiming, pin: &mut Pin<Output, P>)
where
    P: PinOps,
{
    for _ in 0..parameters.number_repetition {
        pin.set_high();
        clock.delay_us(parameters.time_high);
        pin.set_low();
        clock.delay_us(parameters.time_low);
    }
}

const PARAMETERS_60HZ: [PwmTiming; 5] = PwmTiming::from_array([0, 64, 128, 191, 255], 60, 2000);
const PARAMETERS_400HZ: [PwmTiming; 5] = PwmTiming::from_array([0, 64, 128, 191, 255], 400, 2000);

#[hal::entry]
fn problem2() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let mut b0 = pins.pb0.into_output();
    let mut b1 = pins.pb1.into_output();
    b1.set_low();
    loop {
        for parameters in PARAMETERS_60HZ {
            software_pwm(&mut clock, parameters, &mut b0);
        }
        for parameters in PARAMETERS_400HZ {
            software_pwm(&mut clock, parameters, &mut b0);
        }
    }
}
