use crate::cube::Cube;

mod free_play;
mod reaction;

#[derive(Copy, Clone, defmt::Format)]
pub enum Mode {
    FreePlay,
    Simon,
    Reaction,
    Kids,
    Critter,
}

/// Order of button to which mode.
pub const MODES: [Mode; 5] = [
    Mode::FreePlay,
    Mode::Simon,
    Mode::Reaction,
    Mode::Kids,
    Mode::Critter,
];

pub fn run_game(mode: Mode, cube: &mut Cube) {
    match mode {
        Mode::FreePlay => free_play::play(cube),
        Mode::Reaction => reaction::play(cube),
        Mode::Simon | Mode::Kids | Mode::Critter => {}
    }
}
