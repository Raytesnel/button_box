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
    ledc::{
        LSGlobalClkSource, Ledc, LowSpeed,
        channel::{self},
        timer::{self, TimerIFace},
    },
    main,
    rng::Rng,
    time::Rate,
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

    let mut ledc = Ledc::new(peripherals.LEDC);
    ledc.set_global_slow_clock(LSGlobalClkSource::APBClk); // welke klokbron
    let mut timer = ledc.timer::<LowSpeed>(timer::Number::Timer0);
    timer
        .configure(timer::config::Config {
            duty: timer::config::Duty::Duty8Bit, // 2^8 = 256 stapjes helderheid
            clock_source: timer::LSClockSource::APBClk,
            frequency: Rate::from_khz(24),
        })
        .unwrap();

    let mut cube = Cube {
        buzzer: Output::new(peripherals.GPIO4, Level::Low, OutputConfig::default()),
        delay: Delay::new(),
        rng: Rng::new(),
        buttons: [
            Button::new(
                Colors::Green,
                Input::new(peripherals.GPIO2, button_config),
                &ledc,
                &timer,
                channel::Number::Channel0,
                peripherals.GPIO3,
                Note::C4,
            ),
            Button::new(
                Colors::Yellow,
                Input::new(peripherals.GPIO5, button_config),
                &ledc,
                &timer,
                channel::Number::Channel1,
                peripherals.GPIO6,
                Note::D4,
            ),
            Button::new(
                Colors::Red,
                Input::new(peripherals.GPIO7, button_config),
                &ledc,
                &timer,
                channel::Number::Channel2,
                peripherals.GPIO21,
                Note::E4,
            ),
            Button::new(
                Colors::Blue,
                Input::new(peripherals.GPIO20, button_config),
                &ledc,
                &timer,
                channel::Number::Channel3,
                peripherals.GPIO8,
                Note::G4,
            ),
            Button::new(
                Colors::White,
                Input::new(peripherals.GPIO9, button_config),
                &ledc,
                &timer,
                channel::Number::Channel4,
                peripherals.GPIO10,
                Note::A4,
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
