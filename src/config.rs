pub const SPEED: f32 = 1.0;
pub const SCALE: i32 = 15;

// set to true for newer models starting with
// CHIP-48 and SUPER-CHIP
pub const USE_NEW: bool = false;

// affects FX55 and FX65 instructions
pub const USE_NEW_LOAD: bool = true;

pub const CPU_HZ: u64 = 100;
pub const DISPLAY_HZ: u64 = 10;

pub const ROM_PATH: &str = "src/roms/IBM Logo.ch8";

pub const FONT_LOAD_START: usize = 0x050;
pub const ROM_LOAD_START: usize = 0x200;