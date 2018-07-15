#[macro_use] extern crate log;
extern crate env_logger;
extern crate sdl2;
use sdl2::rect::{Rect};
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use std::process;
use std::env;
use std::fs;
use std::io::prelude::*;

mod chip8;

fn main() {
    let rom_path = env::args().nth(1)
        .expect("Provide rom location!");

    env_logger::init();

    let code = fs::read(&rom_path)
        .expect("Could not read file");

    info!("Starting Chip8 emulation for ROM at: {:#}", rom_path);

    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&code);

    let window = match video_ctx
        .window("chip8 VM", 640, 480)
        .position_centered()
        .build() {
        Ok(window) => window,
        Err(err) => panic!("failed to create window: {}", err)
    };

    let mut canvas = match window
        .into_canvas()
        .present_vsync()
        .build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err)
    };

    let mut events = ctx.event_pump().unwrap();

    let mut main_loop = || {
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    process::exit(1);
                },
                _ => {}
            }
        }

        chip8.step();

        let _ = canvas.clear();
        let _ = canvas.present();
    };

    loop {
        main_loop();
    }

}
