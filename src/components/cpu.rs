use crate::components::Display;
use crate::components::Keypad;
use crate::components::Sound;

const ROM_PATH: &str = "";
const FONT_LOAD_START: usize = 0x050;
const ROM_LOAD_START: usize = 0x200;

const MEMORY_SIZE: usize = 4096;
const REGISTERS_SIZE: usize = 16;

const FONTS: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub struct Cpu {
    pub display: Display,
    keypad: Keypad,
    sound: Sound,
    memory: [u8; MEMORY_SIZE],
    registers: [u8; REGISTERS_SIZE],
    pc: usize,
    i: usize,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    paused: bool,
}

impl Cpu {
    pub fn new(display: Display, keypad: Keypad, sound: Sound) -> Self {
        let memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        let registers: [u8; REGISTERS_SIZE] = [0; REGISTERS_SIZE];
        let pc: usize = ROM_LOAD_START;
        let i: usize = 0;
        let stack: Vec<u16> = Vec::new();
        let delay_timer: u8 = 0;
        let sound_timer: u8 = 0;
        let paused: bool = false;

        Cpu { 
            display, 
            keypad, 
            sound, 
            memory, 
            registers, 
            pc, 
            i, 
            stack, 
            delay_timer, 
            sound_timer,
            paused,
        }
    }

    pub fn init_load(&mut self) {
        self.load_fonts();
        self.load_rom();
    }

    fn load_fonts(&mut self) {
        let start = FONT_LOAD_START;
        let end = FONT_LOAD_START + FONTS.len();
        for (i, byte) in (start..end).zip(FONTS) {
            self.memory[i] = byte;
        }
    }

    fn load_rom(&mut self) {

        Cpu::read_rom_from_file(ROM_PATH)
    }

    fn read_rom_from_file(path: &str) {
        
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn cycle(&mut self) {
        let instr = self.fetch();
        self.decode_execute(instr);
    }

    fn fetch(&mut self) -> u16 {
        let i1: u8 = self.memory[self.pc];
        let i2: u8 = self.memory[self.pc + 1];

        ((i1 as u16) << 8) + i2 as u16
    }

    fn decode_execute(&mut self, instr: u16) {
        // x and y are used for register lookup
        let x: usize = (instr & 0x0F00) as usize;
        let y: usize = (instr & 0x00F0) as usize;

        let n: u8 = (instr & 0x000F) as u8;
        let nn: u8 = (instr & 0x00FF) as u8;
        let nnn: usize = (instr & 0x0FFF) as usize; // used as 12-bit memory address

        match instr & 0xF000 {
            0x0 => {
                if nnn == 0x0E0 {
                    self.display.clear()
                }
            },
            0x1 => self.jump(nnn),
            0x2 => {}
            0x3 => {}
            0x4 => {}
            0x5 => {}
            0x6 => self.register_set(x, nn),
            0x7 => self.register_add(x, nn),
            0x8 => {}
            0x9 => {}
            0xA => self.index_set(nnn),
            0xB => {}
            0xC => {}
            0xD => self.draw(x, y, n),
            0xE => {}
            0xF => {}
            _ => {}
        }
    }

    fn jump(&mut self, address: usize) {
        self.pc = address;
    }

    fn register_set(&mut self, address: usize, value: u8) {
        self.registers[address] = value;
    }

    fn register_add(&mut self, address: usize, value: u8) {
        self.registers[address] += value;
    }

    fn index_set(&mut self, value: usize) {
        self.i = value;
    }

    fn draw(&mut self, x: usize, y: usize, height: u8) {
        let x_coord = self.registers[x];
        let y_coord = self.registers[y];

        let sprite = self.memory[self.i];

        // change display
    }
}  