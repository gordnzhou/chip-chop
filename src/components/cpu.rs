use std::fs::File;
use std::io::{self, Read};

use rand::Rng;

use crate::components::{Display, Keypad, Sound};
use crate::config::{ROM_PATH, FONT_LOAD_START, ROM_LOAD_START, USE_NEW, USE_NEW_LOAD};

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
    stack: Vec<usize>,
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new(display: Display, keypad: Keypad, sound: Sound) -> Self {
        let memory: [u8; MEMORY_SIZE] = [0; MEMORY_SIZE];
        let registers: [u8; REGISTERS_SIZE] = [0; REGISTERS_SIZE];
        let pc: usize = ROM_LOAD_START;
        let i: usize = 0;
        let stack: Vec<usize> = Vec::new();
        let delay_timer: u8 = 0;
        let sound_timer: u8 = 0;

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
        match Cpu::read_rom_from_file(ROM_PATH) {
            Ok(rom_data) => {
                println!("Sucessfully read ROM.");
                println!("ROM size: {} bytes", rom_data.len());
                println!("First 16 bytes: {:?}", &rom_data[..16]);

                for i in 0..rom_data.len() {
                    self.memory[ROM_LOAD_START + i] = rom_data[i];
                }
            }
            Err(err) => {
                eprintln!("Error reading ROM file: {}", err);
            }
        }
    }

    fn read_rom_from_file(file_path: &str) -> io::Result<Vec<u8>> {
        let mut file = File::open(file_path)?;

        let mut rom_data = Vec::new();
        file.read_to_end(&mut rom_data)?;

        Ok(rom_data)
    }

    pub fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            self.sound_timer -= 1;

            if self.sound_timer == 0 {
                self.sound.stop_sound();
            }
        }
    }

    pub fn cycle(&mut self) {
        let instr = self.fetch();
        self.decode_execute(instr);
    }

    fn fetch(&mut self) -> u16 {
        let i1: u8 = self.memory[self.pc];
        let i2: u8 = self.memory[self.pc + 1];

        self.pc += 2;

        ((i1 as u16) << 8) + i2 as u16
    }

    fn decode_execute(&mut self, instr: u16) {
        // x and y are used for register lookup
        let x: usize = ((instr & 0x0F00) >> 8) as usize;
        let y: usize = ((instr & 0x00F0) >> 4) as usize;

        let vx: u8 = self.registers[x];
        let vy: u8 = self.registers[y];

        let n: u8 = (instr & 0x000F) as u8;
        let nn: u8 = (instr & 0x00FF) as u8;
        let nnn: usize = (instr & 0x0FFF) as usize; // used as 12-bit memory address

        if instr != 0 {
            println!("Instruction: {:04x}, Type: {:01x}", instr, (instr & 0xF000) >> 12);
        }
        
        match (instr & 0xF000) >> 12 {
            0x0 => {
                match nnn {
                    0x0E0 => self.display.clear(),
                    0x0EE => self.pop_subroutine(),
                    _ => {}
                }
            },
            0x1 => self.jump(nnn),
            0x2 => self.push_subroutine(nnn),
            0x3 => self.skip_if_equal(vx, nn),
            0x4 => self.skip_if_not_equal(vx, nn),
            0x5 => self.skip_if_equal(vx, vy),
            0x6 => self.register_set(x, nn),
            0x7 => self.register_set(x, vx + n),
            0x8 => {

                match n {
                    0x0 => self.register_set(x, vy),
                    0x1 => self.register_set(x, vx | vy),
                    0x2 => self.register_set(x, vx & vy),
                    0x3 => self.register_set(x, vx ^ vy),
                    0x4 => self.register_add(x, vx as u16, vy as u16),
                    0x5 => self.register_sub(x, vx, vy),
                    0x6 => {
                        if USE_NEW {
                            self.register_set(x, vy);
                        }
                        self.shift_right(x);
                    },
                    0x7 => self.register_sub(x, vy, vx),
                    0xE => {
                        if USE_NEW {
                            self.register_set(x, vy);
                        }
                        self.shift_left(x);
                    },
                    _ => {}
                }
            }
            0x9 => self.skip_if_not_equal(vx, vy),
            0xa => self.index_set(nnn),
            0xb => {
                if USE_NEW {
                    self.jump(nnn + vx as usize);
                }
                else {
                    self.jump(nnn + self.registers[0x0] as usize);
                }
            },
            0xc => self.set_random(x, nn),
            0xd => self.draw(x, y, n),
            0xe => {
                match nn {
                    0x9E => self.skip_if_pressed(vx as usize),
                    0xA1 => self.skip_if_not_pressed(vx as usize),
                    _ => {}
                }
            }
            0xf => {
                match nn {
                    0x07 => self.register_set(x, self.delay_timer),
                    0x0A => self.wait_for_key(x),
                    0x15 => self.set_delay_timer(vx),
                    0x18 => self.set_sound_timer(vx),
                    0x1E => self.index_set(self.i + vx as usize),
                    0x29 => self.index_set(FONT_LOAD_START + (vx * 5) as usize),
                    0x33 => self.store_decimal_digits(vx),
                    0x55 => {
                        if !USE_NEW_LOAD {
                            self.i += x + 1;
                        }
                        self.load_memory_from_registers(x);
                    },
                    0x65 => {
                        if !USE_NEW_LOAD {
                            self.i += x + 1;
                        }
                        self.load_registers_from_memory(x);
                    },
                    _ => {}
                }
            }
            _ => {}
        }
    }

    fn jump(&mut self, address: usize) {
        self.pc = address;
    }

    fn load_memory_from_registers(&mut self, end: usize) {
        for reg_index in 0..=end {
            self.memory[self.i + reg_index] = self.registers[reg_index];
        }
    }
    
    fn load_registers_from_memory(&mut self, end: usize) {
        for reg_index in 0..=end {
            self.registers[reg_index] = self.memory[self.i + reg_index];
        }
    }

    fn store_decimal_digits(&mut self, value: u8) {
        self.memory[self.i] = value / 100;
        self.memory[self.i + 1] = (value % 100) / 10;
        self.memory[self.i + 2] = value % 10;
    }

    fn wait_for_key(&mut self, address: usize) {
        for i in 0..0xF {
            if self.keypad.is_pressed(i) {
                self.registers[address] = i as u8;
                return
            }
        }
        
        self.pc -= 2;
    }

    fn set_random(&mut self, address: usize, value: u8) {
        let mut rng = rand::thread_rng();
        let r = rng.gen_range(0..=value);

        self.registers[address] = value & r;
    }

    fn set_delay_timer(&mut self, value: u8) {
        self.delay_timer = value;
    }

    fn set_sound_timer(&mut self, value: u8) {
        self.sound_timer = value;
    }

    fn skip_if_equal(&mut self, a: u8, b: u8) {
        if a == b {
            self.pc += 2;
        }
    }

    fn skip_if_not_equal(&mut self, a: u8, b: u8) {
        if a != b {
            self.pc += 2;
        }
    }

    fn skip_if_pressed(&mut self, key: usize) {
        if self.keypad.is_pressed(key) {
            self.pc += 2;
        }
    }

    fn skip_if_not_pressed(&mut self, key: usize) {
        if !self.keypad.is_pressed(key) {
            self.pc += 2;
        }
    }

    fn shift_left(&mut self, address: usize) {
        self.registers[0xF] = self.registers[address] & 0x80;
        self.registers[address] <<= 1;
    }

    fn shift_right(&mut self, address: usize) {
        self.registers[0xF] = self.registers[address] & 0x1;
        self.registers[address] >>= 1;
    }

    fn push_subroutine(&mut self, address: usize) {
        self.pc = address;
        self.stack.push(address);
    }

    fn pop_subroutine(&mut self) {
        match self.stack.pop() {
            Some(address ) => self.pc = address,
            None => {}
        }
    }

    fn register_set(&mut self, address: usize, value: u8) {
        self.registers[address] = value;
    }

    fn register_add(&mut self, address: usize, a: u16, b: u16) {
        let sum = a + b;

        self.registers[0xF] = 0;
        if sum > 0xFF {
            self.registers[0xF] = 1;
        }

        self.registers[address] = sum as u8;
    }

    fn register_sub(&mut self, address: usize, a: u8, b: u8) {
        self.registers[0xF] = 0;
        if a > b {
            self.registers[0xF] = 1;
        }

        self.registers[address] = a - b;
    }

    fn index_set(&mut self, value: usize) {
        self.i = value;
    }

    fn draw(&mut self, x: usize, y: usize, height: u8) {

        let x_coord = self.registers[x] as usize;
        let y_coord = self.registers[y] as usize;

        self.registers[0xF] = 0;

        println!("Drawing sprite at I ({:#04x}) of height {} at coords ({}, {})", self.i, height, x_coord, y_coord);

        for row in 0..height as usize {
            let sprite = self.memory[self.i + row];

            for col in 0..8 as usize {
                let x = x_coord + col;
                let y = y_coord + row;

                if (sprite & (1 << (7 - col))) > 0 {
                    self.display.flip_pixel(x, y);
                    if !self.display.get_pixel(x, y) {
                        self.registers[0xF] = 1;
                    }
                }
            }
        }
    }
}  