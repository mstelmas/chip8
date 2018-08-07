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
    let mut cli = cli::Cli::new();

    // TODO: for now, lets make debugger stop VM that it is attaching to
    cli.stop();
    gui::run(cli);
}
