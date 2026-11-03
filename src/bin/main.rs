#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]
use button_box::{
    button::{Button, Colors, Note},
    cube::Cube,
    games::run_game,
    menu::select_mode,
};
use esp_hal::{
    clock::CpuClock,
    delay::Delay,
    gpio::{Input, InputConfig, Level, Output, OutputConfig, Pull},
    main,
    rng::Rng,
    time::{Duration, Instant},
};
use panic_rtt_target as _;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    rtt_target::rtt_init_defmt!();
    let peripherals = esp_hal::init(esp_hal::Config::default().with_cpu_clock(CpuClock::max()));
    let button_config = InputConfig::default().with_pull(Pull::Up);
    let led_config = OutputConfig::default();

    let mut cube = Cube {
        buzzer: Output::new(peripherals.GPIO4, Level::Low, led_config),
        delay: Delay::new(),
        rng: Rng::new(),
        buttons: [
            Button::new(
                Colors::Green,
                Input::new(peripherals.GPIO2, button_config),
                Output::new(peripherals.GPIO3, Level::Low, led_config),
                Note::C4,
            ),
            Button::new(
                Colors::Yellow,
                Input::new(peripherals.GPIO5, button_config),
                Output::new(peripherals.GPIO6, Level::Low, led_config),
                Note::E4,
            ),
            Button::new(
                Colors::Red,
                Input::new(peripherals.GPIO7, button_config),
                Output::new(peripherals.GPIO21, Level::Low, led_config),
                Note::G4,
            ),
            Button::new(
                Colors::Blue,
                Input::new(peripherals.GPIO20, button_config),
                Output::new(peripherals.GPIO8, Level::Low, led_config),
                Note::C5,
            ),
            Button::new(
                Colors::White,
                Input::new(peripherals.GPIO9, button_config),
                Output::new(peripherals.GPIO10, Level::Low, led_config),
                Note::A5,
            ),
        ],
    };
    defmt::info!("Setup done, entering loop");
    loop {
        let mode = select_mode(&mut cube);
        defmt::info!("mode: {}", mode);
        run_game(mode, &mut cube);
    }
}
