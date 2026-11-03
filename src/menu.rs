use crate::cube::Cube;
use crate::games::{MODES, Mode};

pub fn select_mode(cube: &mut Cube) -> Mode {
    defmt::info!("Entering menu:");
    for (button, mode) in cube.buttons.iter().zip(MODES.iter()) {
        defmt::info!("  press {} for {}", button.color, mode);
    }
    cube.wait_release();
    cube.all_leds(true);
    let idx = loop {
        if let Some(i) = cube.buttons.iter().position(|b| b.is_pressed()) {
            break i;
        }
    };
    defmt::info!("button {:?} is pressed", idx);
    cube.all_leds(false);
    cube.buttons[idx].led.set_high();
    cube.buttons[idx].make_sound(&mut cube.buzzer, &cube.delay);
    cube.buttons[idx].led.set_low();
    MODES[idx]
}
