#![no_std]
#![no_main]
#![feature(const_for, const_mut_refs, abi_avr_interrupt)]

use core::mem::MaybeUninit;

use atmega_hal as hal;
use hal::port::{
    mode::{Floating, Input},
    Pin, PD2,
};
use inf1900_robot_hal::device::{read_input, set_twoway_del, Color};
use panic_abort as _;

enum StateProblem2 {
    Start,
    FirstPress,
    FirstRelease,
    SecondPress,
    SecondRelease,
    ThirdPress,
}

static mut STATE: StateProblem2 = StateProblem2::Start;
static mut PD2: MaybeUninit<Pin<Input<Floating>, PD2>> = MaybeUninit::uninit();

#[avr_device::interrupt(atmega324pa)]
fn INT0() {
    // This is memory safe only because interrupts stop main when executed.
    let state = unsafe { &mut STATE };
    let pind2 = unsafe { &mut *PD2.as_mut_ptr() };
    match (&state, read_input(&pind2)) {
        (StateProblem2::Start, true) => *state = StateProblem2::FirstPress,
        (StateProblem2::FirstPress, false) => *state = StateProblem2::FirstRelease,
        (StateProblem2::FirstRelease, true) => *state = StateProblem2::SecondPress,
        (StateProblem2::SecondPress, false) => *state = StateProblem2::SecondRelease,
        (StateProblem2::SecondRelease, true) => *state = StateProblem2::ThirdPress,
        (StateProblem2::ThirdPress, false) => *state = StateProblem2::Start,
        (_, _) => (),
    };
}

#[hal::entry]
fn problem1() -> ! {
    let dp = hal::Peripherals::take().unwrap();
    let pins = hal::pins!(dp);
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();
    // Enable INT0
    dp.EXINT.eimsk.write(|x| x.int0().set_bit());
    dp.EXINT.eicra.write(|x| x.isc0().bits(1));
    unsafe {
        PD2 = MaybeUninit::new(pins.pd2.into_floating_input());
        // Prevent the compiler from enabling interrupt before initializing the pin.
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        avr_device::interrupt::enable();
    }

    loop {
        let pd2_color = match unsafe { &STATE } {
            StateProblem2::Start => Color::Red,
            StateProblem2::FirstPress => Color::Amber,
            StateProblem2::FirstRelease => Color::Green,
            StateProblem2::SecondPress => Color::Red,
            StateProblem2::SecondRelease => Color::None,
            StateProblem2::ThirdPress => Color::Green,
        };
        set_twoway_del(&mut pina0, &mut pina1, pd2_color);
        avr_device::asm::sleep();
    }
}
