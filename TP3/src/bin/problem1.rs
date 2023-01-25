#![no_std]
#![no_main]
#![feature(const_for, const_mut_refs)]

use atmega_hal as hal;
use avr_hal_generic::simple_pwm::IntoPwmPin;
use hal::{clock::MHz8, simple_pwm::Prescaler};
use hal::{prelude::*, simple_pwm::Timer1Pwm};
use panic_abort as _;

#[hal::entry]
fn problem1() -> ! {
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
