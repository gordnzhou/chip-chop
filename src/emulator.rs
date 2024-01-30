use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{video::Window, Sdl};

use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::components::{Cpu, Display, Keypad, Sound};

// configurable
const CPU_HZ: u64 = 100;
const DISPLAY_HZ: u64 = 10;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Emulator {
    cpu: Cpu,
    speed: f32,
    event_pump: EventPump,
}

impl Emulator {
    pub fn new(speed: f32, scale: i32) -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;

        let window_width = WIDTH as u32 * scale as u32;
        let window_height = HEIGHT as u32 * scale as u32;
        let window: Window = video_subsystem
            .window("CHIP-8 Emulator", window_width, window_height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;
        println!("Window Width: {}, Window Height: {}", window_width, window_height);

        let display: Display = Display::new(window, scale)?;
        let keypad: Keypad = Keypad::new();
        let sound: Sound = Sound::new();
        let cpu: Cpu = Cpu::new(display, keypad, sound);

        Ok(Emulator { cpu, speed, event_pump })
    }

    pub fn init(&mut self) {
        self.cpu.init_load();
    }
    
    pub fn main_loop(&mut self) {
        let mut last_cpu = Instant::now();
        let mut last_display = Instant::now();
        let ms: f32 = 1000000.0;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } 
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            // run CPU cycle at CPU_HZ per second
            if last_cpu.elapsed() >= Duration::from_micros((ms / (CPU_HZ as f32 * self.speed)) as u64) {
                self.cpu.cycle();
                last_cpu = Instant::now();
            }

            // update timers and display at DISPLAY_HZ per second
            if last_display.elapsed() >= Duration::from_micros((ms / (DISPLAY_HZ as f32 * self.speed)) as u64) {
                self.cpu.update_timers();
                self.cpu.display.update_display();
                last_display = Instant::now();
            }

            sleep(Duration::from_micros(100));
        }
    }

}