use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::components::{Cpu, Display, Keypad, Sound};

pub struct Emulator {
    cpu: Cpu,
    speed: u64,
    // sdl2 object
}

const CPU_HZ: u64 = 700;
const DISPLAY_HZ: u64 = 60;

impl Emulator {
    pub fn new(speed: u64, scale: u32) -> Self {
        // load sdl2 and pass them to components
        let display: Display = Display::new(scale);
        let keypad: Keypad = Keypad::new();
        let sound: Sound = Sound::new();
        let cpu: Cpu = Cpu::new(display, keypad, sound);

        Emulator { cpu, speed }
    }

    pub fn init(&mut self) {
        // initialize sdl2 object
        self.cpu.init_load();
    }

    pub fn main_loop(&mut self) {
        let mut last_cpu = Instant::now();
        let mut last_display = Instant::now();

        loop {
            // run CPU cycle at CPU_HZ per second
            if last_cpu.elapsed() >= Duration::from_micros(1000000 / (CPU_HZ * self.speed)) {
                self.cpu.cycle();
                last_cpu = Instant::now();
            }

            // update timers and display at DISPLAY_HZ per second
            if last_display.elapsed() >= Duration::from_micros(1000000 / (DISPLAY_HZ * self.speed)) {
                self.cpu.update_timers();
                self.cpu.display.update_display();
                last_display = Instant::now();
            }

            sleep(Duration::from_micros(100));
        }
    }

}