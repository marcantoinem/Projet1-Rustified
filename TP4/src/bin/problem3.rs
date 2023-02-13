#![no_std]
#![no_main]

use atmega_hal as hal;
use avr_hal_generic::simple_pwm::IntoPwmPin;
use hal::clock::MHz8;
use hal::prelude::*;
use hal::simple_pwm::{Prescaler, Timer1Pwm};
use panic_abort as _;

// const fn percent_to_duty<const N: usize>(percent: [u8; N]) -> [u8; N] {
//     let mut result = [0; N];
//     let i = 0;
//     while i < N {
//         result[i] = percent[i] * 255 / 100;
//     }
//     result
// }

#[hal::entry]
fn main() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);

    let timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);

    pins.pd3.into_output().set_low();
    let mut pd4 = pins.pd4.into_output().into_pwm(&timer1);
    let mut pd5 = pins.pd5.into_output().into_pwm(&timer1);
    pins.pd6.into_output().set_low();
    pd4.enable();
    pd5.enable();

    loop {
        for x in [0u8, 64, 128, 192, 255, 192, 128, 64] {
            pd4.set_duty(x);
            pd5.set_duty(x);
            hal::delay::Delay::<MHz8>::new().delay_ms(2000u16);
        }
    }
}
