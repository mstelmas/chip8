use super::interconnect;

pub struct Cpu {
    v: [u8; 16],
    pc: u16,
    stack: [u16; 16],
    sp: u8,

    pub interconnect: interconnect::Interconnect
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            pc: 0x200,
            stack: [0; 16],
            sp: 0,

            interconnect: interconnect::Interconnect::new()
        }
    }
    
    pub fn run(&mut self) {
        for i in 0..130 {
            let opcode = self.interconnect.read_word(self.pc);
            self.execute_opcode(opcode);
        }
    }

    fn execute_opcode(&mut self, opcode: u16) {

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let nn = opcode & 0x00FF;
        let n = opcode & 0x000F;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];

        match (op_1, op_2, op_3, op_4) {
            (0x0, _, _, _) => trace!("[CALL] Call RCA 1802 program at {:#x}", nnn),
            (0x0, 0x0, 0xE, 0x0) => trace!("[DISPLAY] Clear screen"),
            (0x0, 0x0, 0xE, 0xE) => trace!("[FLOW] Return from subroutine"),
            (0x1, _, _, _) => {
                trace!("[FLOW] Jump to: {:#x}", nnn);
                self.pc = nnn;
            },
            (0x2, _, _, _) => trace!("[FLOW] Call subroutine at {:#x}", nnn),
            (0x3, _, _, _) => trace!("[COND] Skip next instruction if V{:x} equals {:#x}", x, nn),
            (0x4, _, _, _) => trace!("[COND] Skip next instruction if V{:x} does not equal {:#x}", x, nn),
            (0x5, _, _, 0x0) => trace!("[COND] Skip next instruction if V{:x} equals V{:x}", x, y),
            (0x6, _, _, _) => {
                trace!("[CONST] Set V{:x} to {:#x}", x, nn);
                self.pc += 2;
            },
            (0x7, _, _, _) => {
                trace!("[CONST] Add {:#x} to V{:x}", nn, x);
                self.pc += 2;
            },
            (0x8, _, _, 0x0) => {
                trace!("[ASSIGN] Set V{:x} to the value of V{:x}", x, y);
                self.pc += 2;
            },
            (0x8, _, _, 0x1) => {
                trace!("[BITOP] Set V{:x} to V{:x} OR V{:x}", x, x, y);
                self.pc += 2;
            },
            (0x8, _, _, 0x2) => {
                trace!("[BITOP] Set V{:x} to V{:x} AND V{:x}", x, x, y);
                self.pc += 2;
            },
            (0x8, _, _, 0x3) => {
                trace!("[BITOP] Set V{:x} to V{:x} XOR V{:x}", x, x, y);
                self.pc += 2;
            },
            (0x8, _, _, 0x4) => {
                trace!("[MATH] Add V{:x} to V{:x}", x, y);
                self.pc += 2;
            },
            (0x8, _, _, 0x5) => {
                trace!("[MATH] Substract V{:x} from  V{:x}", y, x);
                self.pc += 2;
            },
            (0x8, _, _, 0x6) => {
                trace!("[BITOP] Shift V{:x} right by 1 and store result to V{:x}", y, x);
                self.pc += 2;
            },
            (0x8, _, _, 0x7) => {
                trace!("[MATH] Set V{:x} to V{:x} - V{:x}", x, y, x);
                self.pc += 2;
            },
            (0x8, _, _, 0xE) => {
                trace!("[BITOP] Shift V{:x} left by 1 and copy the result to V{:x}", y, x);
                self.pc += 2;
            },
            (0x9, _, _, 0x0) => trace!("[COND] Skip next instruction if V{:x} does not equal V{:x}", x, y),
            (0xA, _, _, _) => {
                trace!("[MEM] Set I to the address {:#x}", nnn);
                self.pc += 2;
            },
            (0xB, _, _, _) => {
                trace!("[FLOW] Jump to address {:#x} + V0", nnn);
                self.pc += 2;
            },
            (0xC, _, _, _) => {
                trace!("[RAND] Set V{:x} to the result: rand() AND {:#x}", x, nn);
                self.pc += 2;
            },
            (0xD, _, _, _) => {
                trace!("[DISPLAY] Draw a sprite at coordinate (V{:x}, V{:x}) of size {:#x} pixels", x, y, n);
                self.pc += 2;
            },
            (0xE, _, 0x9, 0xE) => {
                trace!("[KEYOP] Skip next instruction if key stored in V{:x} is present", x);
                self.pc += 2;
            },
            (0xE, _, 0xA, 0x1) => {
                trace!("[KEYOP] Skip next instruction if key stored in V{:x} isn't present", x);
                self.pc += 2;
            },
            (0xF, _, 0x0, 0x7) => {
                trace!("[TIMER] Set V{:x} to the value of the delay timer", x);
                self.pc += 2;
            },
            (0xF, _, 0x0, 0xA) => {
                trace!("[KEYOP] Await (blocking) key press and store result in V{:x}", x);
                self.pc += 2;
            },
            (0xF, _, 0x1, 0x5) => {
                trace!("[TIMER] Set the delay timer to V{:x}", x);
                self.pc += 2;
            },
            (0xF, _, 0x1, 0x8) => {
                trace!("[SOUND] Set the sound timer to V{:x}", x);
                self.pc += 2;
            },
            (0xF, _, 0x1, 0xE) => {
                trace!("[MEM] Add V{:x} to I", x);
                self.pc += 2;
            },
            (0xF, _, 0x2, 0x9) => {
                trace!("[MEM] Set I to the location of the sprite for the character in V{:x}", x);
                self.pc += 2;
            },
            (0xF, _, 0x3, 0x3) => {
                trace!("[BCD] Store BCD representation of V{:x} in memory starting at address I", x);
                self.pc += 2;
            },
            (0xf, _, 0x5, 0x5) => {
                trace!("[MEM] Store V0 to V{:x} in memory starting at address I", x);
                self.pc += 2;
            },
            (0xF, _, 0x6, 0x5) => {
                trace!("[MEM] Fill V0 to V{:x} with values from memory starting at address I", x);
                self.pc += 2;
            }
            _ => {
                panic!("Unrecognized instruction: {:#x}", opcode);
            }
        }
    }
}
