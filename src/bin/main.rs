#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use defmt::info;
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Io, Level, Output, OutputConfig, Pull},
    ledc::{
        Ledc, LowSpeed,
        channel::{self, ChannelIFace},
        timer::{self, TimerIFace},
    },
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
#[derive(Copy, Clone)]
struct BeepConfig {
    note: Note,
    duration_ms: u32,
    repeats: u8,
}

enum Colors {
    RED,
    BLUE,
    GREEN,
    YELLOW,
    WHITE,
}

struct Button {
    color: Colors,
    switch: Input<'static>,
    led: Output<'static>,
    beep: BeepConfig,
}
impl Button {
    fn new(color: Colors, switch: Input<'static>, led: Output<'static>, beep: BeepConfig) -> Self {
        Self {
            color,
            switch,
            led,
            beep,
        }
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
    let mut buttons = [
        Button::new(
            Colors::GREEN,
            Input::new(_peripherals.GPIO2, button_config),
            Output::new(_peripherals.GPIO3, Level::Low, led_config),
            BeepConfig {
                note: Note::C4,
                duration_ms: 150,
                repeats: 1,
            },
        ),
        Button::new(
            Colors::YELLOW,
            Input::new(_peripherals.GPIO5, button_config),
            Output::new(_peripherals.GPIO6, Level::Low, led_config),
            BeepConfig {
                note: Note::E4,
                duration_ms: 150,
                repeats: 1,
            },
        ),
        Button::new(
            Colors::RED,
            Input::new(_peripherals.GPIO7, button_config),
            Output::new(_peripherals.GPIO21, Level::Low, led_config),
            BeepConfig {
                note: Note::G4,
                duration_ms: 150,
                repeats: 1,
            },
        ),
        Button::new(
            Colors::BLUE,
            Input::new(_peripherals.GPIO20, button_config),
            Output::new(_peripherals.GPIO8, Level::Low, led_config),
            BeepConfig {
                note: Note::C5,
                duration_ms: 150,
                repeats: 1,
            },
        ),
        Button::new(
            Colors::WHITE,
            Input::new(_peripherals.GPIO9, button_config),
            Output::new(_peripherals.GPIO10, Level::Low, led_config),
            BeepConfig {
                note: Note::A5,
                duration_ms: 150,
                repeats: 1,
            },
        ),
    ];

    loop {
        info!("Hello world!");
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }
}
