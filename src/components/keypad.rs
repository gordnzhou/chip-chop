

// add keymappings

pub struct Keypad {
    pressed: [bool; 16]
}

impl Keypad {
    pub fn new() -> Self {
        let pressed: [bool; 16] =  [false; 16];
        Keypad { pressed }
    }

    pub fn is_pressed(&self, key: usize) -> bool {
        self.pressed[key]
    }
}