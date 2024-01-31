extern crate sdl2;
extern crate rand;

mod emulator;
mod components;
mod config;

use std::{io, fs};
use crate::{config::ROM_PATH, emulator::Emulator};

fn ask_for_input<T: std::str::FromStr>(variable: &mut T, msg: &str) {
    loop {
        println!("{} (or nothing for default): ", msg);

        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).expect("Failed to read line");

        if input_string.trim().is_empty() {
            return;
        }

        match input_string.trim().parse::<T>() {
            Ok(value) => {
                *variable = value;
                return;
            }
            Err(_) => {
                println!("Input is invalid. Please enter a valid input!");
            }
        }
    }
}

fn list_rom_files() -> Result<Vec<String>, io::Error> {
    let entries = fs::read_dir("src/roms")?;

    let mut files: Vec<String> = Vec::new();

    for (index, entry) in entries.enumerate() {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().into_string().unwrap_or_default();
            println!("{} - {}", index + 1, file_name);
            files.push(file_name);
        }
    }

    Ok(files)
}

fn select_file(files: &[String]) -> Option<String> {
    println!("Enter the number of the ROM you want to select:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match input.trim().parse::<usize>() {
        Ok(index) if index > 0 && index <= files.len() => {
            Some(files[index - 1].clone())
        }
        _ => {
            println!("Invalid ROM selection. Please enter a valid number.");
            None
        }
    }
}

pub fn main() -> Result<(), String> {
    println!("Welcome to CHIP-8 Emulator!");

    let mut speed: f32 = 1.0;
    ask_for_input(&mut speed, "Please enter a floating point number for GAME SPEED");
    let mut scale: i32 = 15;
    ask_for_input(&mut scale, "Please enter an integer for WINDOW SCALE");
    let mut sound_volume: f32 = 0.02;
    ask_for_input(&mut sound_volume, "Please enter a floating point number for GAME VOLUME");

    'main: loop {
        let mut rom_path: String = ROM_PATH.to_string();
        while rom_path == ROM_PATH {
            match list_rom_files() {
                Ok(files) => {
                    match select_file(&files) {
                        Some(selected) => {
                            rom_path.push('/');
                            rom_path.push_str(&*selected);
                        },
                        _ => println!("No ROM selected!")
                    }
                }
                Err(err) => { 
                    eprintln!("Error listing ROMS {}", err);
                    break 'main
                }
            }
        }

        println!("Playing ROM at {}", rom_path);
        let mut emulator: Emulator = Emulator::init(speed, scale, sound_volume, &*rom_path)?;
        emulator.main_loop();
    }

    Ok(())
}


