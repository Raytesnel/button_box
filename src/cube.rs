use crate::button::Button;
use esp_hal::{delay::Delay, gpio::Output, rng::Rng};

pub struct Cube<'a> {
    pub buttons: [Button<'a>; 5],
    pub buzzer: Output<'static>,
    pub delay: Delay,
    pub rng: Rng,
}

impl<'a> Cube<'a> {
    /// Do an action on all leds
    pub fn all_leds(&mut self, on: bool) {
        for b in self.buttons.iter_mut() {
            if on {
                b.turn_led_on();
            } else {
                b.turn_led_off();
            }
        }
    }
    /// Wacht tot alle knoppen los zijn (met debounce)
    pub fn wait_release(&self) {
        while self.buttons.iter().any(|b| b.is_pressed()) {}
        self.delay.delay_millis(50);
    }

    /// choose a random button
    pub fn random_button(&self) -> usize {
        (self.rng.random() % self.buttons.len() as u32) as usize
    }

    pub fn play_tone(&mut self, freq_hz: u32, duration_ms: u32) {
        let half_period_us = 1_000_000 / (freq_hz * 2);
        let cycles = freq_hz * duration_ms / 1000;
        for _ in 0..cycles {
            self.buzzer.set_high();
            self.delay.delay_micros(half_period_us);
            self.buzzer.set_low();
            self.delay.delay_micros(half_period_us);
        }
    }
    /// game over sound
    pub fn make_game_over_sound(&mut self) {
        // (frequentie in Hz, duur in ms): elke noot een halve toon lager
        let notes = [(233, 250), (220, 250), (208, 250)];

        for (freq, ms) in notes {
            self.play_tone(freq, ms);
            self.delay.delay_millis(80); // de stilte maakt het "mwap"
        }
        // the long "mwaaaap": slides down while wobbling
        for step in 0..60u32 {
            let slide = 196 - step; // 196 -> 137 Hz
            let wobble = if (step / 4) % 2 == 0 { 8 } else { 0 };
            self.play_tone(slide + wobble, 25); // 60 x 25 ms = 1.5 s
        }
    }
}
