use super::display;
use super::keypad;
use super::mem_map;

pub struct Interconnect {
    ram: [u8; mem_map::RAM_SIZE],
    display: display::Display,
    keypad: keypad::Keypad
}

impl Interconnect {
    pub fn new(display: display::Display, keypad: keypad::Keypad) -> Interconnect {
        Interconnect {
            ram: [0; mem_map::RAM_SIZE],
            display,
            keypad
        }
    }

    pub fn display(&mut self) -> &mut display::Display {
        &mut self.display
    }

    pub fn ram(&mut self) -> &mut [u8; mem_map::RAM_SIZE] {
        &mut self.ram
    }

    pub fn keypad(&mut self) -> &mut keypad::Keypad {
        &mut self.keypad
    }

    pub fn read_word(&self, location: u16) -> u16 {
        (self.ram[location as usize] as u16) << 8 | (self.ram[(location + 1) as usize] as u16)
    }

    pub fn read_byte(&self, location: u16) -> u8 {
        self.ram[location as usize]
    }

    // TODO: bounds checking
    pub fn write_memory(&mut self, location: u16, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.ram[location as usize + i] = data[i];
        }
    }
}
