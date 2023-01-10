#![no_std]
#![no_main]

#[avr_device::entry]
fn avrmain() -> ! {
    let dp = avr_device::atmega324pa::Peripherals::take().unwrap();
    dp.PORTA.ddra.write(|w| unsafe { w.bits(0xff) });
    dp.PORTB.ddrb.write(|w| unsafe { w.bits(0xff) });
    dp.PORTC.ddrc.write(|w| unsafe { w.bits(0xff) });
    dp.PORTD.ddrd.write(|w| unsafe { w.bits(0xff) });

    let mut compteur: u32 = 0;
    loop {
        compteur += 1;
        dp.PORTD.portd.write(|w| unsafe { w.bits(compteur as u8) });
        dp.PORTC
            .portc
            .write(|w| unsafe { w.bits((compteur >> 8) as u8) });
        dp.PORTB
            .portb
            .write(|w| unsafe { w.bits((compteur >> 16) as u8) });
        dp.PORTA
            .porta
            .write(|w| unsafe { w.bits((compteur >> 24) as u8) });
    }
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
