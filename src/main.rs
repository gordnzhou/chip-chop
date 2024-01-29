extern crate sdl2;

mod emulator;
mod components;

use crate::emulator::Emulator;

pub fn main() {
    println!("Hello, world!!!");

    let emulator: Emulator = Emulator::new(1, 1);
    emulator.init();
}
