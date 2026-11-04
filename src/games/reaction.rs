use crate::cube::Cube;
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
    let level_times = [2000, 1500, 1200, 800, 500, 300, 150, 100, 0];
    while lives > 0 {
        let time_to_push = level_times[(score / 5).min(level_times.len() - 1)];
        let random_button = cube.random_button();
        cube.buttons[random_button].led.set_high();
        cube.buttons[random_button].make_sound(&mut cube.buzzer, &cube.delay);
        let button_timer = Instant::now();
        while button_timer.elapsed() < Duration::from_millis(time_to_push + 1000) {
            if cube.buttons[random_button].is_pressed() {
                score += 1;
                cube.buttons[random_button].led.set_low();
                cube.wait_release();
                break;
            }
            if button_timer.elapsed() > Duration::from_millis(time_to_push) {
                lives -= 1;
                cube.buttons[random_button].led.set_low();
                cube.wait_release();
                break;
            }
        }
    }
}
