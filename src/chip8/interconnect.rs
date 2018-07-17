use super::display;

const RAM_SIZE: usize = 4096;

pub struct Interconnect {
    ram: [u8; RAM_SIZE],
    display: display::Display
}

impl Interconnect {
    pub fn new(display: display::Display) -> Interconnect {
        Interconnect {
            ram: [0; RAM_SIZE],
            display
        }
    }

    pub fn display(&mut self) -> &mut display::Display {
        &mut self.display
    }

    pub fn ram(&mut self) -> &mut [u8; RAM_SIZE] {
        &mut self.ram
    }

    pub fn read_word(&self, location: u16) -> u16 {
        (self.ram[location as usize] as u16) << 8 | (self.ram[(location + 1) as usize] as u16)
    }

    pub fn read_byte(&self, location: u16) -> u8 {
        self.ram[location as usize]
    }

    // TODO: bounds checking
    pub fn write_memory(&mut self, location: u16, data: &Vec<u8>) {
        error!("{}", data.len());
        for i in 0..data.len() {
            self.ram[location as usize + i] = data[i];
        }
    }
}
