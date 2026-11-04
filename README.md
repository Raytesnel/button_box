# Button Box

An open-source, battery-friendly game box with **5 coloured arcade buttons**, **PWM-faded LEDs** and a **buzzer**, running bare-metal Rust on an ESP32 (`esp-hal`, `no_std`).

Everything is open: the firmware, the schematic notes and the 3D-printable enclosure (designed in Onshape).

// TODO: make&insert .gif of working prototype

## Games

Press the button that matches a game's position in the menu to start it.

| Button | Mode | Status | Description |
|---|---|---|---|
| Green | **Free Play** | Done | Press a button, get its light and note. A tiny instrument. |
| Yellow | **Simon** | Planned | Repeat the growing sequence of lights and tones. |
| Red | **Reaction** | Done | A random button lights up and slowly fades. Press it before it's gone. It speeds up every 5 points. |
| Blue | **Kids** | Planned | Just some random blieps and lights, DISCO DISCO. |
| White | **Critter** | Planned | kind of "wack a mole". the "mole" can move only 1 led at the time. press it before it moves to the next led. every hit will let the "mole" move faster |

When you run out of lives you get the sad-trombone "mwap mwap mwap mwaaaap" game-over jingle.

## Features

- Smooth LED fades using the hardware **LEDC** peripheral (no CPU time spent on dimming)
- Each button has its own musical note (C4, E4, G4, C5, A5)
- Square-wave buzzer sounds generated in software
- Random game logic using the hardware RNG
- Logging over RTT with `defmt`

## Hardware

**Bill of materials**

| Qty | Part | Notes |
|---|---|---|
| 1 | xiao esp32 c3  | [xiao esp32 c3](https://wiki.seeedstudio.com/XIAO_ESP32C3_Getting_Started/) |
| 5 | Arcade buttons with LED | [led button](https://www.kiwi-electronics.com/nl/16mm-drukknop-met-led-puls-rood-2178?search=button)|
| 5 | Resistors for the LEDs | 220 ohm |
| 1 | Passive buzzer | [KW-3104](https://www.kiwi-electronics.com/nl/piezo-buzzer-actief--11258?search=buzzer) |
| 1 | 3D-printed enclosure | [onshape model](https://cad.onshape.com/documents/4d4d6337100c3f9f2fc4b8df/w/fc8cba6b7c5cb12c882601c2/e/4c7ae21a745d75b483ff06b9?renderMode=0&uiState=6aebb61106f0c4facda76375) |
| 1 | lipo battery | [PKCELL LP503035](https://www.tinytronics.nl/index.php?route=product/product&product_id=5875)

**Pinout**

| Function | Colour | GPIO |
|---|---|---|
| Button | Green | 2 |
| Button | Yellow | 5 |
| Button | Red | 7 |
| Button | Blue | 20 |
| Button | White | 9 | // TODO: needs to go to GPIO5 for wake up from deepsleep (top led on the box)
| Buzzer | – | 4 |
| LED | Green | 3 |
| LED | Yellow | 6 |
| LED | Red | 21 |
| LED | Blue | 20 |
| LED | White | 10 |

Buttons are wired between the GPIO and GND and use the internal pull-up, so a press reads as **low**. LEDs are driven by LEDC PWM channels 0–4 sharing a single 24 kHz, 8-bit timer.

## CAD model

The enclosure is designed in Onshape and is fully open:

**[Open the Onshape document](https://cad.onshape.com/documents/4d4d6337100c3f9f2fc4b8df/w/fc8cba6b7c5cb12c882601c2/e/4c7ae21a745d75b483ff06b9?renderMode=0&uiState=6aebb61106f0c4facda76375)**

## Getting started

### 1. Development environment

The project uses a [Nix](https://nixos.org/) dev shell, so you don't need to install the toolchain by hand.

```sh
# enable flakes if you haven't: https://nixos.wiki/wiki/Flakes
nix develop
```

This gives you the Rust toolchain, the embedded target and the flashing tools.

### 2. Build and flash

Connect the board over USB and run:

```sh
cargo run --release
```
Logs from `defmt` show up in the terminal via RTT.

## Project structure

```
src/
├── main.rs        # hardware setup (LEDC, pins, buzzer) and the main loop
├── button.rs      # Button: switch + PWM LED + note, fades and sounds
├── cube.rs        # Cube: the 5 buttons, buzzer, delay, RNG and shared helpers
├── menu.rs        # select_mode: pick a game with a button press
└── games/
    ├── mod.rs     # Mode enum and run_game dispatch
    ├── free_play.rs
    └── reaction.rs
```

## Adding a new game

1. Create `src/games/my_game.rs` with a `pub fn play(cube: &mut Cube)`.
2. Add `mod my_game;` and a `Mode` variant in `games/mod.rs`.
3. Add the variant to `MODES` (the position in the array decides which button starts it).
4. Add a match arm in `run_game` in `mod.rs`.

## Roadmap

- [ ] Simon
- [ ] Kids mode
- [ ] Critter mode
- [ ] Penalty sound for pressing the wrong button in Reaction
- [ ] Battery power and sleep mode

## Contributing

Issues and pull requests are welcome, whether that's a new game, a better sound or a tweak to the enclosure.
