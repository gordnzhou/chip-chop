# Chip Chop

A mostly-accurate CHIP-8 Emulator built using Rust and SDL2.


## Setup

1. If you haven't already, install [Rust](https://www.rust-lang.org/tools/install) and [SDL2](https://github.com/libsdl-org/SDL/releases/) on your local machine.

2. After cloning the repository add your CHIP-8 ROMS to the `src/roms` folder, and feel free to adjust the constants in `src/config.rs` to your liking.

3. In this folder, do:

```
cargo run
```
To start the emulator!

## Tips

- You can **exit** the emulator using the escape button
- You can **pause** the emulator using the space button (can be changed in `config.rs`)
- During the emulation, you may see some sprites flickering. That is expected, due to the way rendering worked back in the original CHIP-8.




