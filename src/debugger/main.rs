#[macro_use] extern crate conrod;
#[macro_use] extern crate bincode;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate nom;
extern crate serde;
extern crate serde_json;

mod disasm;
mod cli;
mod commands;
mod gui;

fn main() {
    gui::run();
}
