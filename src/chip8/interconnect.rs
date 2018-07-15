use super::display;

const RAM_SIZE: usize = 4096;

pub struct Interconnect {
    ram: [u8; RAM_SIZE],
    pub display: display::Display
}

impl Interconnect {
    // TODO: inject display
    pub fn new() -> Interconnect {
        Interconnect {
            ram: [0; RAM_SIZE],
            display: display::Display::new()
        }
    }

    pub fn read_word(&self, location: u16) -> u16 {
        (self.ram[location as usize] as u16) << 8 | (self.ram[(location + 1) as usize] as u16)
    }

    // TODO: bounds checking
    pub fn write_memory(&mut self, location: u16, data: &Vec<u8>) {
        for i in 0..data.len() {
            self.ram[location as usize + i] = data[i];
        }
    }
}
