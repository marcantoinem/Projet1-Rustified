#![no_std]
#![no_main]

use atmega_hal as hal;
use atmega_hal::port::Pin;
use avr_hal_generic::delay::Delay;
use avr_hal_generic::port::{mode::Output, PinOps};
use avr_hal_generic::simple_pwm::IntoPwmPin;
use hal::{clock::MHz8, simple_pwm::Prescaler};
use hal::{prelude::*, simple_pwm::Timer1Pwm};
use panic_abort as _;

fn _problem1() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale1024);
    let mut d5 = pins.pd5.into_output().into_pwm(&timer1);
    let mut d4 = pins.pd4.into_output().into_pwm(&timer1);
    let range = (0..255).chain((0..255).rev());
    loop {
        d4.enable();
        d5.disable();
        for i in range.clone() {
            d4.set_duty(i);
            clock.delay_us(1000u16);
        }
        d4.disable();
        d5.enable();

        for i in range.clone() {
            d5.set_duty(i);
            clock.delay_us(1000u16);
        }
    }
}

fn software_pwm<P>(
    clock: &mut Delay<MHz8>,
    duty: u8,
    frequency: u32,
    duration_ms: u32,
    pin: &mut Pin<Output, P>,
) where
    P: PinOps,
{
    let b = 1_000_000u32 / frequency;
    let a = (255 - duty) as u32 * b / 255;
    let sleep = b - a;
    let number_repetition = duration_ms * 1000 / (a + b);
    for _ in 0..number_repetition {
        pin.set_high();
        clock.delay_us(a);
        pin.set_low();
        clock.delay_us(sleep);
    }
}

#[hal::entry]
fn _problem2() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut clock = hal::delay::Delay::<MHz8>::new();
    let mut b0 = pins.pb0.into_output();
    let mut b1 = pins.pb1.into_output();
    b1.set_low();
    loop {
        for duty in [0, 64, 128, 191, 255] {
            software_pwm(&mut clock, duty, 60, 2000, &mut b0);
        }
        for duty in [0, 64, 128, 191, 255] {
            software_pwm(&mut clock, duty, 400, 2000, &mut b0);
        }
    }
}
