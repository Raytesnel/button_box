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
}
