#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::mem::MaybeUninit;

use atmega_hal as hal;
use hal::{
    clock::MHz8,
    delay::Delay,
    pac::{EXINT, TC1},
    port::{
        mode::{Floating, Input},
        Pin, PD2,
    },
    prelude::_embedded_hal_blocking_delay_DelayMs,
};
use inf1900_robot_hal::device::{read_input, set_twoway_del, Color};
use panic_abort as _;
use vcell::VolatileCell;

#[derive(Clone, Copy)]
enum State {
    Waiting,
    Success,
    Failure,
}

static mut PD2: MaybeUninit<Pin<Input<Floating>, PD2>> = MaybeUninit::uninit();
static mut STATE: VolatileCell<State> = VolatileCell::new(State::Waiting);
static mut IS_BUTTON_PUSHED: VolatileCell<bool> = VolatileCell::new(false);

const fn in_ms(n: u16) -> u16 {
    (8_000_000 / 1024 * n as u32 / 1000) as u16
}

#[avr_device::interrupt(atmega324pa)]
fn INT0() {
    unsafe {
        IS_BUTTON_PUSHED.set(IS_BUTTON_PUSHED.get() || read_input(&*PD2.as_ptr()));
    }
}

#[avr_device::interrupt(atmega324pa)]
fn TIMER1_COMPA() {
    match unsafe { (STATE.get(), IS_BUTTON_PUSHED.get()) } {
        (State::Waiting, true) => unsafe { STATE.set(State::Success) },
        (State::Waiting, false) => unsafe { STATE.set(State::Failure) },
        _ => (),
    }
}

fn enable_int0(exint: &EXINT) {
    exint.eimsk.write(|x| x.int0().set_bit());
    exint.eicra.write(|x| x.isc0().bits(1));
}

fn enable_timer(tc1: &TC1) {
    tc1.tccr1a.write(|w| w.wgm1().bits(0b00));
    tc1.tccr1b
        .write(|w| w.cs1().prescale_256().wgm1().bits(0b01));
    tc1.ocr1a.write(|w| w.bits(in_ms(1000)));

    // Enable the timer interrupt
    tc1.timsk1.write(|w| w.ocie1a().set_bit());
}

#[hal::entry]
fn problem2() -> ! {
    let dp = hal::Peripherals::take().unwrap();

    let pins = hal::pins!(dp);
    let mut delay = Delay::<MHz8>::new();
    let mut pina0 = pins.pa0.into_output();
    let mut pina1 = pins.pa1.into_output();

    delay.delay_ms(10000u16);
    set_twoway_del(&mut pina0, &mut pina1, Color::Red);
    delay.delay_ms(100u16);

    unsafe {
        PD2 = MaybeUninit::new(pins.pd2.into_floating_input());
        enable_int0(&dp.EXINT);
        enable_timer(&dp.TC1);
        // Prevent the compiler from enabling interrupt before initializing the pin.
        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
        avr_device::interrupt::enable();
    }

    loop {
        delay.delay_ms(100u16);
        let color = match unsafe { STATE.get() } {
            State::Waiting => Color::None,
            State::Success => Color::Green,
            State::Failure => Color::Red,
        };
        set_twoway_del(&mut pina0, &mut pina1, color);
        avr_device::asm::sleep();
    }
}
