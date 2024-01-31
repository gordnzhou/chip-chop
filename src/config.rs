use sdl2::keyboard::Keycode;

pub const SPEED: f32 = 0.05;
pub const SCALE: i32 = 15;

// set to true for newer models starting with
// CHIP-48 and SUPER-CHIP
pub const USE_NEW: bool = true;

// affects FX55 and FX65 instructions
pub const USE_NEW_LOAD: bool = true;

pub const CPU_HZ: u64 = 700;
pub const DISPLAY_HZ: u64 = 60;

pub const ROM_PATH: &str = "src/roms/pong.ch8";

pub const FONT_LOAD_START: usize = 0x050;
pub const ROM_LOAD_START: usize = 0x200;

pub const SOUND_VOLUME: f32 = 0.01;

// assumes QWERTY keyboard is used
pub const KEYMAPPINGS: [Keycode; 16] = [
    Keycode::Num1,
    Keycode::Num2,
    Keycode::Num3,
    Keycode::Num4,
    Keycode::Q,
    Keycode::W,
    Keycode::E,
    Keycode::R,
    Keycode::A,
    Keycode::S,
    Keycode::D,
    Keycode::F,
    Keycode::Z,
    Keycode::X,
    Keycode::C,
    Keycode::V
];