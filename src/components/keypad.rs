use sdl2::{keyboard::Keycode, EventPump};
use sdl2::event::{Event, WindowEvent};

use crate::emulator::EmulatorState;
use crate::config::{KEYMAPPINGS, PAUSE_KEY};

pub struct Keypad {
    pub pressed: [bool; 16],
    event_pump: EventPump,
}

impl Keypad {
    pub fn new(event_pump: EventPump) -> Self {
        let pressed: [bool; 16] =  [false; 16];
        Keypad { pressed, event_pump }
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.pressed[key]
    }

    pub fn check_inputs(&mut self) -> EmulatorState {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { 
                    keycode: Some(Keycode::Escape), ..
                } => return EmulatorState::Stopped,
                Event::KeyDown { keycode: Some(key), repeat, ..} => {
                    if key == PAUSE_KEY {
                        return EmulatorState::Paused
                    }
                    
                    if !repeat {
                        for i in 0..16 {
                            if KEYMAPPINGS[i] == key {
                                self.pressed[i] = true;
                            }
                        }
                    }
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    for i in 0..16 {
                        if KEYMAPPINGS[i] == key {
                            self.pressed[i] = false;
                        }
                    }
                }
                Event::Window { win_event: WindowEvent::Close, .. } => {
                    return EmulatorState::Stopped;
                }
                _ => {}
            }
        }
        EmulatorState::Playing
    }
}