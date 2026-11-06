use crate::cube::Cube;

mod free_play;
mod led_hero;
mod reaction;
mod simon_says;

#[derive(Copy, Clone, defmt::Format)]
pub enum Mode {
    FreePlay,
    Simon,
    Reaction,
    LedHero,
    Critter,
}

/// Order of button to which mode.
pub const MODES: [Mode; 5] = [
    Mode::FreePlay,
    Mode::Simon,
    Mode::Reaction,
    Mode::LedHero,
    Mode::Critter,
];

pub fn run_game(mode: Mode, cube: &mut Cube) {
    match mode {
        Mode::FreePlay => free_play::play(cube),
        Mode::Reaction => reaction::play(cube),
        Mode::Simon => simon_says::play(cube),
        Mode::LedHero => led_hero::play(cube),
        Mode::Critter => {}
    }
}
