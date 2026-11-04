use crate::{button::Button, cube::Cube};
use esp_hal::time::{Duration, Instant};

/// Reaction Game
/// This game will blink a random light and slowly goes out.
/// Press the button before the light is out and a new random light will pop up.
/// Every x hits you have on the light you go up a level.
/// Each level the timer gets shorter before you need to push.
/// After you press 3 times to late the game ends.
pub fn play(cube: &mut Cube) {
    let mut lives = 3;
    let mut score = 0;
    let level_times = [2500, 2000, 1500, 1200, 800, 500, 300, 150, 100];
    let mut level = 0;
    defmt::info!("Let the reaction game begin!");
    while lives > 0 {
        let time_to_push = level_times[(score / 5).min(level_times.len() - 1)];
        let random_button = cube.random_button();
        cube.buttons[random_button].turn_led_on();
        cube.buttons[random_button].make_sound(&mut cube.buzzer, &cube.delay);
        cube.buttons[random_button].turn_led_off_with(time_to_push as u16);
        if wait_for_press(&cube.buttons[random_button], time_to_push) {
            score += 1;
        } else {
            lives -= 1;
        }
        cube.buttons[random_button].turn_led_off();
        cube.wait_release();
        if (score / 5) as u64 > level {
            defmt::info!("good job! level{:?}, lets go faster", level);
            level += 1;
        }
    }
    defmt::info!(
        "Good job!, you have reached level {:?}, ending game.",
        level
    );
    cube.make_game_over_sound();
}

fn wait_for_press(button: &Button, timeout_ms: u64) -> bool {
    let start = Instant::now();
    while start.elapsed() < Duration::from_millis(timeout_ms) {
        if button.is_pressed() {
            return true;
        }
    }
    false
}
