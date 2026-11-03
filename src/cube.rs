use crate::button::Button;
use esp_hal::{delay::Delay, gpio::Output, rng::Rng};

pub struct Cube {
    pub buttons: [Button; 5],
    pub buzzer: Output<'static>,
    pub delay: Delay,
    pub rng: Rng,
}

impl Cube {
    /// Do an action on all leds
    pub fn all_leds(&mut self, on: bool) {
        for b in self.buttons.iter_mut() {
            if on {
                b.led.set_high()
            } else {
                b.led.set_low()
            }
        }
    }
    /// Wacht tot alle knoppen los zijn (met debounce)
    pub fn wait_release(&self) {
        while self.buttons.iter().any(|b| b.is_pressed()) {}
        self.delay.delay_millis(50);
    }
}
