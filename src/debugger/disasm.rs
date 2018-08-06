pub enum Opcode {
    OP_00E0,
    OP_00EE,
    OP_0NNN(u16),
    OP_1NNN(u16),
    OP_2NNN(u16),
    OP_3XNN(u8, u8),
    OP_4XNN(u8, u8),
    OP_5XY0(u8, u8),
    OP_6XNN(u8, u8),
    OP_7XNN(u8, u8),
    OP_8XY0(u8, u8),
    OP_8XY1(u8, u8),
    OP_8XY2(u8, u8),
    OP_8XY3(u8, u8),
    OP_8XY4(u8, u8),
    OP_8XY5(u8, u8),
    OP_8XY6(u8, u8),
    OP_8XY7(u8, u8),
    OP_8XYE(u8, u8),
    OP_9XY0(u8, u8),
    OP_ANNN(u16),
    OP_BNNN(u16),
    OP_CXNN(u8, u8),
    OP_DXYN(u8, u8, u8),
    OP_EX9E(u8),
    OP_EXA1(u8),
    OP_FX07(u8),
    OP_FX0A(u8),
    OP_FX15(u8),
    OP_FX18(u8),
    OP_FX1E(u8),
    OP_FX29(u8),
    OP_FX33(u8),
    OP_FX55(u8),
    OP_FX65(u8),
    UNKNOWN
}

impl Opcode {
    pub fn repr(&self) -> String {
        match self {
            Opcode::OP_00E0 => String::from("CLS"),
            Opcode::OP_00EE => String::from("RET"),
            Opcode::OP_0NNN(nnn) => format!("SYS 0x{:x}", nnn),
            Opcode::OP_1NNN(nnn) => format!("JP 0x{:x}", nnn),
            Opcode::OP_2NNN(nnn) => format!("CALL 0x{:x}", nnn),
            Opcode::OP_3XNN(x, nn) => format!("SE V{}, 0x{:x}", x, nn),
            Opcode::OP_5XY0(x, y) => format!("SE V{}, V{}", x, y),
            Opcode::OP_4XNN(x, nn) => format!("SNE V{}, 0x{:x}", x, nn),
            Opcode::OP_6XNN(x, nn) => format!("LD V{}, 0x{:x}", x, nn),
            Opcode::OP_7XNN(x, nn) => format!("ADD V{}, 0x{:x}", x, nn),
            Opcode::OP_8XY0(x, y) => format!("LD V{}, V{}", x, y),
            Opcode::OP_8XY1(x, y) => format!("OR V{}, V{}", x, y),
            Opcode::OP_8XY2(x, y) => format!("AND V{}, V{}", x, y),
            Opcode::OP_8XY3(x, y) => format!("XOR V{}, V{}", x, y),
            Opcode::OP_8XY4(x, y) => format!("ADD V{}, V{}", x, y),
            Opcode::OP_8XY5(x, y) => format!("SUB V{}, V{}", x, y),
            Opcode::OP_8XY6(x, y) => format!("SHR V{} {{, V{}}}", x, y),
            Opcode::OP_8XY7(x, y) => format!("SUBN V{}, V{}", x, y),
            Opcode::OP_8XYE(x, y) => format!("SHL V{} {{, V{}}}", x, y),
            Opcode::OP_9XY0(x, y) => format!("SNE V{}, V{}", x, y),
            Opcode::OP_ANNN(nnn) => format!("LD I, 0x{:x}", nnn),
            Opcode::OP_BNNN(nnn) => format!("JP V0, 0x{:x}", nnn),
            Opcode::OP_CXNN(x, nn) => format!("RND V{}, 0x{:x}", x, nn),
            Opcode::OP_DXYN(x, y, n) => format!("DRW V{}, V{}, 0x{:x}", x, y, n),
            Opcode::OP_EX9E(x) => format!("SKP V{}", x),
            Opcode::OP_EXA1(x) => format!("SKNP V{}", x),
            Opcode::OP_FX07(x) => format!("LD V{}, DT", x),
            Opcode::OP_FX0A(x) => format!("LD V{}, K", x),
            Opcode::OP_FX15(x) => format!("LD DT, V{}", x),
            Opcode::OP_FX18(x) => format!("LD ST, V{}", x),
            Opcode::OP_FX1E(x) => format!("ADD I, V{}", x),
            Opcode::OP_FX29(x) => format!("LD F, V{}", x),
            Opcode::OP_FX33(x) => format!("LD B, V{}", x),
            Opcode::OP_FX55(x) => format!("LD [I], V{}", x),
            Opcode::OP_FX65(x) => format!("LD V{}, [I]", x),
            Opcode::UNKNOWN => String::from("???")
        }
    }
}

pub struct Disasm {}

impl Disasm {
    pub fn disasm(code: &Vec<u8>) -> Vec<(Opcode, u16)> {
        let mut opcodes = vec![];

        let mut i: usize = 0;

        while i < code.len() - 1 {
            let opcode = Self::read_word(code, i);
            opcodes.push((Self::opcode(opcode), opcode));
            i += 2;
        };

        opcodes
    }

    fn opcode(opcode: u16) -> Opcode {
        let op_1 = (opcode & 0xF000) >> 12;
        let op_2 = (opcode & 0x0F00) >> 8;
        let op_3 = (opcode & 0x00F0) >> 4;
        let op_4 = opcode & 0x000F;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;

        match (op_1, op_2, op_3, op_4) {
            (0x0, 0x0, 0xE, 0x0) => Opcode::OP_00E0,
            (0x0, 0x0, 0xE, 0xE) => Opcode::OP_00EE,
            (0x0, _, _, _) => Opcode::OP_0NNN(nnn),
            (0x1, _, _, _) => Opcode::OP_1NNN(nnn),
            (0x2, _, _, _) => Opcode::OP_2NNN(nnn),
            (0x3, _, _, _) => Opcode::OP_3XNN(x, nn),
            (0x4, _, _, _) => Opcode::OP_4XNN(x, nn),
            (0x5, _, _, 0x0) => Opcode::OP_5XY0(x, y),
            (0x6, _, _, _) => Opcode::OP_6XNN(x, nn),
            (0x7, _, _, _) => Opcode::OP_7XNN(x, nn),
            (0x8, _, _, 0x0) => Opcode::OP_8XY0(x, y),
            (0x8, _, _, 0x1) => Opcode::OP_8XY1(x, y),
            (0x8, _, _, 0x2) => Opcode::OP_8XY2(x, y),
            (0x8, _, _, 0x3) => Opcode::OP_8XY3(x, y),
            (0x8, _, _, 0x4) => Opcode::OP_8XY4(x, y),
            (0x8, _, _, 0x5) => Opcode::OP_8XY5(x, y),
            (0x8, _, _, 0x6) => Opcode::OP_8XY6(x, y),
            (0x8, _, _, 0x7) => Opcode::OP_8XY7(x, y),
            (0x8, _, _, 0xE) => Opcode::OP_8XYE(x, y),
            (0x9, _, _, 0x0) => Opcode::OP_9XY0(x, y),
            (0xA, _, _, _) => Opcode::OP_ANNN(nnn),
            (0xB, _, _, _) => Opcode::OP_BNNN(nnn),
            (0xC, _, _, _) => Opcode::OP_CXNN(x, nn),
            (0xD, _, _, _) => Opcode::OP_DXYN(x, y, n),
            (0xE, _, 0x9, 0xE) => Opcode::OP_EX9E(x),
            (0xE, _, 0xA, 0x1) => Opcode::OP_EXA1(x),
            (0xF, _, 0x0, 0x7) => Opcode::OP_FX07(x),
            (0xF, _, 0x0, 0xA) => Opcode::OP_FX0A(x),
            (0xF, _, 0x1, 0x5) => Opcode::OP_FX15(x),
            (0xF, _, 0x1, 0x8) => Opcode::OP_FX18(x),
            (0xF, _, 0x2, 0x9) => Opcode::OP_FX29(x),
            (0xF, _, 0x3, 0x3) => Opcode::OP_FX33(x),
            (0xf, _, 0x5, 0x5) => Opcode::OP_FX55(x),
            (0xF, _, 0x6, 0x5) => Opcode::OP_FX65(x),
            _ => Opcode::UNKNOWN
        }
    }

    fn read_word(code: &Vec<u8>, location: usize) -> u16 {
        (code[location] as u16) << 8 | (code[(location + 1)] as u16)
    }

}
