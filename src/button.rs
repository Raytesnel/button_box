use esp_hal::{
    delay::Delay,
    gpio::{Input, Output},
    time::{Duration, Instant},
};
use panic_rtt_target as _;

#[derive(Copy, Clone)]
pub enum Note {
    C4 = 262,
    E4 = 330,
    G4 = 392,
    C5 = 523,
    A5 = 666,
}

#[derive(Copy, Clone, defmt::Format)]
pub enum Colors {
    Red,
    Blue,
    Green,
    Yellow,
    White,
}

pub struct Button {
    pub color: Colors,
    pub switch: Input<'static>,
    pub led: Output<'static>,
    pub note: Note,
    pub beep_duration_ms: u32,
    pub blink_time_ms: u64,
}
impl Button {
    pub fn new(color: Colors, switch: Input<'static>, led: Output<'static>, note: Note) -> Self {
        Self {
            color,
            switch,
            led,
            note,
            beep_duration_ms: 150,
            blink_time_ms: 500,
        }
    }
    pub fn make_sound(&self, buzzer: &mut Output<'_>, delay: &Delay) {
        let freq_hz = self.note as u32;
        let half_period_us = 1_000_000 / (freq_hz * 2);
        let cycles = freq_hz * self.beep_duration_ms / 1000;
        for _ in 0..cycles {
            buzzer.set_high();
            delay.delay_micros(half_period_us);
            buzzer.set_low();
            delay.delay_micros(half_period_us);
        }
    }
    pub fn blink_and_beep(&mut self, buzzer: &mut Output<'_>, delay: &Delay) {
        defmt::debug!("its time for color:{:?}", &self.color);
        self.led.set_high();
        let delay_start = Instant::now();
        self.make_sound(buzzer, delay);
        while delay_start.elapsed() < Duration::from_millis(self.blink_time_ms) {}
        self.led.set_low();
    }
    pub fn is_pressed(&self) -> bool {
        self.switch.is_low()
    }
}
