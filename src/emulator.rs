use sdl2::video::Window;
use sdl2::{Sdl, VideoSubsystem};

use std::time::{Duration, Instant};
use std::thread::sleep;

use crate::components::{Cpu, Display, Keypad, Sound};
use crate::config::{CPU_HZ, DISPLAY_HZ};

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Emulator {
    cpu: Cpu,
    speed: f32,
}

impl Emulator {
    pub fn init(speed: f32, scale: i32) -> Result<Self, String> {
        let sdl_context: Sdl = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        let audio_subsystem = sdl_context.audio()?;
        let event_pump = sdl_context.event_pump()?;
        let window = Emulator::build_window(video_subsystem, scale as u32)?;

        let display: Display = Display::new(window, scale)?;
        let keypad: Keypad = Keypad::new(event_pump);
        let sound: Sound = Sound::new(audio_subsystem);
        let mut cpu: Cpu = Cpu::new(display, keypad, sound);
        cpu.init_load();

        Ok(Emulator { cpu, speed })
    }

    pub fn build_window(video_subsystem: VideoSubsystem, scale: u32) -> Result<Window, String> {
        let window_width = WIDTH as u32 * scale;
        let window_height = HEIGHT as u32 * scale;

        let window = video_subsystem
            .window("CHIP-8 Emulator", window_width, window_height)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        println!("Created window of width {} and height {}", window_width, window_height);
        Ok(window)
    }
    
    pub fn main_loop(&mut self) {
        let mut last_cpu = Instant::now();
        let mut last_display = Instant::now();

        let cpu_delta_t = 1000000.0 / (CPU_HZ as f32 * self.speed);
        let display_delta_t = 1000000.0 / (DISPLAY_HZ as f32 * self.speed);

        loop {
            if !self.cpu.keypad.check_inputs() {
                break;
            }

            // run CPU cycle at CPU_HZ per second
            if last_cpu.elapsed() >= Duration::from_micros(cpu_delta_t as u64) {
                last_cpu = Instant::now();

                self.cpu.cycle();
            }

            // update timers and display at DISPLAY_HZ per second
            if last_display.elapsed() >= Duration::from_micros(display_delta_t as u64) {
                last_display = Instant::now();

                self.cpu.update_timers();
                self.cpu.display.update_display();
                self.cpu.display.render_canvas();
            }

            sleep(Duration::from_micros(100));
        }
    }

}