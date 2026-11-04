use crate::cube::Cube;
use esp_hal::time::{Duration, Instant};

///Free-play Game
/// This game will listen to input and light up the button with the corresponding sound.
/// After no input is given for 30 seconds the game will stop.
pub fn play(cube: &mut Cube) {
    defmt::info!("playing free play! just press buttons, disco! disco!");
    let mut last_input = Instant::now();
    while last_input.elapsed() < Duration::from_secs(30) {
        for i in 0..cube.buttons.len() {
            if cube.buttons[i].is_pressed() {
                cube.buttons[i].turn_led_on();
                cube.buttons[i].make_sound(&mut cube.buzzer, &cube.delay);
                cube.buttons[i].turn_led_off();
                last_input = Instant::now();
            }
        }
    }
    cube.make_game_over_sound();
}
