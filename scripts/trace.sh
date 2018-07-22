#!/bin/bash

RUST_LOG=info cargo run --bin chip8 -- games/tetris.c8
