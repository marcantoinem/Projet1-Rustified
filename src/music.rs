use atmega_hal as hal;
use hal::{
    clock::MHz8,
    delay::Delay,
    port::{mode::Output, Pin, PinOps, PA0, PA1},
    prelude::*,
};

const TEMPO: u32 = 104;
const TIME_WHOLE_NOTE: u32 = (240000 as u32 / TEMPO) as u32;

pub unsafe trait FixedSizeArray<T> {
    fn as_slice(&self) -> &[T];
    fn as_mut_slice(&mut self) -> &mut [T];
}

unsafe impl<T, A: core::marker::Unsize<[T]>> FixedSizeArray<T> for A {
    #[inline]
    fn as_slice(&self) -> &[T] {
        self
    }
    #[inline]
    fn as_mut_slice(&mut self) -> &mut [T] {
        self
    }
}

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

pub struct Song<T: FixedSizeArray<u16>, U: FixedSizeArray<(u8, i8)>> {
    frequencies: T,
    notes: U,
}

impl<T: FixedSizeArray<u16>, U: FixedSizeArray<(u8, i8)>> Song<T, U> {
    pub fn play(
        &self,
        clock: &mut Delay<MHz8>,
        pina0: &mut Pin<Output, PA0>,
        pina1: &mut Pin<Output, PA1>,
    ) {
        for (note, divider) in self.notes.as_slice() {
            let note_duration = if divider >= &0 {
                TIME_WHOLE_NOTE / *divider as u32
            } else {
                ((TIME_WHOLE_NOTE as f32 / (-divider) as f32) * 1.5) as u32
            };
            let sound_duration = (note_duration as f32 * 0.9) as u32;
            let frequency = self.frequencies.as_slice()[*note as usize];
            make_sound(clock, pina0, pina1, frequency, sound_duration);
            let sleep_duration = (note_duration as f32 * 0.1) as u16;
            clock.delay_ms(sleep_duration);
        }
    }
}

pub const TETRIS: Song<[u16; 11], [(u8, i8); 99]> = Song {
    frequencies: TETRIS_FREQUENCIES,
    notes: TETRIS_NOTES,
};

const TETRIS_FREQUENCIES: [u16; 11] = [659, 494, 523, 587, 40, 698, 80, 784, 0, 415, 831];
const TETRIS_NOTES: [(u8, i8); 99] = [
    (0, 4),
    (1, 8),
    (2, 8),
    (3, 4),
    (2, 8),
    (1, 8),
    (4, 4),
    (4, 8),
    (2, 8),
    (0, 4),
    (3, 8),
    (2, 8),
    (1, -4),
    (2, 8),
    (3, 4),
    (0, 4),
    (2, 4),
    (4, 4),
    (4, 8),
    (4, 4),
    (1, 8),
    (2, 8),
    (3, -4),
    (5, 8),
    (6, 4),
    (7, 8),
    (5, 8),
    (0, -4),
    (2, 8),
    (0, 4),
    (3, 8),
    (2, 8),
    (1, 4),
    (1, 8),
    (2, 8),
    (3, 4),
    (0, 4),
    (2, 4),
    (4, 4),
    (4, 4),
    (8, 4),
    (0, 4),
    (1, 8),
    (2, 8),
    (3, 4),
    (2, 8),
    (1, 8),
    (4, 4),
    (4, 8),
    (2, 8),
    (0, 4),
    (3, 8),
    (2, 8),
    (1, -4),
    (2, 8),
    (3, 4),
    (0, 4),
    (2, 4),
    (4, 4),
    (4, 8),
    (4, 4),
    (1, 8),
    (2, 8),
    (3, -4),
    (5, 8),
    (6, 4),
    (7, 8),
    (5, 8),
    (0, -4),
    (2, 8),
    (0, 4),
    (3, 8),
    (2, 8),
    (1, 4),
    (1, 8),
    (2, 8),
    (3, 4),
    (0, 4),
    (2, 4),
    (4, 4),
    (4, 4),
    (8, 4),
    (0, 2),
    (2, 2),
    (3, 2),
    (1, 2),
    (2, 2),
    (4, 2),
    (9, 2),
    (1, 4),
    (8, 8),
    (0, 2),
    (2, 2),
    (3, 2),
    (1, 2),
    (2, 4),
    (0, 4),
    (6, 2),
    (10, 2),
];
