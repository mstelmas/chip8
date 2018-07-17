#!/bin/bash

RUST_LOG=trace cargo run --bin chip8 -- games/tetris.c8
