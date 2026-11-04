use esp_hal::{
    delay::Delay,
    gpio::{DriveMode, Input, Output, interconnect::PeripheralOutput},
    ledc::{
        Ledc, LowSpeed,
        channel::{self, Channel, ChannelIFace},
        timer::Timer,
    },
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

pub struct Button<'a> {
    pub color: Colors,
    pub switch: Input<'static>,
    pub led: Channel<'a, LowSpeed>,
    pub note: Note,
    pub beep_duration_ms: u32,
    pub blink_time_ms: u64,
}
impl<'a> Button<'a> {
    pub fn new(
        color: Colors,
        switch: Input<'static>,
        ledc: &Ledc<'a>,
        timer: &'a Timer<'a, LowSpeed>,
        channel_nr: channel::Number,
        led_pin: impl PeripheralOutput<'a>,
        note: Note,
    ) -> Self {
        let mut led = ledc.channel(channel_nr, led_pin);
        led.configure(channel::config::Config {
            timer,
            duty_pct: 0,
            drive_mode: DriveMode::PushPull,
        })
        .unwrap();
        Self {
            color,
            switch,
            led,
            note,
            beep_duration_ms: 150,
            blink_time_ms: 500,
        }
    }
    /// make the corresponding sound of the button
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
    /// Turn led on, make a corresponding sound of the button and turn it off again.
    pub fn blink_and_beep(&mut self, buzzer: &mut Output<'_>, delay: &Delay) {
        defmt::debug!("its time for color:{:?}", &self.color);
        self.turn_led_on();
        let delay_start = Instant::now();
        self.make_sound(buzzer, delay);
        while delay_start.elapsed() < Duration::from_millis(self.blink_time_ms) {}
        self.turn_led_off();
    }
    /// turn the led of the button on
    pub fn turn_led_on(&mut self) {
        defmt::debug!("its time for color:{:?}", &self.color);
        self.led.set_duty(100).unwrap();
    }
    /// Fade in with time
    pub fn turn_led_on_with(&mut self, fade_ms: u16) {
        self.led.start_duty_fade(0, 100, fade_ms).unwrap();
    }
    /// turn the led of the button on
    pub fn turn_led_off(&mut self) {
        defmt::debug!("its time for color:{:?}", &self.color);
        self.led.set_duty(0).unwrap();
    }
    /// Fade out with time
    pub fn turn_led_off_with(&mut self, fade_ms: u16) {
        self.led.start_duty_fade(100, 0, fade_ms).unwrap();
    }
    /// check if button is pressed
    pub fn is_pressed(&self) -> bool {
        self.switch.is_low()
    }
}
