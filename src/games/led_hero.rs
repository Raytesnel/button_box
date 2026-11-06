use esp_hal::time::{Duration, Instant};

use crate::{button::Note, cube::Cube};

// ---- note lengths (in eighth notes) ----
pub const EIGHTH: u8 = 1;
pub const QUARTER: u8 = 2;
pub const DOT_QUARTER: u8 = 3; // quarter + eighth
pub const HALF: u8 = 4;
pub const WHOLE: u8 = 8;

pub type Step = (Note, u8);

pub struct Song {
    _name: &'static str,
    eighth_ms: u32,
    buttons: [Note; 5],
    steps: &'static [Step],
}

/// Mary Had a Little Lamb: notes C D E G
#[rustfmt::skip]
pub const MARY: Song = Song {
    _name: "Mary Had a Little Lamb",
    eighth_ms: 250,
    buttons: [Note::C4,Note::D4,Note::E4, Note::G4,Note::F4],
    steps: &[
        (Note::E4, QUARTER), (Note::D4, QUARTER), (Note::C4, QUARTER), (Note::D4, QUARTER),
        (Note::E4, QUARTER), (Note::E4, QUARTER), (Note::E4, HALF),
        (Note::D4, QUARTER), (Note::D4, QUARTER), (Note::D4, HALF),
        (Note::E4, QUARTER), (Note::G4, QUARTER), (Note::G4, HALF),
        (Note::E4, QUARTER), (Note::D4, QUARTER), (Note::C4, QUARTER), (Note::D4, QUARTER),
        (Note::E4, QUARTER), (Note::E4, QUARTER), (Note::E4, QUARTER), (Note::E4, QUARTER),
        (Note::D4, QUARTER), (Note::D4, QUARTER), (Note::E4, QUARTER), (Note::D4, QUARTER),
        (Note::C4, WHOLE),
    ],
};

#[rustfmt::skip]
pub const HOT_CROSS_BUNS: Song = Song {
    _name: "Hot Cross Buns",
    eighth_ms: 250,
    buttons: [Note::C4,Note::D4,Note::E4, Note::G4,Note::F4],
    steps: &[
        (Note::E4, QUARTER), (Note::D4, QUARTER), (Note::C4, HALF),
        (Note::E4, QUARTER), (Note::D4, QUARTER), (Note::C4, HALF),
        (Note::C4, EIGHTH), (Note::C4, EIGHTH), (Note::C4, EIGHTH), (Note::C4, EIGHTH),
        (Note::D4, EIGHTH), (Note::D4, EIGHTH), (Note::D4, EIGHTH), (Note::D4, EIGHTH),
        (Note::E4, QUARTER), (Note::D4, QUARTER), (Note::C4, HALF),
    ],
};

/// Au Clair de la Lune: notes C D E
#[rustfmt::skip]
pub const AU_CLAIR: Song = Song {
    _name: "Au Clair de la Lune",
    eighth_ms: 300,
    buttons: [Note::C4,Note::D4,Note::E4, Note::G4,Note::F4],
    steps: &[
        (Note::C4, QUARTER), (Note::C4, QUARTER), (Note::C4, QUARTER), (Note::D4, QUARTER),
        (Note::E4, HALF), (Note::D4, HALF),
        (Note::C4, QUARTER), (Note::E4, QUARTER), (Note::D4, QUARTER), (Note::D4, QUARTER),
        (Note::C4, WHOLE),
    ],
};

/// Ode to Joy: notes C D E F G (uses all 5 buttons!)
#[rustfmt::skip]
pub const ODE_TO_JOY: Song = Song {
    _name: "Ode to Joy",
    eighth_ms: 300,
    buttons: [Note::C4,Note::D4,Note::E4, Note::G4,Note::F4],
    steps: &[
        (Note::E4, QUARTER), (Note::E4, QUARTER), (Note::F4, QUARTER), (Note::G4, QUARTER),
        (Note::G4, QUARTER), (Note::F4, QUARTER), (Note::E4, QUARTER), (Note::D4, QUARTER),
        (Note::C4, QUARTER), (Note::C4, QUARTER), (Note::D4, QUARTER), (Note::E4, QUARTER),
        (Note::E4, DOT_QUARTER), (Note::D4, EIGHTH), (Note::D4, HALF),

        (Note::E4, QUARTER), (Note::E4, QUARTER), (Note::F4, QUARTER), (Note::G4, QUARTER),
        (Note::G4, QUARTER), (Note::F4, QUARTER), (Note::E4, QUARTER), (Note::D4, QUARTER),
        (Note::C4, QUARTER), (Note::C4, QUARTER), (Note::D4, QUARTER), (Note::E4, QUARTER),
        (Note::D4, DOT_QUARTER), (Note::C4, EIGHTH), (Note::C4, HALF),
    ],
};

pub const SONGS: &[Song] = &[MARY, HOT_CROSS_BUNS, AU_CLAIR, ODE_TO_JOY];
const CHUNK_MS: u32 = 10; // buzzer slice between checks

const TICK_MS: u32 = 30;
const TICK_HZ: u32 = 1000;
const ACCENT_HZ: u32 = 1500; // higher pitch on beat 1

/// tik, tik, tik, tik. The first beat also flashes the LED of the first note.
/// Returns the (fade_ms, fade_start) of the first note's fade, for the off-by measurement.
fn count_in(cube: &mut Cube, song: &Song, first_button: usize) -> (u32, Instant) {
    let beat_ms = QUARTER as u32 * song.eighth_ms;
    let fade_ms = QUARTER as u32 * beat_ms; // fade for the last 2 beats of the count-in
    let mut fade_start = Instant::now();

    for beat in 0..4 {
        let beat_start = Instant::now();

        match beat {
            0 => {
                cube.buttons[first_button].turn_led_on(); // blink
                cube.play_tone(ACCENT_HZ, TICK_MS);
            }
            2 => {
                // start fading in: full exactly when the song starts
                cube.buttons[first_button].turn_led_on_with(fade_ms as u16);
                fade_start = Instant::now();
                cube.play_tone(TICK_HZ, TICK_MS);
            }
            _ => cube.play_tone(TICK_HZ, TICK_MS),
        }

        // wait for the rest of the beat (this is what keeps the tempo steady)
        while beat_start.elapsed() < Duration::from_millis(beat_ms as u64) {}

        if beat == 0 {
            cube.buttons[first_button].turn_led_off(); // end of the blink
        }
    }
    (fade_ms, fade_start)
}

pub fn play(cube: &mut Cube) {
    let song = &SONGS[(cube.rng.random() % SONGS.len() as u32) as usize];
    let lead_ms: u32 = song.eighth_ms * 2; //start of ned led
    let find = |note| song.buttons.iter().position(|&n| n == note).unwrap();
    let (mut fade_ms, mut fade_start) = count_in(cube, song, find(song.steps[0].0));

    for (i, &(note, length)) in song.steps.iter().enumerate() {
        let button = find(note);
        while !cube.buttons[button].is_pressed() {}
        let off_ms = (fade_start.elapsed().as_millis() as i64 - fade_ms as i64).abs();
        defmt::debug!("off by {} ms", off_ms);

        // when should the next LED start fading, and for how long?
        let note_ms = song.eighth_ms * length as u32;
        let next_fade_ms = note_ms.min(lead_ms);
        let fade_after_ms = note_ms - next_fade_ms;

        // the LED of the next note, if there is one
        let next_button = song.steps.get(i + 1).map(|&(n, _)| find(n));
        // same LED as now? Then it can't fade in until this note is over.
        let same_button = next_button == Some(button);
        let mut pending_fade = if same_button { None } else { next_button };

        let note_start = Instant::now();
        loop {
            let elapsed_ms = note_start.elapsed().as_millis() as u32;
            if elapsed_ms >= note_ms {
                break;
            }

            // check: is a fade due?
            if let Some(next_button) = pending_fade
                && elapsed_ms >= fade_after_ms
            {
                cube.buttons[next_button].turn_led_on_with(next_fade_ms as u16);
                fade_start = Instant::now();
                fade_ms = next_fade_ms;
                pending_fade = None;
            }

            // sound: only a short slice, and only while the button is held
            if cube.buttons[button].is_pressed() {
                cube.play_tone(note as u32, CHUNK_MS);
            }
        }
        cube.buttons[button].turn_led_off();
        if same_button {
            cube.buttons[button].turn_led_on_with(next_fade_ms as u16);
            fade_start = Instant::now();
            fade_ms = next_fade_ms;
        }
        cube.wait_release();
    }
}
