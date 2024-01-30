use sdl2::{video::Window, Sdl};

use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::components::{Cpu, Display, Keypad, Sound};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Emulator {
    cpu: Cpu,
    speed: u64,
}

const CPU_HZ: u64 = 700;
const DISPLAY_HZ: u64 = 60;

impl Emulator {
    pub fn new(speed: u64, scale: i32) -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;

        let video_subsystem = sdl_context.video()?;
        let window: Window = video_subsystem
            .window("CHIP-8 Emulator", WIDTH as u32 * scale as u32, HEIGHT as u32 * scale as u32)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let display: Display = Display::new(window, scale)?;
        let keypad: Keypad = Keypad::new();
        let sound: Sound = Sound::new();
        let cpu: Cpu = Cpu::new(display, keypad, sound);

        Ok(Emulator { cpu, speed })
    }

    pub fn init(&mut self) {
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