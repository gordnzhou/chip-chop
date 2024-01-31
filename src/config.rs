use sdl2::keyboard::Keycode;

pub const ROM_PATH: &str = "src/roms/space_invaders.ch8";

pub const CPU_HZ: u64 = 400;
pub const DISPLAY_HZ: u64 = 60;

pub const SPEED: f32 = 1.0;
pub const SCALE: i32 = 15;

pub const SOUND_VOLUME: f32 = 0.02;

// set to true for newer models starting with
// CHIP-48 and SUPER-CHIP
// affects 8XY6, 8XYE, BXXX instructions
pub const USE_NEW: bool = true;

// affects FX55 and FX65 instructions
pub const USE_NEW_LOAD: bool = true;

pub const FONT_LOAD_START: usize = 0x050;
pub const ROM_LOAD_START: usize = 0x200;

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