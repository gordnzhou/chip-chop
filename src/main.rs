extern crate sdl2;

mod emulator;
mod components;

use crate::emulator::Emulator;

pub fn main() -> Result<(), String> {

    let mut emulator: Emulator = Emulator::new(1, 1)?;
    emulator.init();
    emulator.main_loop();

    Ok(())
}
