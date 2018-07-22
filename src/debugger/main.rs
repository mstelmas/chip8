use std::fs;

mod disasm;

fn main() {
    fs::read("games/tetris.c8")
        .map(|ref code| disasm::Disasm::disasm(code))
        .expect("Could not read file")
        .iter()
        .enumerate()
        .for_each(|(i, op)| println!("0x{:x}   {}", 0x200 + i, op.repr()));
}
