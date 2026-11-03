#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    time::{Duration, Instant},
};
use panic_rtt_target as _;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[derive(Copy, Clone)]
enum Note {
    C4 = 262,
    E4 = 330,
    G4 = 392,
    C5 = 523,
    A5 = 666,
}
#[derive(Copy, Clone, defmt::Format)]
enum Colors {
    Red,
    Blue,
    Green,
    Yellow,
    White,
}

struct Button {
    color: Colors,
    switch: Input<'static>,
    led: Output<'static>,
    note: Note,
    beep_duration_ms: u32,
    blink_time_ms: u64,
}
impl Button {
    fn new(color: Colors, switch: Input<'static>, led: Output<'static>, note: Note) -> Self {
        Self {
            color,
            switch,
            led,
            note,
            beep_duration_ms: 150,
            blink_time_ms: 500,
        }
    }
    fn make_sound(&self, buzzer: &mut Output<'_>, delay: &Delay) {
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
    fn blink_and_beep(&mut self, buzzer: &mut Output<'_>, delay: &Delay) {
        defmt::debug!("its time for color:{:?}", &self.color);
        self.led.set_high();
        let delay_start = Instant::now();
        self.make_sound(buzzer, delay);
        while delay_start.elapsed() < Duration::from_millis(self.blink_time_ms) {}
        self.led.set_low();
    }
    fn is_pressed(&self) -> bool {
        self.switch.is_low()
    }
}

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);
    let button_config = InputConfig::default().with_pull(Pull::Up);
    let led_config = OutputConfig::default();
    let mut buzzer = Output::new(_peripherals.GPIO4, Level::Low, led_config);
    let delay = Delay::new();
    let mut buttons = [
        Button::new(
            Colors::Green,
            Input::new(_peripherals.GPIO2, button_config),
            Output::new(_peripherals.GPIO3, Level::Low, led_config),
            Note::C4,
        ),
        Button::new(
            Colors::Yellow,
            Input::new(_peripherals.GPIO5, button_config),
            Output::new(_peripherals.GPIO6, Level::Low, led_config),
            Note::E4,
        ),
        Button::new(
            Colors::Red,
            Input::new(_peripherals.GPIO7, button_config),
            Output::new(_peripherals.GPIO21, Level::Low, led_config),
            Note::G4,
        ),
        Button::new(
            Colors::Blue,
            Input::new(_peripherals.GPIO20, button_config),
            Output::new(_peripherals.GPIO8, Level::Low, led_config),
            Note::C5,
        ),
        Button::new(
            Colors::White,
            Input::new(_peripherals.GPIO9, button_config),
            Output::new(_peripherals.GPIO10, Level::Low, led_config),
            Note::A5,
        ),
    ];
    defmt::info!("button setup done, entering loop");
    loop {
        for button in buttons.iter_mut() {
            if button.is_pressed() {
                defmt::info!("button {:?} is pressed!", &button.color);
                button.blink_and_beep(&mut buzzer, &delay);
            }
        }
    }
}
