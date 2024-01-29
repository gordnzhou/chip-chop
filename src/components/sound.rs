pub struct Sound {
    is_playing: bool
}

impl Sound {
    pub fn new() -> Self {
        let is_playing: bool = false;
        Sound { is_playing }
    }

    // TODO: 
    // play beeping sound while is_playing
    // change is_playing based on cpu.sound_timer
}