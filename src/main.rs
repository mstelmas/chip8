#[macro_use] extern crate log;
extern crate env_logger;

use std::env;
use std::fs;
use std::io::prelude::*;

mod arch;

const ROM_LOCATION: u16 = 0x200;

struct Chip8 {
    cpu: arch::cpu::Cpu
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: arch::cpu::Cpu::new()
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.cpu.interconnect.write_memory(ROM_LOCATION, rom);
    }

    pub fn run(&mut self) {
        self.cpu.run();
    }
}


fn main() {
    let rom_path = env::args().nth(1)
        .expect("Provide rom location!");

    env_logger::init();

    let code = fs::read(&rom_path)
        .expect("Could not read file");

    info!("Starting Chip8 emulation for ROM at: {:#}", rom_path);

    let mut chip8 = Chip8::new();
    chip8.load_rom(&code);
    chip8.run();

}
