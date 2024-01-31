use sdl2::AudioSubsystem;
use sdl2::audio::{AudioCallback, AudioSpecDesired, AudioDevice};

pub struct Sound {
    device: AudioDevice<SquareWave>,
}

impl Sound {
    pub fn new(audio_subsystem: AudioSubsystem, sound_volume: f32) -> Self {
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1),
            samples: None
        };
        let device: AudioDevice<SquareWave> = audio_subsystem.open_playback(None, &desired_spec, |spec| {
            SquareWave {
                phase_inc: 440.0 / spec.freq as f32,
                phase: 0.0,
                volume: sound_volume,
            }
        }).unwrap();

        Sound { device }
    }

    pub fn start_sound(&mut self) {
        self.device.resume()
    }

    pub fn stop_sound(&mut self) {
        self.device.pause()
    }
    
}

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}