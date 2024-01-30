extern crate sdl2;

mod emulator;
mod components;

use crate::emulator::Emulator;

// configurable
const SPEED: f32 = 1.0;
const SCALE: i32 = 15;

pub fn main() -> Result<(), String> {

    let mut emulator: Emulator = Emulator::new(SPEED, SCALE)?;
    emulator.init();
    emulator.main_loop();

    Ok(())
}
