#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::arch::asm;
use panic_halt as _;

fn busy_loop(mut c: u16) {
    // Wait for 3+c instruction
    #[allow(unused_assignments)]
    unsafe {
        asm!(
            "1:",
            "sbiw {c}, 1",
            "brne 1b",
            c = inout(reg_iw) c,
        );
    }
}

// fn sleep(n: u32) {
//     let mut i = 0;
//     while i < n {
//         busy_loop(0xffff);
//         i += 1;
//     }
// }

#[avr_device::entry]
fn avrmain() -> ! {
    let dp = avr_device::atmega324pa::Peripherals::take().unwrap();
    dp.PORTA.ddra.write(|w| unsafe { w.bits(0x03) });
    dp.PORTD.ddrd.write(|w| unsafe { w.bits(0x00) });
    loop {
        let read = dp.PORTD.pind.read().pd2().bit_is_set();
        busy_loop(0x0fff);
        let debounced_read = read & dp.PORTD.pind.read().pd2().bit_is_set();
        let bits = ((debounced_read as u8) << 1) + !debounced_read as u8;
        dp.PORTA.porta.write(|w| unsafe { w.bits(bits) });
    }
}

/*
 * Table vérité
 * AB -> CD
 * 00 -> 10
 * 01 -> 01
 * 10 -> XX
 * 11 -> XX
 *
 * C
 * A\B | 0 | 1 |
 *  0  | 1 | 0 |
 *  1  | X | X |
 * C=B'
 *
 * D
 * A\B | 0 | 1 |
 *  0  | 0 | 1 |
 *  1  | X | X |
 * D=B
 *
 * CD = B >> 2 + B'
 */
