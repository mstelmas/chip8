use super::super::interconnect;
use std::fmt;
use rand;
use rand::Rng;

pub struct Cpu {
    v: [u8; 16],
    i: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,

    // TODO: extract ?
    delay_timer: u8,
    await_key_press: bool
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PC: 0x{:x}, SP: 0x{:x}, DT: {:x}, V: {:?}", self.pc, self.sp, self.delay_timer, self.v)
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            i: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,

            delay_timer: 0,
            await_key_press: false
        }
    }
    
    pub fn execute_cycle(&mut self, interconnect: &mut interconnect::Interconnect) {
        trace!("{:?}", self);

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        let opcode = interconnect.read_word(self.pc);
        self.execute_opcode(opcode, interconnect);
    }

    fn execute_opcode(&mut self, opcode: u16, interconnect: &mut interconnect::Interconnect) {

        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let vx = self.v[x];
        let vy = self.v[y];

        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0x0) => {
                trace!("[DISPLAY] Clear screen");
                interconnect.display().clear();
                self.pc += 2;
            },
            (0x0, 0x0, 0xE, 0xE) => {
                trace!("[FLOW] Return from subroutine");
                self.sp -= 1;
                self.pc = self.stack[self.sp as usize];
                self.pc += 2;
            },
            (0x0, _, _, _) => {
                trace!("[CALL] Call RCA 1802 program at {:#x}", nnn);
                panic!("[CALL] Call RCA 1802 program at {:#x}", nnn);
            }
            (0x1, _, _, _) => {
                trace!("[FLOW] Jump to: {:#x}", nnn);
                self.pc = nnn;
            },
            (0x2, _, _, _) => {
                trace!("[FLOW] Call subroutine at {:#x}", nnn);
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            (0x3, _, _, _) => {
                trace!("[COND] Skip next instruction if V{:x} equals {:#x}", x, nn);
                if vx == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            (0x4, _, _, _) => {
                trace!("[COND] Skip next instruction if V{:x} does not equal {:#x}", x, nn);
                if vx != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            (0x5, _, _, 0x0) => {
                trace!("[COND] Skip next instruction if V{:x} equals V{:x}", x, y);
                if vx == vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => {
                trace!("[CONST] Set V{:x} to {:#x}", x, nn);
                self.v[x] = nn;
                self.pc += 2;
            },
            (0x7, _, _, _) => {
                trace!("[CONST] Add {:#x} to V{:x}", nn, x);
                let r = vx as u16 + nn as u16;
                self.v[x] = r as u8;
                self.pc += 2;
            },
            (0x8, _, _, 0x0) => {
                trace!("[ASSIGN] Set V{:x} to the value of V{:x}", x, y);
                self.v[x] = self.v[y];
                self.pc += 2;
            },
            (0x8, _, _, 0x1) => {
                trace!("[BITOP] Set V{:x} to V{:x} OR V{:x}", x, x, y);
                self.v[x] = self.v[x] | self.v[y];
                self.pc += 2;
            },
            (0x8, _, _, 0x2) => {
                trace!("[BITOP] Set V{:x} to V{:x} AND V{:x}", x, x, y);
                self.v[x] = self.v[x] & self.v[y];
                self.pc += 2;
            },
            (0x8, _, _, 0x3) => {
                trace!("[BITOP] Set V{:x} to V{:x} XOR V{:x}", x, x, y);
                self.v[x] = self.v[x] ^ self.v[y];
                self.pc += 2;
            },
            (0x8, _, _, 0x4) => {
                trace!("[MATH] Add V{:x} to V{:x}", x, y);
                let r = vx as u16 + vy as u16;
                self.v[x] = r as u8;
                self.v[0xF] = if r > 0xFF { 1 } else { 0 };
                self.pc += 2;
            },
            (0x8, _, _, 0x5) => {
                trace!("[MATH] Substract V{:x} from  V{:x}", y, x);
                self.v[0xF] = if vx > vy { 1 } else { 0 };
                self.v[x] = self.v[x].wrapping_sub(vy);
                self.pc += 2;
            },
            (0x8, _, _, 0x6) => {
                trace!("[BITOP] Shift V{:x} right by 1 and store result to V{:x}", y, x);
                self.v[0xF] = vy & 1;
                self.v[x] = vy >> 1;
                self.pc += 2;
            },
            (0x8, _, _, 0x7) => {
                trace!("[MATH] Set V{:x} to V{:x} - V{:x}", x, y, x);
                self.v[0xF] = if vx > vy { 0 } else { 1 };
                self.v[x] = vy.wrapping_sub(vx);
                self.pc += 2;
            },
            (0x8, _, _, 0xE) => {
                trace!("[BITOP] Shift V{:x} left by 1 and copy the result to V{:x}", y, x);
                self.v[0xF] = vy & 0x80;
                self.v[x] = vy << 1;
                self.pc += 2;
            },
            (0x9, _, _, 0x0) => {
                trace!("[COND] Skip next instruction if V{:x} does not equal V{:x}", x, y);
                if vx != vy {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            (0xA, _, _, _) => {
                trace!("[MEM] Set I to the address {:#x}", nnn);
                self.i = nnn;
                self.pc += 2;
            },
            (0xB, _, _, _) => {
                trace!("[FLOW] Jump to address {:#x} + V0", nnn);
                self.pc = (self.v[0] as u16) + nnn;
            },
            (0xC, _, _, _) => {
                trace!("[RAND] Set V{:x} to the result: rand() AND {:#x}", x, nn);
                self.v[x] = rand::thread_rng().gen::<u8>() & nn;
                self.pc += 2;
            },
            (0xD, _, _, _) => {
                trace!("[DISPLAY] Draw a sprite at coordinate (V{:x} (={:x}), V{:x} (={:x})) of size {:#x} pixels", x, vx, y, vy, n);

                self.v[0xF] = 0;

                for j in 0..n {
                    let pixel = interconnect.ram()[(self.i + j as u16) as usize];
                    for i in 0..8 {
                        if (pixel & (0x80 >> i)) != 0 {
                            // FIXME: index out of bounds: the len is 64 but the index is 255 (UFO)
                            if interconnect.display().vram()[(vy + j) as usize][(vx + i) as usize] == 1 {
                                self.v[0xF] = 1;
                            }
                            interconnect.display().vram()[(vy + j) as usize][(vx + i) as usize] ^= 1;
                        }
                    }
                }

                interconnect.display().draw();
                self.pc += 2;
            },
            (0xE, _, 0x9, 0xE) => {
                trace!("[KEYOP]); Skip next instruction if key; stored in V{:x} is present", x);
                if interconnect.keypad().is_key_pressed(x) == true {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            (0xE, _, 0xA, 0x1) => {
                trace!("[KEYOP] Skip next instruction if key stored in V{:x} isn't present", x);
                if interconnect.keypad().is_key_pressed(x) == false {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            (0xF, _, 0x0, 0x7) => {
                trace!("[TIMER] Set V{:x} to the value of the delay timer", x);
                self.v[x] = self.delay_timer;
                self.pc += 2;
            },
            (0xF, _, 0x0, 0xA) => {
                trace!("[KEYOP] Await (blocking) key press and store result in V{:x}", x);
                panic!("[KEYOP] Await (blocking) key press and store result in V{:x}", x);
                self.await_key_press = true;
                // TODO: FINISH IMPLEMENTATION
                self.pc += 2;
            },
            (0xF, _, 0x1, 0x5) => {
                trace!("[TIMER] Set the delay timer to V{:x}", x);
                self.delay_timer = vx;
                self.pc += 2;
            },
            (0xF, _, 0x1, 0x8) => {
                trace!("[SOUND] Set the sound timer to V{:x}", x);
                warn!("UNIMPLEMENTED - [SOUND] Set the sound timer to V{:x}", x);
                self.pc += 2;
            },
            (0xF, _, 0x1, 0xE) => {
                trace!("[MEM] Add V{:x} to I", x);
                self.i += vx as u16;
                self.pc += 2;
            },
            (0xF, _, 0x2, 0x9) => {
                trace!("[MEM] Set I to the location of the sprite for the character in V{:x}", x);
                self.i = (vx * 5) as u16;
                self.pc += 2;
            },
            (0xF, _, 0x3, 0x3) => {
                trace!("[BCD] Store BCD representation of V{:x} in memory starting at address I", x);
                let bcd_repr = [vx / 100, (vx % 100) / 10, vx % 10];
                interconnect.write_memory(self.i, &bcd_repr.to_vec());
                self.pc += 2;
            },
            (0xf, _, 0x5, 0x5) => {
                trace!("[MEM] Store V0 to V{:x} in memory starting at address I", x);
                interconnect.write_memory(self.i, &(self.v[0..x].to_vec()));
                self.pc += 2;
            },
            (0xF, _, 0x6, 0x5) => {
                trace!("[MEM] Fill V0 to V{:x} with values from memory starting at address I", x);
                for i in 0..x {
                    self.v[i] = interconnect.read_byte(self.i + i as u16);
                }
                self.pc += 2;
            }
            _ => {
                panic!("Unrecognized instruction: {:#x}", opcode);
            }
        }
    }
}
