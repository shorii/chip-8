use std::convert::TryFrom;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub const FONT_BASE: usize = 0;

pub const FONT_LENGTH: usize = 5;

const FONTS: [u8; 80] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0, // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0, // 3
    0x90, 0x90, 0xf0, 0x10, 0x10, // 4
    0xf0, 0x80, 0xf0, 0x10, 0x10, // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0, // 6
    0xf0, 0x10, 0x20, 0x40, 0x40, // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0, // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0, // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90, // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0, // B
    0xf0, 0x80, 0x80, 0x80, 0xf0, // C
    0xe0, 0x90, 0x90, 0x90, 0xe0, // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0, // E
    0xf0, 0x80, 0xf0, 0x80, 0x80, // F
];

// 0x000 - 0x1FF reserved by interpreter
// 0xEA0 - 0xEFF reserved for call stack (16 layer)
// 0xF00 - 0xFFF reserved for display refresh
pub struct Memory {
    pub all: [u8; 4096],
    pub stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Self {
        let mut all = [0; 4096];
        let mut fonts_copy = FONTS;
        fonts_copy.swap_with_slice(&mut all[0..80]);
        Memory {
            all: [0; 4096],
            stack: Vec::new(),
        }
    }

    pub fn load<P: AsRef<Path>>(&mut self, rom: P) {
        let mut fd = File::open(rom).unwrap();
        let mut rom_data = Vec::new();
        fd.read_to_end(&mut rom_data).unwrap();

        let start = 0x200;
        let end = start + rom_data.len();

        rom_data.swap_with_slice(&mut self.all[start..end]);
    }

    pub fn read(&self, program_counter: u16) -> [u8; 2] {
        let s = program_counter as usize;
        let e = s + 2;
        <[u8; 2]>::try_from(&self.all[s..e]).unwrap()
    }
}
