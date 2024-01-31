extern crate sdl2;
extern crate rand;

mod emulator;
mod components;
mod config;

use crate::emulator::Emulator;
use crate::config::{SPEED, SCALE};

pub fn main() -> Result<(), String> {

    let mut emulator: Emulator = Emulator::init(SPEED, SCALE)?;
    emulator.main_loop();

    Ok(())
}
