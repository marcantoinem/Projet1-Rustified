pub use atmega_hal as hal;
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

struct SoundParameters {
    number_of_cycle: u32,
    waiting_time: u32,
}

impl SoundParameters {
    const fn new(frequency: u16, time_ms: u32) -> SoundParameters {
        let number_of_cycle = time_ms as u32 * frequency as u32 / 2000;
        let waiting_time = 2_000_000 / frequency as u32;
        SoundParameters {
            number_of_cycle,
            waiting_time,
        }
    }
}

fn make_sound<P1, P2>(
    clock: &mut Delay<MHz8>,
    pin0: &mut Pin<Output, P1>,
    pin1: &mut Pin<Output, P2>,
    parameters: SoundParameters,
) where
    P1: PinOps,
    P2: PinOps,
{
    if parameters.number_of_cycle == 0 {
        clock.delay_us(2 * parameters.waiting_time);
        return;
    }
    for _ in 0..parameters.number_of_cycle {
        pin0.set_high();
        pin1.set_low();
        clock.delay_us(parameters.waiting_time);
        pin0.set_low();
        pin1.set_high();
        clock.delay_us(parameters.waiting_time);
    }
}
pub struct Song<const N1: usize, const N2: usize> {
    pub frequencies: [u16; N1],
    pub notes: [(u8, i8); N2],
}

impl<const N1: usize, const N2: usize> Song<N1, N2> {
    pub fn play<P1, P2>(
        &self,
        clock: &mut Delay<MHz8>,
        pin0: &mut Pin<Output, P1>,
        pin1: &mut Pin<Output, P2>,
        tempo: u32,
    ) where
        P1: PinOps,
        P2: PinOps,
    {
        let time_whole_note: u32 = 240000 as u32 / tempo;
        for (note, divider) in self.notes.as_slice() {
            let note_duration = time_whole_note / *divider as u32;
            let sound_duration = (note_duration as f32 * 0.9) as u32;
            let frequency = self.frequencies.as_slice()[*note as usize];
            make_sound(
                clock,
                pin0,
                pin1,
                SoundParameters::new(frequency, sound_duration),
            );
            let sleep_duration = (note_duration as f32 * 0.1) as u16;
            clock.delay_ms(sleep_duration);
        }
    }
    pub fn play_pwm<const TEMPO: usize>(
        &self,
        clock: &mut Delay<MHz8>,
        pina0: &mut Pin<PwmOutput<Timer1Pwm>, PD4>,
        pina1: &mut Pin<PwmOutput<Timer1Pwm>, PD5>,
        tempo: u32,
    ) {
        let time_whole_note: u32 = 240000 as u32 / tempo;
        for (note, divider) in self.notes.as_slice() {
            let note_duration = if divider >= &0 {
                time_whole_note / *divider as u32
            } else {
                ((time_whole_note as f32 / (-divider) as f32) * 1.5) as u32
            };
            let sound_duration = (note_duration as f32 * 0.9) as u32;
            let frequency = self.frequencies.as_slice()[*note as usize];
            make_sound_pwm(clock, pina0, pina1, frequency, sound_duration);
            let sleep_duration = (note_duration as f32 * 0.1) as u16;
            clock.delay_ms(sleep_duration);
        }
    }
}

// struct Attributes<const A: usize, const B: usize> {
//     a: [u32; A],
//     b: [u32; B],
// }

// // 6 bits pour index a
// // 6 bits pour index b
// // 10 bits pour playing time
// // 10 bits pour waiting time
// struct Note<const A: usize, const B: usize>(u32);

// impl<const A: usize, const B: usize> Note<A, B> {
//     fn a(&self, attributes: &Attributes<{ A }, { B }>) -> u32 {
//         attributes.a[(self.0 >> 26) as usize]
//     }
//     fn b(&self, attributes: &Attributes<{ A }, { B }>) -> u32 {
//         attributes.b[((self.0 as u64) << 6 >> 32) as usize]
//     }
//     fn playing_time_ms(&self) -> u16 {
//         ((self.0 as u64) << 12 >> 32) as u16
//     }
//     fn sleeping_time_ms(&self) -> u16 {
//         ((self.0 as u64) << 18 >> 18) as u16
//     }
// }

// pub struct NewSong<const A: usize, const B: usize, const N: usize> {
//     attributes: Attributes<A, B>,
//     notes: [Note<A, B>; N],
// }
