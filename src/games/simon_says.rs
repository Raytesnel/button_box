use crate::cube::Cube;
const MAX_ROUNDS: usize = 40;

///Simon-says Game
/// This game wil light up a random button.
/// When the correct button is pressed a new random button will be added to the list.
/// After no input is given for 30 seconds or a wrond input is given, the game stopped.
pub fn play(cube: &mut Cube) {
    defmt::info!("playing Simon-says! good luck.");
    let buttons: [usize; MAX_ROUNDS] = core::array::from_fn(|_index| cube.random_button());
    let mut round: usize = 1;
    'game: while round <= MAX_ROUNDS {
        // show sequence of buttons to press.
        for &score_button in buttons.iter().take(round + 1) {
            defmt::info!(
                "step {:?}: press {:?}",
                score_button,
                cube.buttons[score_button].color
            );
            cube.buttons[score_button].blink_and_beep(&mut cube.buzzer, &cube.delay);
            cube.delay.delay_millis(500);
        }
        // wait for button input.
        for &score_button in buttons.iter().take(round + 1) {
            match cube.wait_press_timeout(30_000) {
                Some(pressed) if pressed == score_button => {
                    cube.buttons[pressed].blink_and_beep(&mut cube.buzzer, &cube.delay);
                    cube.wait_release();
                }
                _ => break 'game, // wrong button or timeout
            }
        }
        round += 1;
    }
    cube.make_game_over_sound();
}
