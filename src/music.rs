use atmega_hal as hal;
use hal::{
    clock::MHz8,
    delay::Delay,
    port::{
        mode::{Output, PwmOutput},
        Pin, PinOps, PD4, PD5,
    },
    prelude::*,
    simple_pwm::Timer1Pwm,
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

fn make_sound_pwm(
    clock: &mut Delay<MHz8>,
    pin0: &mut Pin<PwmOutput<Timer1Pwm>, PD4>,
    pin1: &mut Pin<PwmOutput<Timer1Pwm>, PD5>,
    frequency: u16,
    time_ms: u32,
) {
    let waiting_time = 1_000_000 / frequency as u32;
    let number_of_cycle = time_ms as u32 * frequency as u32 / 2000;
    for _ in 0..number_of_cycle {
        pin0.enable();
        pin1.disable();
        for x in (0..=255).chain((0..=254).rev()).step_by(10) {
            pin0.set_duty(x);
            clock.delay_us(waiting_time / 51);
        }
        pin0.disable();
        pin1.enable();
        for x in (0..=255).chain((0..=254).rev()).step_by(10) {
            pin1.set_duty(x);
            clock.delay_us(waiting_time / 51);
        }
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
    // let mut frequency = frequency;
    if frequency == 0 {
        return;
    } else if frequency <= 160 {
        let natural_frequency = 4100;
        let waiting_time = 1_000_000 / natural_frequency as u32;
        let number_of_cycle = time_ms as u32 * natural_frequency as u32 / 2000;
        let ratio_to_skip = natural_frequency / frequency;
        for n in 0..number_of_cycle {
            if n % ratio_to_skip as u32 == 0 {
                clock.delay_us(2 * waiting_time);
                continue;
            }
            pin0.set_high();
            pin1.set_low();
            clock.delay_us(waiting_time);
            pin0.set_low();
            pin1.set_high();
            clock.delay_us(waiting_time);
        }
        return;
        // let multiplier = 200 / frequency;
        // frequency *= multiplier;
    }
    let waiting_time = 1_000_000 / frequency as u32;
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

#[allow(unused)]
impl<T: FixedSizeArray<u16>, U: FixedSizeArray<(u8, i8)>> Song<T, U> {
    pub fn play<P1, P2>(
        &self,
        clock: &mut Delay<MHz8>,
        pin0: &mut Pin<Output, P1>,
        pin1: &mut Pin<Output, P2>,
    ) where
        P1: PinOps,
        P2: PinOps,
    {
        for (note, divider) in self.notes.as_slice() {
            let note_duration = if divider >= &0 {
                TIME_WHOLE_NOTE / *divider as u32
            } else {
                ((TIME_WHOLE_NOTE as f32 / (-divider) as f32) * 1.5) as u32
            };
            let sound_duration = (note_duration as f32 * 0.9) as u32;
            let frequency = self.frequencies.as_slice()[*note as usize];
            make_sound(clock, pin0, pin1, frequency, sound_duration);
            let sleep_duration = (note_duration as f32 * 0.1) as u16;
            clock.delay_ms(sleep_duration);
        }
    }
    pub fn play_pwm(
        &self,
        clock: &mut Delay<MHz8>,
        pina0: &mut Pin<PwmOutput<Timer1Pwm>, PD4>,
        pina1: &mut Pin<PwmOutput<Timer1Pwm>, PD5>,
    ) {
        for (note, divider) in self.notes.as_slice() {
            let note_duration = if divider >= &0 {
                TIME_WHOLE_NOTE / *divider as u32
            } else {
                ((TIME_WHOLE_NOTE as f32 / (-divider) as f32) * 1.5) as u32
            };
            let sound_duration = (note_duration as f32 * 0.9) as u32;
            let frequency = self.frequencies.as_slice()[*note as usize];
            make_sound_pwm(clock, pina0, pina1, frequency, sound_duration);
            let sleep_duration = (note_duration as f32 * 0.1) as u16;
            clock.delay_ms(sleep_duration);
        }
    }
}

pub struct Music();

#[allow(unused)]
impl Music {
    pub const TETRIS: Song<[u16; 11], [(u8, i8); 99]> = Song {
        frequencies: Self::TETRIS_FREQUENCIES,
        notes: Self::TETRIS_NOTES,
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
    pub const MII_CHANNEL: Song<[u16; 22], [(u8, i8); 286]> = Song {
        frequencies: Self::MII_FREQUENCIES,
        notes: Self::MII_NOTES,
    };
    const MII_FREQUENCIES: [u16; 22] = [
        370, 0, 40, 54, 294, 27, 349, 659, 62, 587, 415, 392, 30, 31, 494, 262, 247, 46, 740, 80,
        20, 23,
    ];
    const MII_NOTES: [(u8, i8); 286] = [
        (0, 8),
        (1, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (0, 8),
        (4, 8),
        (4, 8),
        (4, 8),
        (1, 8),
        (1, 4),
        (1, 8),
        (5, 8),
        (4, 8),
        (0, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (6, 8),
        (7, -4),
        (8, 8),
        (9, 8),
        (1, 8),
        (1, 4),
        (10, 8),
        (1, 8),
        (3, 8),
        (0, 8),
        (1, 8),
        (3, 8),
        (1, 8),
        (10, 8),
        (1, 8),
        (3, 8),
        (11, 8),
        (0, 8),
        (1, 8),
        (12, 8),
        (1, 8),
        (12, 8),
        (12, 8),
        (12, 8),
        (1, 8),
        (1, 4),
        (12, 8),
        (12, 8),
        (12, 8),
        (1, 8),
        (1, 4),
        (13, 8),
        (4, 8),
        (5, 8),
        (1, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (0, 8),
        (4, 8),
        (4, 8),
        (4, 8),
        (1, 8),
        (7, 8),
        (7, 8),
        (7, 8),
        (1, 8),
        (1, 8),
        (0, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (6, 8),
        (7, 2),
        (9, 8),
        (1, 8),
        (1, 4),
        (14, 8),
        (11, 8),
        (4, 8),
        (5, 4),
        (14, 8),
        (11, 8),
        (5, 8),
        (2, 8),
        (0, 8),
        (15, 8),
        (16, 4),
        (6, 8),
        (4, 8),
        (16, 8),
        (12, 8),
        (12, 8),
        (12, 8),
        (1, 4),
        (1, 4),
        (17, 4),
        (3, 8),
        (9, 8),
        (18, 8),
        (19, 8),
        (1, 8),
        (1, 4),
        (1, 2),
        (20, 4),
        (21, 4),
        (20, -4),
        (20, 8),
        (20, 2),
        (1, 4),
        (20, 8),
        (21, 8),
        (20, 8),
        (6, 4),
        (15, 8),
        (20, -4),
        (20, 8),
        (20, 2),
        (1, 2),
        (16, 4),
        (15, 4),
        (5, -4),
        (15, 8),
        (5, 2),
        (1, 4),
        (5, 8),
        (15, 8),
        (5, 8),
        (10, 4),
        (13, 8),
        (5, -4),
        (13, 8),
        (16, 1),
        (12, 4),
        (12, 4),
        (12, 4),
        (1, 8),
        (0, 8),
        (1, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (0, 8),
        (4, 8),
        (4, 8),
        (4, 8),
        (1, 8),
        (1, 4),
        (1, 8),
        (5, 8),
        (4, 8),
        (0, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (6, 8),
        (7, -4),
        (8, 8),
        (9, 8),
        (1, 8),
        (1, 4),
        (10, 8),
        (1, 8),
        (3, 8),
        (0, 8),
        (1, 8),
        (3, 8),
        (1, 8),
        (10, 8),
        (1, 8),
        (3, 8),
        (11, 8),
        (0, 8),
        (1, 8),
        (12, 8),
        (1, 8),
        (12, 8),
        (12, 8),
        (12, 8),
        (1, 8),
        (1, 4),
        (12, 8),
        (12, 8),
        (12, 8),
        (1, 8),
        (1, 4),
        (13, 8),
        (4, 8),
        (5, 8),
        (1, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (0, 8),
        (4, 8),
        (4, 8),
        (4, 8),
        (1, 8),
        (7, 8),
        (7, 8),
        (7, 8),
        (1, 8),
        (1, 8),
        (0, 8),
        (2, 8),
        (3, 8),
        (1, 8),
        (2, 8),
        (1, 8),
        (6, 8),
        (7, 2),
        (9, 8),
        (1, 8),
        (1, 4),
        (14, 8),
        (11, 8),
        (4, 8),
        (5, 4),
        (14, 8),
        (11, 8),
        (5, 8),
        (2, 8),
        (0, 8),
        (15, 8),
        (16, 4),
        (6, 8),
        (4, 8),
        (16, 8),
        (12, 8),
        (12, 8),
        (12, 8),
        (1, 4),
        (1, 4),
        (17, 4),
        (3, 8),
        (9, 8),
        (18, 8),
        (19, 8),
        (1, 8),
        (1, 4),
        (1, 2),
        (20, 4),
        (21, 4),
        (20, -4),
        (20, 8),
        (20, 2),
        (1, 4),
        (20, 8),
        (21, 8),
        (20, 8),
        (6, 4),
        (15, 8),
        (20, -4),
        (20, 8),
        (20, 2),
        (1, 2),
        (16, 4),
        (15, 4),
        (5, -4),
        (15, 8),
        (5, 2),
        (1, 4),
        (5, 8),
        (15, 8),
        (5, 8),
        (10, 4),
        (13, 8),
        (5, -4),
        (13, 8),
        (16, 1),
        (12, 4),
        (12, 4),
        (12, 4),
        (1, 8),
    ];
    pub const SUPER_MARIO_BROS: Song<[u16; 17], [(u8, i8); 321]> = Song {
        frequencies: Self::MARIO_FREQUENCIES,
        notes: Self::MARIO_NOTES,
    };
    const MARIO_FREQUENCIES: [u16; 17] = [
        659, 0, 523, 784, 392, 30, 40, 494, 46, 80, 698, 587, 740, 62, 415, 262, 294,
    ];
    const MARIO_NOTES: [(u8, i8); 321] = [
        (0, 8),
        (0, 8),
        (1, 8),
        (0, 8),
        (1, 8),
        (2, 8),
        (0, 8),
        (3, 4),
        (1, 4),
        (4, 8),
        (1, 4),
        (2, -4),
        (4, 8),
        (1, 4),
        (5, -4),
        (6, 4),
        (7, 4),
        (8, 8),
        (6, 4),
        (4, -8),
        (0, -8),
        (3, -8),
        (9, 4),
        (10, 8),
        (3, 8),
        (1, 8),
        (0, 4),
        (2, 8),
        (11, 8),
        (7, -4),
        (2, -4),
        (4, 8),
        (1, 4),
        (5, -4),
        (6, 4),
        (7, 4),
        (8, 8),
        (6, 4),
        (4, -8),
        (0, -8),
        (3, -8),
        (9, 4),
        (10, 8),
        (3, 8),
        (1, 8),
        (0, 4),
        (2, 8),
        (11, 8),
        (7, -4),
        (1, 4),
        (3, 8),
        (12, 8),
        (10, 8),
        (13, 4),
        (0, 8),
        (1, 8),
        (14, 8),
        (6, 8),
        (15, 8),
        (1, 8),
        (6, 8),
        (2, 8),
        (11, 8),
        (1, 4),
        (13, 4),
        (1, 8),
        (11, -4),
        (2, 2),
        (1, 2),
        (1, 4),
        (3, 8),
        (12, 8),
        (10, 8),
        (13, 4),
        (0, 8),
        (1, 8),
        (14, 8),
        (6, 8),
        (15, 8),
        (1, 8),
        (6, 8),
        (2, 8),
        (11, 8),
        (1, 4),
        (13, 4),
        (1, 8),
        (11, -4),
        (2, 2),
        (1, 2),
        (2, 8),
        (2, 4),
        (2, 8),
        (1, 8),
        (2, 8),
        (11, 4),
        (0, 8),
        (2, 4),
        (6, 8),
        (4, 2),
        (2, 8),
        (2, 4),
        (2, 8),
        (1, 8),
        (2, 8),
        (11, 8),
        (0, 8),
        (1, 1),
        (2, 8),
        (2, 4),
        (2, 8),
        (1, 8),
        (2, 8),
        (11, 4),
        (0, 8),
        (2, 4),
        (6, 8),
        (4, 2),
        (0, 8),
        (0, 8),
        (1, 8),
        (0, 8),
        (1, 8),
        (2, 8),
        (0, 4),
        (3, 4),
        (1, 4),
        (4, 4),
        (1, 4),
        (2, -4),
        (4, 8),
        (1, 4),
        (5, -4),
        (6, 4),
        (7, 4),
        (8, 8),
        (6, 4),
        (4, -8),
        (0, -8),
        (3, -8),
        (9, 4),
        (10, 8),
        (3, 8),
        (1, 8),
        (0, 4),
        (2, 8),
        (11, 8),
        (7, -4),
        (2, -4),
        (4, 8),
        (1, 4),
        (5, -4),
        (6, 4),
        (7, 4),
        (8, 8),
        (6, 4),
        (4, -8),
        (0, -8),
        (3, -8),
        (9, 4),
        (10, 8),
        (3, 8),
        (1, 8),
        (0, 4),
        (2, 8),
        (11, 8),
        (7, -4),
        (0, 8),
        (2, 4),
        (4, 8),
        (1, 4),
        (14, 4),
        (6, 8),
        (10, 4),
        (10, 8),
        (6, 2),
        (11, -8),
        (9, -8),
        (9, -8),
        (9, -8),
        (3, -8),
        (10, -8),
        (0, 8),
        (2, 4),
        (6, 8),
        (4, 2),
        (0, 8),
        (2, 4),
        (4, 8),
        (1, 4),
        (14, 4),
        (6, 8),
        (10, 4),
        (10, 8),
        (6, 2),
        (7, 8),
        (10, 4),
        (10, 8),
        (10, -8),
        (0, -8),
        (11, -8),
        (2, 8),
        (5, 4),
        (5, 8),
        (15, 2),
        (0, 8),
        (2, 4),
        (4, 8),
        (1, 4),
        (14, 4),
        (6, 8),
        (10, 4),
        (10, 8),
        (6, 2),
        (11, -8),
        (9, -8),
        (9, -8),
        (9, -8),
        (3, -8),
        (10, -8),
        (0, 8),
        (2, 4),
        (6, 8),
        (4, 2),
        (0, 8),
        (2, 4),
        (4, 8),
        (1, 4),
        (14, 4),
        (6, 8),
        (10, 4),
        (10, 8),
        (6, 2),
        (7, 8),
        (10, 4),
        (10, 8),
        (10, -8),
        (0, -8),
        (11, -8),
        (2, 8),
        (5, 4),
        (5, 8),
        (15, 2),
        (2, 8),
        (2, 4),
        (2, 8),
        (1, 8),
        (2, 8),
        (11, 8),
        (0, 8),
        (1, 1),
        (2, 8),
        (2, 4),
        (2, 8),
        (1, 8),
        (2, 8),
        (11, 4),
        (0, 8),
        (2, 4),
        (6, 8),
        (4, 2),
        (0, 8),
        (0, 8),
        (1, 8),
        (0, 8),
        (1, 8),
        (2, 8),
        (0, 4),
        (3, 4),
        (1, 4),
        (4, 4),
        (1, 4),
        (0, 8),
        (2, 4),
        (4, 8),
        (1, 4),
        (14, 4),
        (6, 8),
        (10, 4),
        (10, 8),
        (6, 2),
        (11, -8),
        (9, -8),
        (9, -8),
        (9, -8),
        (3, -8),
        (10, -8),
        (0, 8),
        (2, 4),
        (6, 8),
        (4, 2),
        (0, 8),
        (2, 4),
        (4, 8),
        (1, 4),
        (14, 4),
        (6, 8),
        (10, 4),
        (10, 8),
        (6, 2),
        (7, 8),
        (10, 4),
        (10, 8),
        (10, -8),
        (0, -8),
        (11, -8),
        (2, 8),
        (5, 4),
        (5, 8),
        (15, 2),
        (2, -4),
        (4, -4),
        (5, 4),
        (6, -8),
        (7, -8),
        (6, -8),
        (14, -8),
        (8, -8),
        (14, -8),
        (4, 8),
        (16, 8),
        (5, -2),
    ];
    pub const IMPERIAL_MARCH: Song<[u16; 14], [(u8, i8); 86]> = Song {
        frequencies: Self::IMPERIAL_FREQUENCIES,
        notes: Self::IMPERIAL_NOTES,
    };
    const IMPERIAL_FREQUENCIES: [u16; 14] = [
        40, 349, 0, 523, 659, 698, 80, 831, 784, 62, 587, 54, 494, 415,
    ];
    const IMPERIAL_NOTES: [(u8, i8); 86] = [
        (0, -4),
        (0, -4),
        (0, 16),
        (0, 16),
        (0, 16),
        (0, 16),
        (1, 8),
        (2, 8),
        (0, -4),
        (0, -4),
        (0, 16),
        (0, 16),
        (0, 16),
        (0, 16),
        (1, 8),
        (2, 8),
        (0, 4),
        (0, 4),
        (0, 4),
        (1, -8),
        (3, 16),
        (0, 4),
        (1, -8),
        (3, 16),
        (0, 2),
        (4, 4),
        (4, 4),
        (4, 4),
        (5, -8),
        (3, 16),
        (0, 4),
        (1, -8),
        (3, 16),
        (0, 2),
        (6, 4),
        (0, -8),
        (0, 16),
        (6, 4),
        (7, -8),
        (8, 16),
        (9, 16),
        (10, 16),
        (9, 8),
        (2, 8),
        (0, 8),
        (9, 4),
        (10, -8),
        (11, 16),
        (3, 16),
        (12, 16),
        (3, 16),
        (2, 8),
        (1, 8),
        (13, 4),
        (1, -8),
        (0, -16),
        (3, 4),
        (0, -8),
        (3, 16),
        (4, 2),
        (6, 4),
        (0, -8),
        (0, 16),
        (6, 4),
        (7, -8),
        (8, 16),
        (9, 16),
        (10, 16),
        (9, 8),
        (2, 8),
        (0, 8),
        (9, 4),
        (10, -8),
        (11, 16),
        (3, 16),
        (12, 16),
        (3, 16),
        (2, 8),
        (1, 8),
        (13, 4),
        (1, -8),
        (0, -16),
        (0, 4),
        (1, -8),
        (3, 16),
        (0, 2),
    ];
    pub fn metro_ringtone<P1, P2>(
        clock: &mut Delay<MHz8>,
        pin0: &mut Pin<Output, P1>,
        pin1: &mut Pin<Output, P2>,
    ) where
        P1: PinOps,
        P2: PinOps,
    {
        const METRO_RINGTONE: [(u16, u32); 3] = [(360, 1000), (480, 1000), (720, 1000)];
        for (frequency, time_ms) in METRO_RINGTONE {
            make_sound(clock, pin0, pin1, frequency, time_ms);
        }
    }
}
