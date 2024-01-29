use crate::components::{Cpu, Display, Keypad, Sound};

pub struct Emulator {
    cpu: Cpu,
}

impl Emulator {
    const INSTR_PER_SECOND: u16 = 700;

    pub fn new(speed: u8, scale: u8) -> Self {
        let display: Display = Display::new();
        let keypad: Keypad = Keypad::new();
        let sound: Sound = Sound::new();
        let mut cpu: Cpu = Cpu::new(display, keypad, sound);

        Emulator { cpu }
    }

    pub fn init(mut self) {
        self.cpu.cycle();
    }
}