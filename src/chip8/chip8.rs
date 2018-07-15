use super::{Cpu, Interconnect};

const ROM_LOCATION: u16 = 0x200;

pub struct Chip8 {
    cpu: Cpu,
    interconnect: Interconnect
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            interconnect: Interconnect::new()
        }
    }

    pub fn load_rom(&mut self, rom: &Vec<u8>) {
        self.interconnect.write_memory(ROM_LOCATION, rom);
    }

    pub fn step(&mut self) {
        self.cpu.execute_cycle(&mut self.interconnect);
    }
}
