use std::io::prelude::*;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

pub struct Memory(Vec<u8>);

const MEMORY_SIZE: usize = 0x1000;

const FONTS_BASE: usize = 0;
const FONT_SIZE: usize = 5; 
const FONT_COUNT: usize = 16;

pub const PROGRAM_CODE_BASE: usize = 0x200;

pub const STACK_BASE: usize = 0xefe;

pub const WORD_SIZE: usize = 2;

const FONTS: [u8; FONT_SIZE * FONT_COUNT] = [
    0xf0, 0x90, 0x90, 0x90, 0xf0,   // 0
    0x20, 0x60, 0x20, 0x20, 0x70,   // 1
    0xf0, 0x10, 0xf0, 0x80, 0xf0,   // 2
    0xf0, 0x10, 0xf0, 0x10, 0xf0,   // 3
    0x90, 0x90, 0xf0, 0x10, 0x10,   // 4
    0xf0, 0x80, 0xf0, 0x10, 0xf0,   // 5
    0xf0, 0x80, 0xf0, 0x90, 0xf0,   // 6
    0xf0, 0x10, 0x20, 0x40, 0x40,   // 7
    0xf0, 0x90, 0xf0, 0x90, 0xf0,   // 8
    0xf0, 0x90, 0xf0, 0x10, 0xf0,   // 9
    0xf0, 0x90, 0xf0, 0x90, 0x90,   // A
    0xe0, 0x90, 0xe0, 0x90, 0xe0,   // B
    0xf0, 0x80, 0x80, 0x80, 0xf0,   // C
    0xe0, 0x90, 0x90, 0x90, 0xe0,   // D
    0xf0, 0x80, 0xf0, 0x80, 0xf0,   // E
    0xf0, 0x80, 0xf0, 0x80, 0x80    // F
];

impl Memory {
    pub fn new(program_code: &[u8]) -> Memory {
        let mut memory = vec![0; MEMORY_SIZE];
        memory.splice(FONTS_BASE .. (FONTS_BASE + FONT_SIZE * FONT_COUNT), FONTS.iter().cloned());
        memory.splice(PROGRAM_CODE_BASE .. (PROGRAM_CODE_BASE + program_code.len()), program_code.iter().cloned());

        Memory(memory)
    }

    pub fn read_at(&self, buf: &mut [u8], offset: usize) {
        (&self.0[offset..]).read_exact(buf).unwrap()
    }

    pub fn write_at(&mut self, buf: &[u8], offset: usize) {
        (&mut self.0[offset..]).write_all(buf).unwrap()
    }

    pub fn read_u16_at(&self, offset: usize) -> u16 {
        (&self.0[offset..]).read_u16::<BigEndian>().unwrap()
    }

    pub fn write_u16_at(&mut self, value: u16, offset: usize){
        (&mut self.0[offset..]).write_u16::<BigEndian>(value).unwrap()
    }
}